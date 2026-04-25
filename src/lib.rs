use anyhow::{anyhow, Result};
use candle_core::{Device, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::bert::{BertModel, Config, DTYPE};
use hf_hub::{api::sync::ApiBuilder, Repo};
use serde::Deserialize;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use tokenizers::Tokenizer;
use walkdir::WalkDir;

#[derive(Debug, Deserialize)]
pub struct SkillMetadata {
    pub name: String,
    pub description: String,
}

pub struct Skill {
    pub name: String,
    pub description: String,
    pub content: String,
    #[allow(dead_code)]
    pub path: String,
}

pub struct Model {
    pub model: BertModel,
    pub tokenizer: Tokenizer,
    pub device: Device,
}

impl Model {
    pub fn load(cache_dir_opt: Option<PathBuf>) -> Result<Self> {
        let device = Device::Cpu;
        let mut builder = ApiBuilder::new();
        
        if let Some(cache_dir) = cache_dir_opt {
            builder = builder.with_cache_dir(cache_dir);
        }

        let api = builder.build()?;
        let repo = api.repo(Repo::model("sentence-transformers/all-MiniLM-L6-v2".to_string()));
        
        let config_filename = repo.get("config.json")?;
        let weights_filename = repo.get("model.safetensors")?;
        let tokenizer_filename = repo.get("tokenizer.json")?;

        let config = fs::read_to_string(config_filename)?;
        let config: Config = serde_json::from_str(&config)?;
        let tokenizer = Tokenizer::from_file(tokenizer_filename)
            .map_err(|e| anyhow!("Tokenizer error: {e}"))?;

        let vb = unsafe {
            VarBuilder::from_mmaped_safetensors(&[weights_filename], DTYPE, &device)?
        };
        let model = BertModel::load(vb, &config)?;

        Ok(Self {
            model,
            tokenizer,
            device,
        })
    }

    pub fn embed(&self, text: &str) -> Result<Tensor> {
        let tokens = self.tokenizer.encode(text, true)
            .map_err(|e| anyhow!("Tokenization error: {e}"))?;
        let token_ids = tokens.get_ids().to_vec();
        let token_ids = Tensor::new(token_ids.as_slice(), &self.device)?.unsqueeze(0)?;
        let token_type_ids = token_ids.zeros_like()?;
        
        let embeddings = self.model.forward(&token_ids, &token_type_ids, None)?;
        
        // Mean pooling
        let (_n_batch, n_tokens, _hidden_size) = embeddings.dims3()?;
        let embeddings = (embeddings.sum(1)? / (n_tokens as f64))?;
        
        // Normalize
        let norm = embeddings.sqr()?.sum_all()?.sqrt()?;
        let embeddings = embeddings.broadcast_div(&norm)?;
        
        Ok(embeddings)
    }
}

pub fn parse_skill(file_path: &Path) -> Option<Skill> {
    let content = fs::read_to_string(file_path).ok()?;

    if !content.starts_with("---") {
        return None;
    }

    let rest = &content[3..];
    let end = rest.find("---")?;
    let frontmatter = &rest[..end];

    let meta = serde_yaml::from_str::<SkillMetadata>(frontmatter).ok()?;
    Some(Skill {
        name: meta.name,
        description: meta.description,
        content: content.clone(),
        path: file_path.to_string_lossy().into_owned(),
    })
}

pub fn get_skills_in_dirs(dirs: &[PathBuf]) -> Vec<Skill> {
    let mut skills = Vec::new();
    for dir in dirs {
        if !dir.exists() {
            continue;
        }
        for entry in WalkDir::new(dir).max_depth(2).into_iter().filter_map(|e| e.ok()) {
            if entry.file_name() == "SKILL.md" {
                if let Some(skill) = parse_skill(entry.path()) {
                    skills.push(skill);
                }
            }
        }
    }
    skills
}

pub fn get_default_skills() -> Vec<Skill> {
    let mut dirs = Vec::new();
    
    if let Some(home) = dirs::home_dir() {
        dirs.push(home.join(".agents").join("passive_skills"));
    }
    dirs.push(PathBuf::from(".agents").join("passive_skills"));

    get_skills_in_dirs(&dirs)
}

pub fn dot_product(t1: &Tensor, t2: &Tensor) -> Result<f32> {
    let dot = (t1 * t2)?.sum_all()?.to_scalar::<f32>()?;
    Ok(dot)
}

pub fn get_keyword_overlap(query: &str, name: &str, description: &str) -> f32 {
    let query_words: HashSet<String> = query.to_lowercase()
        .split(|c: char| !c.is_alphanumeric())
        .filter(|s| s.len() > 2)
        .map(|s| s.to_string())
        .collect();
    
    if query_words.is_empty() {
        return 0.0;
    }

    let skill_text = format!("{} {}", name, description).to_lowercase();
    let mut matches = 0;
    for word in &query_words {
        if skill_text.contains(word) {
            matches += 1;
        }
    }

    (matches as f32) / (query_words.len() as f32)
}

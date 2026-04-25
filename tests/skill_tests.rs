use anyhow::Result;
use skill_finder::{get_skills_in_dirs, dot_product, get_keyword_overlap, Model, parse_skill};
use std::path::PathBuf;
use std::io::Write;
use tempfile::NamedTempFile;
use candle_core::{Device, Tensor};

fn get_test_skills_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests").join("skills")
}

#[test]
fn test_discovery_all_skills() {
    let test_dir = get_test_skills_dir();
    let skills = get_skills_in_dirs(&[test_dir]);
    
    let mut names: Vec<String> = skills.iter().map(|s| s.name.clone()).collect();
    names.sort();
    
    let expected = vec![
        "code-linter",
        "database-migration",
        "image-resizer",
        "pdf-processing",
        "web-scraper"
    ];
    
    assert_eq!(names, expected);
}

#[test]
fn test_search_pdf_processing() -> Result<()> {
    let model = Model::load(None)?;
    let test_dir = get_test_skills_dir();
    let skills = get_skills_in_dirs(&[test_dir]);
    
    let query = "I need to extract text from a pdf";
    let query_embedding = model.embed(query)?;
    
    let mut results = Vec::new();
    for skill in skills {
        let skill_text = format!("{}. {}", skill.name, skill.description);
        let skill_embedding = model.embed(&skill_text)?;
        let vector_sim = dot_product(&query_embedding, &skill_embedding)?;
        let keyword_boost = get_keyword_overlap(query, &skill.name, &skill.description);
        let score = vector_sim * 0.5 + keyword_boost * 0.5;
        results.push((skill.name, score));
    }
    
    results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    
    assert_eq!(results[0].0, "pdf-processing");
    assert!(results[0].1 > 0.6);
    Ok(())
}

#[test]
fn test_search_database_migration() -> Result<()> {
    let model = Model::load(None)?;
    let test_dir = get_test_skills_dir();
    let skills = get_skills_in_dirs(&[test_dir]);
    
    let query = "How to migrate SQL tables?";
    let query_embedding = model.embed(query)?;
    
    let mut results = Vec::new();
    for skill in skills {
        let skill_text = format!("{}. {}", skill.name, skill.description);
        let skill_embedding = model.embed(&skill_text)?;
        let vector_sim = dot_product(&query_embedding, &skill_embedding)?;
        let keyword_boost = get_keyword_overlap(query, &skill.name, &skill.description);
        let score = vector_sim * 0.5 + keyword_boost * 0.5;
        results.push((skill.name, score));
    }
    
    results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    
    assert_eq!(results[0].0, "database-migration");
    Ok(())
}

#[test]
fn test_search_web_scraper() -> Result<()> {
    let model = Model::load(None)?;
    let test_dir = get_test_skills_dir();
    let skills = get_skills_in_dirs(&[test_dir]);
    
    let query = "Scrape content from a website";
    let query_embedding = model.embed(query)?;
    
    let mut results = Vec::new();
    for skill in skills {
        let skill_text = format!("{}. {}", skill.name, skill.description);
        let skill_embedding = model.embed(&skill_text)?;
        let vector_sim = dot_product(&query_embedding, &skill_embedding)?;
        let keyword_boost = get_keyword_overlap(query, &skill.name, &skill.description);
        let score = vector_sim * 0.5 + keyword_boost * 0.5;
        results.push((skill.name, score));
    }
    
    results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    
    assert_eq!(results[0].0, "web-scraper");
    Ok(())
}

#[test]
fn test_search_image_resizer() -> Result<()> {
    let model = Model::load(None)?;
    let test_dir = get_test_skills_dir();
    let skills = get_skills_in_dirs(&[test_dir]);
    
    let query = "Make my images smaller";
    let query_embedding = model.embed(query)?;
    
    let mut results = Vec::new();
    for skill in skills {
        let skill_text = format!("{}. {}", skill.name, skill.description);
        let skill_embedding = model.embed(&skill_text)?;
        let vector_sim = dot_product(&query_embedding, &skill_embedding)?;
        let keyword_boost = get_keyword_overlap(query, &skill.name, &skill.description);
        let score = vector_sim * 0.5 + keyword_boost * 0.5;
        results.push((skill.name, score));
    }
    
    results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    
    assert_eq!(results[0].0, "image-resizer");
    Ok(())
}

#[test]
fn test_search_code_linter() -> Result<()> {
    let model = Model::load(None)?;
    let test_dir = get_test_skills_dir();
    let skills = get_skills_in_dirs(&[test_dir]);
    
    let query = "Fix code style and errors";
    let query_embedding = model.embed(query)?;
    
    let mut results = Vec::new();
    for skill in skills {
        let skill_text = format!("{}. {}", skill.name, skill.description);
        let skill_embedding = model.embed(&skill_text)?;
        let vector_sim = dot_product(&query_embedding, &skill_embedding)?;
        let keyword_boost = get_keyword_overlap(query, &skill.name, &skill.description);
        let score = vector_sim * 0.5 + keyword_boost * 0.5;
        results.push((skill.name, score));
    }
    
    results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    
    assert_eq!(results[0].0, "code-linter");
    Ok(())
}

// Logic tests from main.rs

#[test]
fn test_keyword_overlap() {
    let query = "i need a pdf document";
    let score = get_keyword_overlap(query, "pdf-processing", "Extracts text from PDF documents.");
    assert!(score > 0.4);

    let query2 = "i need a pdf document";
    let score2 = get_keyword_overlap(query2, "database-migration", "Migrates SQL tables.");
    assert_eq!(score2, 0.0);
}

#[test]
fn test_dot_product() -> Result<()> {
    let device = Device::Cpu;
    let t1 = Tensor::new(&[[1.0f32, 0.0, 0.0]], &device)?;
    let t2 = Tensor::new(&[[1.0f32, 0.0, 0.0]], &device)?;
    let t3 = Tensor::new(&[[0.0f32, 1.0, 0.0]], &device)?;

    let sim1 = dot_product(&t1, &t2)?;
    assert!((sim1 - 1.0).abs() < 1e-5);

    let sim2 = dot_product(&t1, &t3)?;
    assert!(sim2.abs() < 1e-5);
    Ok(())
}

#[test]
fn test_parse_skill() -> Result<()> {
    let mut file = NamedTempFile::new()?;
    writeln!(file, "---")?;
    writeln!(file, "name: test-skill")?;
    writeln!(file, "description: A simple test skill.")?;
    writeln!(file, "---")?;
    writeln!(file, "Content here")?;

    let skill = parse_skill(file.path()).expect("Should successfully parse skill");
    assert_eq!(skill.name, "test-skill");
    assert_eq!(skill.description, "A simple test skill.");
    assert!(skill.content.contains("Content here"));
    Ok(())
}

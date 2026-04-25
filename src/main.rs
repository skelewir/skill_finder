use anyhow::Result;
use clap::{Parser, Subcommand};
use skill_finder::{get_default_skills, dot_product, get_keyword_overlap, Model};
use std::path::Path;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Search for relevant skills
    Search {
        /// Search prompt/query
        query: String,
    },
    /// Activate specific skills
    Activate {
        /// Comma-separated skill names to activate
        skills: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Search { query } => {
            let cache_dir = std::env::current_exe()?
                .parent()
                .unwrap_or(Path::new("."))
                .join(".model_cache");
            let model = Model::load(Some(cache_dir))?;
            let query_embedding = model.embed(&query)?;
            let skills = get_default_skills();

            if skills.is_empty() {
                println!("No skills found in search paths.");
                return Ok(());
            }

            let mut results = Vec::new();
            for skill in skills {
                let skill_text = format!("{}. {}", skill.name, skill.description);
                let skill_embedding = model.embed(&skill_text)?;
                let vector_sim = dot_product(&query_embedding, &skill_embedding)?;
                let keyword_boost = get_keyword_overlap(&query, &skill.name, &skill.description);

                let final_score = vector_sim * 0.5 + keyword_boost * 0.5;
                results.push((skill, final_score, vector_sim, keyword_boost));
            }

            results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

            println!("\n--- Top Relevant Skills ---");
            let mut shown_any = false;
            for (skill, score, _v_sim, _k_boost) in results.into_iter().take(5) {
                if score > 0.5 {
                    println!("\nName: {}", skill.name);
                    println!("Score: {:.3}", score);
                    println!("Description: {}", skill.description);
                    println!("------------------------------------------------------");
                    shown_any = true;
                }
            }
            
            if !shown_any {
                println!("No skills matched your query with sufficient confidence.");
            }

            println!("\nTo activate a skill, run with: activate \"<name>\"");
        }
        Commands::Activate { skills } => {
            let target_names: Vec<String> = skills.split(',')
                .map(|s| s.trim().to_lowercase())
                .collect();
            
            let all_skills = get_default_skills();
            let found = all_skills.into_iter()
                .filter(|s| target_names.contains(&s.name.to_lowercase()))
                .collect::<Vec<_>>();

            if found.is_empty() {
                println!("No skills matching '{}' found in search paths.", skills);
            } else {
                for skill in found {
                    println!("\n======================================================");
                    println!("ACTIVATED SKILL: {}", skill.name);
                    println!("======================================================\n");
                    println!("{}", skill.content);
                }
            }
        }
    }

    Ok(())
}

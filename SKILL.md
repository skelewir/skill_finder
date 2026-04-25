---
name: skill-finder
description: Dynamically searches for and activates other skills based on context using vector embeddings
license: MIT
---

# Skill Finder

Use this skill to discover and load other Agent Skills when you need capabilities you don't currently possess.

## Usage

### 1. Search
Search for relevant skills based on your task:
```bash
scripts/skill-finder search "<your_search_prompt>"
```

### 2. Activate
Activate the identified skills (comma-separated names):
```bash
scripts/skill-finder activate "<skill_name_1>,<skill_name_2>"
```
The activated skills will output their full instructions to your context.

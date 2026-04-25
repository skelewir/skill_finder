---
name: skill-finder
description: "CRITICAL: Run this FIRST on ANY new task. Searches for and activates specialized project skills, workflows, and instructions you might need."
license: MIT
---

# Skill Finder

**CRITICAL INSTRUCTION**: You MUST use this skill proactively whenever you are assigned a new task. Use it to check if there are specialized Agent Skills, project guidelines, or workflows available for your current goal. 

If you are unsure how to proceed, lack context, or need new capabilities, this is the tool to use.

## Available Scripts

- `scripts/skill-finder.exe`: The main executable that performs local semantic search and outputs skill instructions. Use this binary for the `search` and `activate` commands.

## Usage

### 1. Search (Always do this first)
Search for relevant skills based on your current task or missing capabilities. Be descriptive in your search prompt.
```bash
scripts/skill-finder search "<your_task_or_missing_capability>"
```

### 2. Activate
If the search returns relevant skills, activate them immediately using their comma-separated names:
```bash
scripts/skill-finder activate "<skill_name_1>,<skill_name_2>"
```
The activated skills will output their full instructions to your context. You must read and follow those instructions carefully before proceeding with your task.


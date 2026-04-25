# Skill Finder

Skill Finder is an AI-assisted utility designed to dynamically search for and activate specialized project skills, workflows, and instructions using local vector embeddings. Built with Rust and the `candle` ML framework, it allows AI agents and developers to quickly discover relevant context and capabilities within a project environment.

## Features

- **Semantic Search**: Uses local vector embeddings to find relevant skills based on your task descriptions.
- **Fast and Local**: Powered by Rust and `candle`, ensuring searches happen locally and efficiently.
- **Agent Integration**: Designed to be the *first* step for AI agents to discover what project-specific workflows or guidelines exist.

## Prerequisites

- [Rust & Cargo](https://rustup.rs/) (edition 2024)

## Building

To build the project using Cargo, run:

```bash
cargo build --release
```

## Usage

This tool can be used directly or via its wrapper scripts. It provides two main commands: `search` and `activate`.

### 1. Search for Skills

To find relevant capabilities based on a natural language description of your task:

```bash
scripts/skill-finder search "<your_task_or_missing_capability>"
```

### 2. Activate Skills

Once you identify the required skills from the search results, activate them to output their full instructions:

```bash
scripts/skill-finder activate "<skill_name_1>,<skill_name_2>"
```

## AI Agent Instructions

**CRITICAL INSTRUCTION**: AI agents MUST use this skill proactively whenever assigned a new task. If an agent is unsure how to proceed, lacks context, or needs new capabilities, this tool acts as the primary gateway to discover project-specific rules and workflows.

## License

This project is licensed under the [MIT License](LICENSE).

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

This tool is designed to be integrated as a specialized AI Agent Skill. It is not intended for typical manual CLI usage, but rather for an agent to discover and invoke when it needs to find project-specific context.

### 1. Installation as a Skill
To make this available to your AI agent, place this repository in your agent's skills directory (e.g., `~/.agents/skills/skill_finder` or the equivalent path for your agent).

**Default Search Paths:**
When searching, the tool scans for `SKILL.md` files (up to a depth of 2) in the following default directories:
1. `~/.agents/passive_skills` (Global environment)
2. `.agents/passive_skills` (Local project directory)

Ensure that the skills you want the agent to discover are placed in one of these locations.

### 2. How the Agent Uses It

When the agent evaluates its available skills, it will read the `SKILL.md` file. The instructions in `SKILL.md` explicitly mandate that the agent must use this tool proactively for any new task.

The agent will then use the provided binaries or scripts internally to:
1. **Search**: Run a semantic query to find required skills.
2. **Activate**: Load the instructions of the found skills into its context.

## AI Agent Instructions

**CRITICAL INSTRUCTION**: AI agents MUST use this skill proactively whenever assigned a new task. If an agent is unsure how to proceed, lacks context, or needs new capabilities, this tool acts as the primary gateway to discover project-specific rules and workflows.

## License

This project is licensed under the [MIT License](LICENSE).

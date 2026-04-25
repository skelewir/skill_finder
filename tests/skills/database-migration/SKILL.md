---
name: database-migration
description: Handles Postgres and MySQL database migrations. Generates SQL schema changes, runs tests, and applies schema updates. Use when you need to modify database tables or structures.
license: Apache-2.0
---

# Database Migration

This skill manages schema changes for relational databases.

## Instructions
1. Generate a migration file using `scripts/generate.sh`.
2. Apply the migration using `scripts/apply.sh`.

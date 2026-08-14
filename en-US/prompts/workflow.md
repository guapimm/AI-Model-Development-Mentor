# Development Workflow Specifications

## 1. Project Initialization & Documentation System

### Lightweight Mode (Code Lines < 500)

Only `README.md` is required, including: project overview, tech stack, core table structures, API list, deployment steps, and a resource estimate mini-table.

### Standard Mode (Code Lines ≥ 500)

Create the following document structure upon project initialization:

```
📁 /docs/
├── architecture.md      # Tech stack selection rationale (with daily analogies), system architecture diagrams (Mermaid), directory structure
├── resource_estimate.md # Project resource estimate table (three tiers: memory / disk / configuration, scale-up thresholds)
├── dev_log.md           # Development log: timestamps, changes, test results, known issues & fixes
├── api_interface.md     # Frontend-backend API contracts (URLs, parameters, return values, exception scenarios)
└── SNAPSHOT.md          # Core snapshot (≤ 200 lines): tech stack versions, table list, API paths, business flow charts
```

### Phase-0 Mandatory Output: Project Resource Estimate Table

Must be output after requirements are confirmed and before coding; standard format:

| Configuration Tier | Peak Runtime Memory | Initial Disk Usage | Annual Disk Growth Estimate | Minimum CPU | Applicable Scenario |
| :---------------- | :------------------ | :----------------- | :-------------------------- | :---------- | :------------------ |
| Low               | XX MB               | XX MB              | XX MB                       | 1 core      | Solo development, low traffic |
| Recommended       | XX MB               | XX MB              | XX MB                       | 2 cores     | Daily use, access by under 100 users |
| High              | XX MB               | XX GB              | XX GB                       | 4 cores     | Concurrent access, production environment |

- Scale-up trigger conditions: specify the user/data volume thresholds at which a configuration upgrade is required
- Token consumption estimate: estimated Token consumption range across the full project lifecycle

Token Optimization: Generate a 【Context Summary】 after each conversation (progress, variable names, pending tasks, resume token); keep each summary within 100 characters.

## 2. Database Design Mandatory Spec

- Output an index design plan alongside table structure design; core query fields must be indexed
- Estimate per-table data volume; provide sharding/optimization plans in advance when it exceeds 100,000 rows
- Set field lengths and types as needed to avoid excessive storage consumption
- Mandatorily configure a database connection pool upper limit to prevent service crashes when connection counts are exhausted

## 3. Front-End Visual Positioning Protocol

Output the following positioning information before writing front-end code:

### 1. Page Layout Diagram

Define page structure via ASCII wireframes or Mermaid component trees.

### 2. UI Element Mapping Table

| Visual Position          | Component Name | Corresponding File Path    | CSS Class/ID | Function Description          |
| :----------------------- | :------------- | :------------------------- | :----------- | :---------------------------- |
| Top Right Navigation Bar | UserAvatar     | /src/components/Header.tsx | .user-avatar | User avatar and dropdown menu |

### 3. Front-End Event Mapping Table

| Name         | Operation | Invoked Backend API | Expected Outcome                  |
| :----------- | :-------- | :------------------ | :-------------------------------- |
| Login Button | Click     | POST /api/login     | Redirect to homepage, store Token |

## 4. Deployment & Disaster Recovery Mechanisms

### Local Backup

- Provide a one-click backup script `backup.sh` to export code, configurations and database to `./local_backup/`
- Check for existing local backups before every deployment; block deployment if no backup exists

### Gray-Scale Rollback for Cloud Servers

- Compress old code versions into `backup_[timestamp].zip` before deploying new releases
- 3-Step Emergency Rollback Process:
  1. `./rollback.sh latest` — Extract the latest backup archive
  2. `docker-compose restart` (or `pm2 restart all`)
  3. `./health_check.sh` — Output service running status
- Record backup timestamps, paths and rollback operations in `dev_log.md`

### Environment Isolation

- Separate configurations for development and production environments
- Highlight security configuration items requiring modification for production use in advance

## 5. Requirement Expansion & Suggestions

After completing user-specified features, output a Feature Enhancement Suggestion Card:

- ✅ Completed Feature Summary — Clear breakdown of available functionality
- 🔮 Potential Risk Warnings — Concurrent access, data consistency, third-party dependency risks, etc.
- 🚀 Recommended Extended Features — Mark priority P0/P1/P2, implementation difficulty star rating, expected benefits
- ⚡ Performance Optimization Suggestions — Mark priority, e.g., P0: add indexes to core fields, P1: add cache to hot data
- ⚠️ Beginner Pitfall Guide — Common misunderstandings and operational precautions

## 6. Closed-Loop Testing & Self-Inspection

### Minimal Verifiable Test Cases

Provide manual validation steps for users, example:

> "Click the login button, input valid username and password, verify successful redirect to homepage"

### Logical Consistency Declaration

Mandatorily attach this statement after outputting code:

> "Self-inspection completed: ① Correct variable scopes ② Full asynchronous handling ③ Complete exception capture ④ No sensitive data leakage ⑤ No obvious performance bottlenecks ⑥ Memory usage under control"

## 7. Version Anchoring

Output standardized Git Commit messages upon completing each milestone:

```
feat: user login module completed
Implement JWT Token authentication
Add password hash storage
Frontend login form validation
Author: AI Assistant
Date: 2026-08-08
```

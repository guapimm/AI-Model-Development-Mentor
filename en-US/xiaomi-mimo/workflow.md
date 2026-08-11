# Development Workflow Specifications

## 1. Project Initialization & Documentation System

### Lightweight Mode (Code Lines < 500)

Only `README.md` is required, including: project overview, tech stack, core table structures, API list, deployment steps.

### Standard Mode (Code Lines ≥ 500)

Create the following document structure upon project initialization:

```
📁 /docs/
├── architecture.md      # Tech stack selection rationale (with daily analogies), system architecture diagrams (Mermaid), directory structure
├── dev_log.md           # Development log: timestamps, changes, test results, known issues & fixes
├── api_interface.md     # Frontend-backend API contracts (URLs, parameters, return values, exception scenarios)
└── SNAPSHOT.md          # Core snapshot (≤ 200 lines): tech stack versions, table list, API paths, business flow charts
```

Token Optimization: Generate a Context Summary after each conversation (progress, variable names, pending tasks, resume token).

## 2. Front-End Visual Positioning Protocol

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

## 3. Deployment & Disaster Recovery Mechanisms

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

## 4. Requirement Expansion & Suggestions

After completing user-specified features, output a Feature Enhancement Suggestion Card:

- ✅ Completed Feature Summary — Clear breakdown of available functionality
- 🔮 Potential Risk Warnings — Concurrent access, data consistency, third-party dependency risks, etc.
- 🚀 Recommended Extended Features — Mark priority P0/P1/P2, implementation difficulty star rating, expected benefits
- ⚠️ Beginner Pitfall Guide — Common misunderstandings and operational precautions

## 5. Closed-Loop Testing & Self-Inspection

### Minimal Verifiable Test Cases

Provide manual validation steps for users, example:

> "Click the login button, input valid username and password, verify successful redirect to homepage"

### Logical Consistency Declaration

Mandatorily attach this statement after outputting code:

> "Self-inspection completed: ① Correct variable scopes ② Full asynchronous handling ③ Complete exception capture ④ No sensitive data leakage ⑤ No obvious performance bottlenecks"

## 6. Version Anchoring

Output standardized Git Commit messages upon completing each milestone:

```
feat: user login module completed
- Implement JWT Token authentication
- Add password hash storage
- Frontend login form validation
Author: AI Assistant
Date: 2026-08-08
```
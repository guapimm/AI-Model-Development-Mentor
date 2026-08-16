# Project Resource Estimate (required at Phase 0)

> Filled in at project start, guided by the mentor AI, as the basis for tech-stack selection and deployment planning.
> After filling in, archive this table into `docs/architecture.md` and keep it updated in later phases.

## 1. Basic Project Info

| Item | Value |
|------|-------|
| Project name | |
| Estimated total lines of code | (< 500 lines enables "lightweight mode", keeping only a single README.md) |
| Target user scale | Personal / small team / public product |
| Peak concurrent users | |
| Data type | Plain text / images / audio-video / large files |

## 2. Three-tier Resource Estimate

| Dimension | Minimum (dev/demo) | Recommended (small-scale launch) | High-availability (public product) |
|-----------|--------------------|----------------------------------|-------------------------------------|
| Memory | | | |
| Disk | | | |
| CPU cores | | | |
| Bandwidth | | | |
| Database | SQLite / in-memory | MySQL / PostgreSQL | Cluster + read-write splitting |

## 3. Third-party Service Dependencies

| Service | Purpose | Required? | Free tier sufficient? |
|---------|---------|-----------|-----------------------|
| Cloud server | | | |
| Object storage (files/images) | | | |
| SMS / email | | | |
| Payment | | | |
| Other | | | |

## 4. Performance & Resource Plan

- [ ] List query endpoints paginate by default; no full-table scans
- [ ] Database design includes an index plan
- [ ] Large-file / large-data operations use streaming
- [ ] Large memory operations have an explicit release mechanism
- [ ] External requests set timeout and retry policies

## 5. Monthly Cost Estimate

| Item | Minimum | Recommended |
|------|---------|-------------|
| Server | | |
| Storage | | |
| Third-party services | | |
| **Total** | | |

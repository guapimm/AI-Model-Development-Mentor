# Frontend UI Mapping Table + Event Mapping Table (required before writing frontend code)

> Output by the mentor AI before writing frontend code, so that zero-foundation users can report problems precisely.
> Archive into `docs/` and use together with the API contract (`docs/api_interface.md`).

## 1. Page Wireframe (ASCII or Mermaid)

```
┌──────────────────────────────────────────┐
│  Top navbar (logo / menu / user avatar)  │
├───────────────┬──────────────────────────┤
│               │                          │
│   Sidebar     │     Main content         │
│               │                          │
└───────────────┴──────────────────────────┘
```

## 2. UI Element Mapping Table

| Visual location | Component | File path | CSS class/ID | Description |
|-----------------|-----------|-----------|--------------|-------------|
| Top navbar, right | UserAvatar | src/components/Header.tsx | .user-avatar | User avatar and dropdown menu (logout, profile) |
| | | | | |

## 3. Frontend Event Mapping Table

| Chinese name | Action (click/swipe/input) | Backend endpoint called | Expected result |
|--------------|---------------------------|-------------------------|-----------------|
| Login button | Click | POST /api/login | Redirect to home after validation, show error on failure |
| | | | | |

## 4. Usage Guide (for zero-foundation users)

1. To report a page issue, just say "**location** + **what happened**", e.g.:
   > "The avatar at the top-right of the navbar does not respond to clicks"
2. The mentor AI will locate the exact component file and endpoint using the two tables above — no code description needed.

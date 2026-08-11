# Detailed Security Specification Manual

## 1. Secret Key & Configuration Management

- Hard-coding any secret keys, passwords, or API Tokens in source code is prohibited
- Use environment variables uniformly; only reference variable names in code
- Extract all configuration items into `.env.example` (contains only variable names, no real credentials)
- Add production `.env` file to `.gitignore`

## 2. User Input Validation

- All user inputs must pass type validation (e.g., reject string values for numeric fields)
- Enforce reasonable length limits (e.g., username: 2–50 characters)
- Block injection of special characters (SQL keywords, HTML tags, etc.)
- Restrict file upload types & sizes, validate MIME types

## 3. Database Security

- Mandatorily use parameterized queries or precompiled ORM statements
- String concatenation for SQL queries is forbidden
- Sensitive fields (passwords) must be stored as hashes (bcrypt/argon2)
- Store database connection string passwords via environment variables

## 4. Front-End XSS Protection

- All dynamically rendered content must undergo HTML escaping
- Leverage built-in escaping mechanisms of frameworks (e.g., React `{}`, Vue `{{}}`)
- Rendering user input directly via `innerHTML` or `v-html` is prohibited
- Set `HttpOnly` and `Secure` flags for Cookies

## 5. File System Security

- Validate all file path operations to prevent directory traversal (`../`)
- Restrict accessible directories with allowlists
- Rename uploaded files to random UUIDs, discard original filenames

## 6. External Request Security

- Set timeouts for all HTTP requests (5–10 seconds recommended)
- Implement retry logic (max 3 attempts with exponential backoff)
- Verify SSL certificates; skipping certificate validation is forbidden

## 7. Exception Handling

- Wrap all exceptions with try-catch blocks
- Do not expose raw stack traces to clients in production environments
- Log error details (timestamp, request ID, error type)
- Record audit logs for sensitive operations (login failures, insufficient permissions)

## 8. Performance & Security

- Account for interface performance bottlenecks; add cache (Redis) when necessary
- Optimize slow queries and create database indexes
- Process large file uploads via chunked or streaming uploads
- Prevent resource exhaustion attacks (enforce request rate limiting)
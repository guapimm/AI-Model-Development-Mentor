# Build context: mcp/ after `node scripts/sync.mjs && npm run build`
FROM node:22-alpine
WORKDIR /app
COPY package.json package-lock.json ./
RUN npm ci --omit=dev
COPY dist ./dist
COPY policy ./policy
COPY prompts ./prompts
ENTRYPOINT ["node", "dist/index.js"]

# lol_arena_stats

Contract-first scaffold for moving the backend to Rust with code generation at the center.

## Architecture

- `backend/`: Axum service that serves the API.
- `openapi/arena-api.yaml`: OpenAPI contract for the HTTP surface.
- `schemas/`: JSON Schema documents that drive Rust type generation and OpenAPI response models.
- `generated/ts-client/`: generated TypeScript client output from the OpenAPI contract.

## Why this layout

- The Rust backend stays focused on handlers and orchestration.
- Request and response shapes come from shared schema documents instead of being hand-written twice.
- The TypeScript client is generated from the OpenAPI contract so a future Next.js frontend can consume the same API contract directly.

## Commands

```bash
npm install
npm run generate
npm run build
npm test
```

Run the backend locally with:

```bash
cargo run -p backend
```
# lol_arena_stats

Contract-first scaffold for moving the backend to Rust with code generation at the center.

## Architecture

- `backend/`: hand-written implementation crate that serves the API by implementing generated server traits.
- `generated/rust-server/`: generated Axum server interfaces, models, and router from the OpenAPI contract.
- `openapi/arena-api.yaml`: OpenAPI contract for the HTTP surface.
- `schemas/`: JSON Schema documents that drive Rust type generation and OpenAPI response models.
- `generated/ts-client/`: generated TypeScript client output from the OpenAPI contract.
- `openapitools.json`: pins the OpenAPI generator version used for Rust stub generation.

## Why this layout

- The Rust backend stays focused on handlers and orchestration.
- Generated Axum server stubs own the HTTP contract and validation layer.
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

## Current API surface

- `GET /health`
- `GET /api/v1/arena/stats`
- `GET /api/v1/players/search`
- `GET /api/v1/players/{puuid}/profile`

## AI-friendly context

- Repo-specific Copilot guidance lives in `/tmp/workspace/mvb0005/lol_arena_stats/.github/copilot-instructions.md`.
- Follow the contract-first flow: update `openapi/` + `schemas/` first, then run `npm run generate`.
- Treat `generated/` artifacts as derived outputs from the contract and schemas.
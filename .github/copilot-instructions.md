# Copilot instructions for `lol_arena_stats`

## Project overview
- This repository is a **contract-first Rust backend scaffold** for League of Legends Arena stats.
- API contract source: `openapi/arena-api.yaml`.
- Schema source: `schemas/*.json`.
- Generated code:
  - Rust server stubs: `generated/rust-server/`
  - TypeScript client: `generated/ts-client/`
- Hand-written backend implementation: `backend/src/main.rs`.

## How to work in this repo
1. Update the API contract and/or JSON schemas first.
2. Regenerate code with `npm run generate`.
3. Implement backend behavior in `backend/src/main.rs` by implementing generated traits.
4. Validate with lint/build/test commands before finalizing.

## Commands
- Install dependencies: `npm install`
- Regenerate code: `npm run generate`
- Lint: `npm run lint`
- Build: `npm run build`
- Test: `npm test`
- Run backend locally: `cargo run -p backend`

## Guardrails for changes
- Do not hand-edit generated files unless the task explicitly requires it.
- Prefer changing contract/schema inputs and regenerating.
- Keep API operation names and schema references stable unless intentionally versioning changes.
- If an API response shape changes, update both OpenAPI and JSON schema source files.

## Entry points
- Health endpoint contract: `openapi/arena-api.yaml` (`/health`)
- Arena stats endpoint contract: `openapi/arena-api.yaml` (`/api/v1/arena/stats`)
- Runtime server wiring: `backend/src/main.rs`

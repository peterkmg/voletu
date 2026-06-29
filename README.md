# Voletu

Voletu is a full-stack logistics application for modeling operational flows across companies, bases, warehouses, storages, and ports. It focuses on structured document workflows and traceable inventory movement.

The application covers incoming and outgoing cargo, truck and rail transport, physical and ownership transfers, blending, reconciliation, cargo flow views, catalog management, synchronization status, users, and audit logs.

## Architecture

Voletu is organized as a monorepo:

- `packages/core` contains the shared Rust backend domain logic, API surface, persistence, validation, and tests.
- `packages/server` contains the standalone Rust server binary, including environment configuration loading and HTTP API startup.
- `packages/frontend` provides the React/TypeScript UI used by both the browser and desktop versions.
- `packages/desktop` wraps the frontend in a Tauri desktop shell and connects it to the shared Rust core.
- `packages/core-macros` contains proc macros used to reduce backend boilerplate.
- `deploy` contains Docker and Nginx deployment assets.

## Used Frameworks

- Backend: Rust, Axum, SeaORM, Utoipa
- Frontend: React, TypeScript, TanStack Router, TanStack Query, TanStack Table, Tailwind CSS
- Desktop: Tauri
- Tooling: Vite, Kubb, Vitest, Docker, Nginx

# Docker Deployment

Multi-node Docker setup for testing and deployment of the Voletu distributed sync system.

## Architecture

Each node runs a backend server (Rust/Axum) with its own SQLite database, paired with a frontend (React SPA served via nginx). Nodes communicate over a shared Docker network.

```
Browser                           Docker Network
┌──────────┐  ┌──────────┐      ┌─────────────────────────────┐
│ :8001    │  │ :8002    │      │ backend-central:3000        │
│ Central  │──│ Periph-A │──────│ backend-periph1:3000        │
│ Frontend │  │ Frontend │      │ backend-periph2:3000        │
└──────────┘  └──────────┘      └─────────────────────────────┘
```

## Prerequisites

- Docker Desktop (allocate at least 4 GB RAM for Rust compilation)
- `packages/frontend/src/generated/` must exist (run `pnpm api:generate` locally if missing)

## Profiles

Use Docker compose profiles to select which nodes to spin up:

| Profile | Services | Ports |
|---------|----------|-------|
| `central` | backend-central + frontend-central | 3001 (API), 8001 (UI) |
| `periph1` | backend-periph1 + frontend-periph1 | 3002 (API), 8002 (UI) |
| `periph2` | backend-periph2 + frontend-periph2 | 3003 (API), 8003 (UI) |

```bash
cd deploy

# Central only
docker compose --profile central up --build

# Central + one peripheral
docker compose --profile central --profile periph1 up --build

# Full 3-node cluster
docker compose --profile central --profile periph1 --profile periph2 up --build
```

First build takes ~15 minutes (Rust compilation). Subsequent starts are near-instant.

## Quick-Start (Pre-Built Artifacts)

For faster iteration, build locally first and use the quick compose:

```bash
# Build locally
cargo build -p voletu-server --release
pnpm --filter @voletu/frontend build

# Run with pre-built artifacts (no Docker build step)
cd deploy
docker compose -f docker-compose.quick.yml --profile central --profile periph1 up
```

## Node Initialization

All nodes start uninitialized with default credentials `admin` / `admin`.

### 1. Initialize Central

1. Open http://localhost:8001
2. Login with `admin` / `admin`
3. Redirected to initialization page
4. Set **Node Type**: `CENTRAL`
5. Set new admin credentials
6. Submit (server restarts briefly)
7. Login with new credentials

### 2. Initialize Peripherals

1. Open http://localhost:8002 (or :8003)
2. Login with `admin` / `admin`
3. Set **Node Type**: `PERIPHERAL`
4. Set **Central Server URL**: `http://backend-central:3000`
   - This is the Docker-internal hostname, not `localhost:3001`
5. Set new admin credentials
6. Submit and re-login

### 3. Test Sync

1. On Central, create catalog data (Companies, Products, Bases, Warehouses, Storages)
2. Wait ~5 seconds for the sync cycle
3. Check Peripheral UIs — data should appear
4. On a Peripheral, create a document and observe it propagate to Central

## Cleanup

```bash
# Stop containers (preserves databases)
docker compose --profile central --profile periph1 --profile periph2 down

# Stop and wipe all databases
docker compose --profile central --profile periph1 --profile periph2 down -v
```

## Troubleshooting

**First build is very slow**: Rust compiles SQLCipher + OpenSSL from vendored source. This is cached in Docker layers — subsequent builds are fast.

**Port already in use**: Change the host port mapping in `docker-compose.yml` (e.g., `3011:3000` instead of `3001:3000`).

**Sync not working**: Verify the Central Server URL uses the Docker hostname (`http://backend-central:3000`), not `localhost`. The sync worker runs server-side inside the Docker network.

**Frontend shows empty state**: Make sure you initialized the node first (login → `/init` page). The dashboard requires an initialized node.

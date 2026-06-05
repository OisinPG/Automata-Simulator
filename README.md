# Automata Sim

A high-performance cellular automata simulator built with Rust + WebAssembly + React.

---

## Project Structure

```
automata-sim/
├── sim-core/          ← Rust WASM library (simulation logic)
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs
|       └── automata.rs
└── frontend/          ← TypeScript + React + Vite (rendering & UI)
    ├── index.html
    ├── vite.config.ts
    ├── package.json
    └── src/
        └── components/
        |   └── SimCanvas.tsx
        └── main.tsx
```

---

## Prerequisites

Install these once before anything else.

| Tool | Purpose | Download |

| Rust + Cargo | Compiles the simulation core | https://rustup.rs |
| wasm-pack | Builds Rust → WASM + generates TS bindings | `cargo install wasm-pack` |
| Node.js (LTS) | Runs the frontend dev server | https://nodejs.org |

After installing Rust, add the WASM compilation target:

```powershell
rustup target add wasm32-unknown-unknown
```

Install wasm-pack:

```powershell
cargo install wasm-pack
```

Install frontend dependencies (run once inside the `frontend/` folder):

```powershell
cd frontend
npm install
```

---

## Building

### 1. Compile Rust → WASM

Run from inside `sim-core/`. This compiles your Rust code and writes the WASM
binary + auto-generated TypeScript bindings into `frontend/src/wasm/`.

**Development build** (faster compile, unoptimized):
```powershell
cd sim-core
wasm-pack build --target web --out-dir ../frontend/src/wasm --dev
```

**Release build** (slower compile, fully optimized — use before shipping):
```powershell
cd sim-core
wasm-pack build --target web --out-dir ../frontend/src/wasm
```

---

## Running

### 2. Start the frontend dev server

Run from inside `frontend/`. Vite will hot-reload TypeScript changes automatically.

```powershell
cd frontend
npm run dev
```

Then open your browser to: **http://localhost:5173**

---

## Common Errors

**`sh` not recognized** — You are on Windows. Do not use the Linux Rust install command.
Download the installer directly from https://rustup.rs instead.

**`wasm-pack` not found** — Close and reopen PowerShell after running `cargo install wasm-pack`
so the new PATH is picked up.

**`Cannot find module './wasm/sim_core.js'`** — You haven't run the `wasm-pack build` step yet,
or you ran it from the wrong directory. The WASM build must run before the frontend.

**Blank canvas / nothing rendering** — Open the browser console (F12). If you see a WASM
error, the build is stale — re-run `wasm-pack build`.

---

## Quick Reference

| Task | Command | Run from |

| Install WASM target | `rustup target add wasm32-unknown-unknown` | anywhere |
| Install wasm-pack | `cargo install wasm-pack` | anywhere |
| Install frontend deps | `npm install` | `frontend/` |
| Dev WASM build | `wasm-pack build --target web --out-dir ../frontend/src/wasm --dev` | `sim-core/` |
| Release WASM build | `wasm-pack build --target web --out-dir ../frontend/src/wasm` | `sim-core/` |
| Start dev server | `npm run dev` | `frontend/` |

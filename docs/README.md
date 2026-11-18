# Wishlez

Wishlez is a desktop personal-finance app for understanding where money goes
across accounts and over time. Accounts act as virtual piggy banks; tags and
journals support cash-flow analysis without requiring a full accounting workflow.

The app is under active development. Imports, automated tagging, reconciliation,
and reporting are planned capabilities, not a claim that they are already complete.

## Run locally

Install Node.js 22.12+ or a newer supported LTS release, pnpm, Rust via rustup,
and the [Tauri platform prerequisites](https://v2.tauri.app/start/prerequisites/).

Run these commands from the repository root:

```shell
pnpm install
pnpm tauri dev
```

Tauri starts the frontend development server automatically. To create a desktop
build and platform bundles:

```shell
pnpm tauri build
```

## Planned work

Browse the [feature backlog](backlog/README.md). Each feature owns its task list
and open decisions.

---
description: Implement a new feature
argument-hint: <feature-description>
---

Implement a new feature for FreshRig: $ARGUMENTS

1. Create a feature branch: `git checkout -b feat/<short-name>`
2. Plan the implementation — which files need to be created or modified.
3. Implement following existing patterns:
   - Rust commands in `src-tauri/src/commands/`
   - Models in `src-tauri/src/models/`
   - React components in `src/components/<feature>/`
   - Zustand stores in `src/stores/`
   - Types in `src/types/`
4. Use the app's design tokens (bg-bg-card, text-accent, etc.)
5. Register any new Tauri commands in `lib.rs`
6. Run `npx tsc --noEmit` and `cargo clippy --manifest-path src-tauri/Cargo.toml`
7. Commit with conventional commit message: `feat: <description>`
8. Provide a PR description summary.

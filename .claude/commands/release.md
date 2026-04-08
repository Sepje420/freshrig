---
description: Prepare and tag a new release
argument-hint: patch|minor|major
---

Prepare a release for FreshRig.

1. Determine the release type from $ARGUMENTS (patch, minor, or major). Default to patch if not specified.
2. Read the current version from `src-tauri/tauri.conf.json` (the `version` field) or `src-tauri/Cargo.toml`.
3. Calculate the new version based on semver rules.
4. Update the version in ALL of these locations:
   - `src-tauri/tauri.conf.json` → `version`
   - `src-tauri/Cargo.toml` → `[package] version`
   - `package.json` → `version`
   - `src/config/app.ts` → `APP_VERSION`
5. Run `cargo generate-lockfile --manifest-path src-tauri/Cargo.toml` to update Cargo.lock.
6. Run `npx tsc --noEmit` and `cargo clippy --manifest-path src-tauri/Cargo.toml` to verify.
7. Create a git commit: `git add -A && git commit -m "chore: release v{NEW_VERSION}"`
8. Tell me to run: `git tag v{NEW_VERSION} && git push origin main --tags`
9. Remind me that pushing the tag will trigger the GitHub Actions release workflow.

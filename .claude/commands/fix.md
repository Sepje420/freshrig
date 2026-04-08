---
description: Fix a GitHub issue
argument-hint: <issue-number>
---

Fix GitHub issue #$ARGUMENTS for FreshRig.

1. Read the issue: `gh issue view $ARGUMENTS`
2. Understand the bug or problem described.
3. Find the relevant code in the project.
4. Implement the fix following existing code patterns.
5. Run `npx tsc --noEmit` and `cargo clippy --manifest-path src-tauri/Cargo.toml` to verify.
6. Create a commit: `git commit -m "fix: <description> (closes #$ARGUMENTS)"`

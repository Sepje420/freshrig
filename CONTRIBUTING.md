# Contributing to FreshRig

Thank you for your interest in contributing to FreshRig! This guide will help you get started.

## Development Setup

### Prerequisites
- [Node.js](https://nodejs.org/) (LTS)
- [Rust](https://www.rust-lang.org/tools/install) (stable, MSVC toolchain)
- [Microsoft C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
- [Git](https://git-scm.com/)

### Getting Started
```bash
git clone https://github.com/ZIPREX420/freshrig.git
cd freshrig
npm install
npm run tauri dev
```

### Project Structure
- `src/` — React frontend (TypeScript, Tailwind CSS)
- `src-tauri/src/` — Rust backend (Tauri v2, WMI queries)
- `src/components/` — React components organized by feature
- `src/stores/` — Zustand state management
- `src/types/` — TypeScript type definitions

### Commands
- `npm run tauri dev` — Start development server
- `npm run tauri build` — Production build
- `npx tsc --noEmit` — Type-check TypeScript
- `cargo clippy --manifest-path src-tauri/Cargo.toml` — Lint Rust code
- `cargo fmt --manifest-path src-tauri/Cargo.toml` — Format Rust code

## Making Changes

1. Fork the repository
2. Create a feature branch: `git checkout -b feat/your-feature`
3. Make your changes
4. Run checks: `npx tsc --noEmit && cargo clippy --manifest-path src-tauri/Cargo.toml`
5. Commit using [Conventional Commits](https://www.conventionalcommits.org/): `git commit -m "feat: add new feature"`
6. Push and open a Pull Request

## Commit Convention

We use Conventional Commits:
- `feat:` — New feature
- `fix:` — Bug fix
- `docs:` — Documentation only
- `chore:` — Build process, dependencies
- `refactor:` — Code change that neither fixes a bug nor adds a feature
- `style:` — Code style (formatting, semicolons, etc.)
- `test:` — Adding or updating tests

## Code Style

- **TypeScript**: Strict mode, no `any` types, functional components only
- **Rust**: Follow `cargo fmt` and `cargo clippy` conventions
- **CSS**: Tailwind utility classes using design tokens from `src/styles.css`

## Reporting Bugs

Use the [Bug Report template](https://github.com/ZIPREX420/freshrig/issues/new?template=bug_report.yml) on GitHub Issues.

## Feature Requests

Use the [Feature Request template](https://github.com/ZIPREX420/freshrig/issues/new?template=feature_request.yml) on GitHub Issues.

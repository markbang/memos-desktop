# Repository Instructions

## GPUI Reference

- For GPUI architecture, lifecycle, platform integration, interaction, and updater work, inspect the current Zed source first. Treat Zed as the primary behavioral reference, then adapt the design to the pinned GPUI and GPUI Component versions in this repository.
- Record the relevant Zed module or pattern in project documentation when a change introduces a substantial GPUI subsystem.
- Reuse concepts and state models, not source text. Zed is GPL-licensed while this project is Apache-2.0; keep implementations original and compatible with this repository's dependency and licensing policy.
- Prefer Zed's separation of model, background task, platform boundary, and UI notification over embedding network or filesystem work in render code.

## Validation

- GPUI lifecycle changes require focused unit tests for pure logic, workspace tests, strict Clippy, and a release build.
- Platform installation changes require isolated installer tests and explicit behavior for Linux, macOS, Windows, package-manager installs, and source builds.

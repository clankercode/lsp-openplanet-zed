# Changelog

## [0.1.1] - 2026-10-22

### Fixed

- Carry complete license notices inside each packaged language directory. Zed's native registry packager excludes a loose `languages/LICENSES` file; the AngelScript and manifest payloads now each include both first-party license texts and their matching BSD-3-Clause or MIT notice.
- Validate license notices as complete, ordered blocks in their own language payload. Regression coverage rejects missing, truncated, reordered, swapped, and loose-only notices.
- Keep a successful downloaded server available on later startups when release lookup fails. Invalid or cross-platform cache paths remain rejected.
- Match the upstream TOML grammar export so the manifest grammar loads correctly.

### Included

- AngelScript syntax, outline, indentation, and `openplanet-lsp` integration for `.as` sources and exact `info.toml` manifests.
- Explicit binary configuration, PATH discovery, official release downloads for supported platforms, and workspace configuration passthrough.
- Real Linux Zed diagnostics and download/cache verification, a real editor screenshot, and documented platform limits.

### Distribution

- First tagged GitHub source release. The source tree previously identified itself as 0.1.0, without a GitHub release.
- Zed registry publication remains a separate human submission; this release does not claim marketplace availability.

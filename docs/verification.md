# Verification record

Verified on 2026-10-19, Linux x86-64. Rust `1.97.0`, Zed `1.24.2` (`fb69067729cb16a7f6135e2c1600c26a28c59659`), openplanet-lsp `0.8.0`.

## Automated checks

- `cargo fmt --all -- --check`: PASS.
- `cargo test --locked -j 2`: PASS, 13 tests (6 resolution/cache/argument tests, 5 AngelScript/package tests, 2 TOML tests).
- `cargo clippy --locked --all-targets -j 2 -- -D warnings`: PASS.
- `cargo build --locked --release --target wasm32-wasip2 -j 2`: PASS, real WASI preview-2 component.
- `cargo +1.90 build --locked --release --target wasm32-wasip2 -j 2`: PASS on the registry Rust toolchain.
- The exact registry `zed-extension` CLI at `9ee3c503a4bbbc6b4a0f8a789acca4871d773223` packaged the extension successfully. Its output lists languages, grammars and language servers and reports WASM API 0.7.0. Upstream manifest/license validators passed.
- The same `zed-extension` CLI, run on this tree, wrote `/tmp/zed-openplanet-real-package/archive.tar.gz` (SHA-256 `3a39d1bc78f9784a5ed66d7b49d41702c63cc46b91110bb15b3f27f3824c2a7c`). That archive was not edited after the CLI. Its members include `languages/angelscript/LICENSES` and `languages/openplanet_manifest/LICENSES` and do not include a loose `languages/LICENSES`. `scripts/check-package-notices.py` passes on it. `python3 -m unittest tests/test_package_notices.py` passes, including the case where a loose bundle alone fails.
- Both pinned C grammar parsers compiled using WASI SDK 34.0, the same command shape as Zed's extension builder.
- Official GitHub `v0.8.0` release contains all six asset filenames covered by tests; the Linux archive has `./openplanet-lsp` at its root.

`../lsp-openplanet` was not built, modified, or subjected to its heavy suite. All builds here used at most two Cargo jobs. Run `scripts/verify.sh` for the local checks.

Queries are compiled against the exact pinned upstream parser, not merely checked for balanced syntax. Test sources cover classes, namespaces, enums, preprocessor lines, handles, templates, const/non-const references, and `in`, `out`, `inout` variants. A failing test caught an invalid `enum_body` node and it was removed. A dedicated regression test failed for a renamed TOML grammar before the grammar key was corrected to the upstream `tree_sitter_toml` export. TOML and AngelScript native parser runtimes use separate test executables to avoid duplicate C symbols.

## Actual Zed session

`/usr/bin/zed` on this machine is the ZFS Event Daemon, **not the editor**. The official Zed Linux release was downloaded from `zed-industries/zed` and extracted to `/tmp/zed.app`; no system-wide editor install or OS update was performed.

The GUI ran on a private `Xvfb :187 -screen 0 1440x1000x24 -nolisten tcp` display with private data/config under `/tmp/zed-openplanet-session`. It used llvmpipe software rendering, a private D-Bus session, and no shared browser. In this Zed version `--user-data-dir` relocates settings to `<user-data-dir>/config/settings.json`; simply setting XDG_CONFIG_HOME was insufficient.

Zed's native dev-extension file picker did not appear in this headless desktop, even with a private GTK portal. Therefore the actual load used the same dev symlink layout as Zed's `ExtensionStore::install_dev_extension`: compiled `extension.wasm` and grammar WASM modules at the repository root, symlinked from the private `extensions/installed/openplanet` directory. Zed indexed this as **dev: true**, loaded the Rust component and grammars, and ran the server. The normal interactive file-picker installation flow remains untested. `scripts/build-dev.py --wasi-sdk /path/to/wasi-sdk --user-data-dir /tmp/a-private-zed-profile` reproduces this fallback without replacing an existing different extension.

Verified in the actual editor:

- PATH resolution selected `/home/xertrov/.local/bin/openplanet-lsp`, arguments `["--lsp"]`.
- Explicit `lsp.openplanet-lsp.binary` selected a transparent Python wire recorder which forwarded to the installed real server. Zed itself applies a configured binary override, so the native settings must include `arguments: ["--lsp"]`.
- Default initialization options were null/unset; explicit `{"unused": false}` reached `initialize` unchanged and suppressed the unused-variable diagnostic.
- `Main.as` opened as **AngelScript**, with real syntax colors and diagnostics `unknown type MissingType` and `undefined identifier missing_function` from `openplanet-lsp`.
- `info.toml` opened as **Openplanet Manifest** with TOML syntax colors. Appending a temporary invalid array produced a real `manifest` diagnostic; restoring the file cleared it.
- `ordinary.toml` stayed **Unknown** in the clean profile and Zed suggested installing its normal TOML extension. This proves the extension does not claim every TOML file. The fixture is deliberately retained for repeatability.
- With both `~/.local/bin` and `~/.cargo/bin` removed from this session's PATH and no binary override, Zed downloaded the official Linux x86-64 release through the extension API to its work directory. It launched that binary with `--lsp`, and the same two real errors appeared. The downloaded binary reports `openplanet-lsp 0.8.0`.

Independent read-only review found the offline-restart cache issue; the fix persists only a successful install path, validates it against the current platform, and falls back to it on release-service failure. Missing/corrupted cache entries retain the original download error. The startup test uses a fresh resolver call and a failed lookup. A subsequent actual Zed session with failing proxy settings also launched the cached binary.

The recorder only transported actual JSON-RPC messages; it did not synthesize responses. [Sanitized LSP evidence](lsp-evidence.json) contains the relevant initialize, didOpen, and publishDiagnostics data. [Startup evidence](zed-startup-evidence.txt) records selected binary paths and arguments. [Build hashes](build-sha256.txt) identify the component, grammar modules, and downloaded server used for the screenshot.

## Screenshot review

[zed-diagnostics.png](assets/zed-diagnostics.png) is an unmodified 1440×1000 X11 root-window capture of real Zed, made with ImageMagick `import -window root` on the isolated display. Its original minimal plugin is in `fixtures/plugin`; it is not copied from an undisclosed third-party source. The two intentionally invalid names demonstrate the server's diagnostics. No Openplanet type database is bundled.

Visual inspection of the full image and a zoomed text crop: readable type, visible source lines, both full diagnostic messages and server attribution, no popups covering the errors. Geometry inspection: diagnostic region occupies x=133..1058, y=180..327 within the 1440×1000 viewport; neither right-hand message is clipped by the project panel at x=1202. Screenshot content was compared against the actual JSON-RPC error messages. The empty editor area is the real diagnostics multibuffer layout, not a mockup.

## Explicit limits

- Linux x86-64 was the only actual GUI/download platform exercised. macOS, Windows, Linux ARM64, and their archives are covered by target/name tests, not runtime tests. Linux musl is not an official download target.
- Interactive dev-install file picker did not work in the headless environment; the native extension host load did work. No claim that Zed was installed at `/usr/bin/zed`.
- No Openplanet game instance was launched. The fixture intentionally contains errors; it demonstrates editor integration, not game compilation.
- Completion, hover, references, rename, multiple-plugin workspace routing, were not individually exercised in the GUI. Offline cached startup has a regression test that fails with the original download-only behavior and passes with persistent cache fallback; a fresh Zed session was additionally run with HTTP_PROXY, HTTPS_PROXY and ALL_PROXY pointed at the closed loopback port `127.0.0.1:9`. It still launched the cached server and showed both diagnostics. This verifies a failed-proxy startup, not a machine-wide network namespace isolation test. They use the upstream server and Zed LSP support.
- No claim of exhaustive AngelScript grammar coverage. The maintained grammar may recover imperfectly on Openplanet-specific syntax; semantic diagnostics belong to openplanet-lsp.
- GitHub registry review/acceptance is separate from successful local validation. See publishing status.

## Source release 0.1.1 — 2026-10-22

Revalidated the release candidate locally on Linux x86-64: `cargo fmt --all -- --check`, `cargo test --locked -j 2` (13 passing tests), `python3 -m unittest tests/test_package_notices.py` (6 passing tests), `cargo clippy --locked --all-targets -j 2 -- -D warnings`, and `cargo build --locked --release --target wasm32-wasip2 -j 2` all pass. The component is a real WASI build. Cargo, lockfile, and Zed manifest versions agree at 0.1.1.

This is a GitHub source release with the packaging corrections already verified above. The earlier temporary registry CLI, checkout, and packaged archive have been cleaned from `/tmp`; that historical native-package check was not rerun for this metadata-only release. The screenshot and real editor observations above were captured on 2026-10-19, not recaptured for this tag. Human registry submission must reconstruct and validate a package from the tagged source before publication.

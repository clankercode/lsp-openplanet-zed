# Publishing

Repository: https://github.com/clankercode/lsp-openplanet-zed

Zed registry ID: `openplanet`; initial version: `0.1.0`.

Status: local verification complete; registry submission is recorded below when created. This repository does not claim to be listed until the registry pull request merges and Zed publishes it.

## Release procedure

1. Run `scripts/verify.sh` and test the exact candidate commit in Zed; update `docs/verification.md` without widening its claims.
2. Commit and push `main`. The extension is source-built by Zed; do not check in generated `extension.wasm` or grammar WASM modules.
3. Fork https://github.com/zed-industries/extensions using the existing GitHub account. No new account or paid service is required.
4. Add the public HTTPS repository as submodule `extensions/openplanet` at the verified commit. Add to `extensions.toml`:

   ```toml
   [openplanet]
   submodule = "extensions/openplanet"
   version = "0.1.0"
   ```

5. Run `pnpm sort-extensions`, run registry checks, push a dedicated branch, and open one PR for this extension. Include the exact commit, features, verification limits, and screenshot URL. The registry permits at most three open submissions per author.
6. Respond to maintainer feedback within three weeks. Acceptance and publishing are maintainer-controlled.

The full Unlicense is present as both `UNLICENSE` and `LICENSE` so the registry's prefix-based license scanner recognizes the accepted Unlicense option. `LICENSE-CC0` preserves the alternative CC0 grant. Grammar notices are in `docs/` and are retained independently.

For updates, bump `Cargo.toml` and `extension.toml` together, regenerate Cargo.lock, validate, push, then submit a registry PR updating this extension's submodule commit and version.

Primary references checked 2026-10-19:

- https://zed.dev/docs/extensions/publishing/prerequisites
- https://zed.dev/docs/extensions/publishing/license-requirements
- https://zed.dev/docs/extensions/publishing/publishing-guide
- https://zed.dev/docs/extensions/developing-extensions

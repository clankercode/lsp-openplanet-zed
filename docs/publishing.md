# Publishing

Repository: https://github.com/clankercode/lsp-openplanet-zed

Zed registry ID: `openplanet`; initial version: `0.1.0`.

Status: local verification and the actual registry packaging CLI passed. Registry submission requires a human author under the venue's [AI policy](https://github.com/zed-industries/extensions/blob/main/AI_POLICY.md): **"Autonomous agents are not allowed to be used for contributing to this project."** It also requires the contributor to write the PR body in their own words. No upstream PR has been opened by this agent. A prepared local registry checkout and exact handoff are supplied to the coordinating human. This repository does not claim to be listed until an approved registry PR merges and Zed publishes it.

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

## Prepared human handoff

A local branch `add-openplanet` in `/tmp/zed-openplanet-registry` contains the sorted registry entry and HTTPS submodule. Its manifest, license, extension ID/version and submodule-location validators pass. The exact registry packaging CLI has also produced `/tmp/zed-openplanet-package/archive.tar.gz` and `manifest.json` successfully. No registry fork, upstream comment or PR was created by the agent.

To finish, a human who has reviewed and understands this extension can:

```sh
cd /tmp/zed-openplanet-registry
gh repo fork zed-industries/extensions --clone=false --remote=true
# If needed, add the fork URL as a separate remote first.
git push <your-fork-remote> add-openplanet
```

Then open a PR against `zed-industries/extensions:main`, writing its title/body in your own words. Explain that this adds AngelScript/Openplanet language support, link the public extension repository, identify the submodule commit, and describe the verified Linux GUI/download behavior and untested operating systems. Do not copy agent prose into maintainer communication; their policy requires human-authored explanations. If this temporary checkout is no longer present, use the complete release procedure above.

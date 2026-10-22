# Publishing

Repository: https://github.com/clankercode/lsp-openplanet-zed

Zed registry ID: `openplanet`; current version: `0.1.1`.

Status: local verification and the actual registry packaging CLI passed. Registry submission requires a human author under the venue's [AI policy](https://github.com/zed-industries/extensions/blob/main/AI_POLICY.md): **"Autonomous agents are not allowed to be used for contributing to this project."** It also requires the contributor to write the PR body in their own words. No upstream PR has been opened by this agent. The reconstruction steps below provide the human handoff; temporary verification files from the initial implementation are no longer available. This repository does not claim to be listed until an approved registry PR merges and Zed publishes it.

## Release procedure

1. Run `scripts/verify.sh` and test the exact candidate commit in Zed; update `docs/verification.md` without widening its claims.
2. Commit and push `main`. The extension is source-built by Zed; do not check in generated `extension.wasm` or grammar WASM modules.
3. Fork https://github.com/zed-industries/extensions using the existing GitHub account. No new account or paid service is required.
4. Add the public HTTPS repository as submodule `extensions/openplanet` at the verified commit. Add to `extensions.toml`:

   ```toml
   [openplanet]
   submodule = "extensions/openplanet"
   version = "0.1.1"
   ```

5. Run `pnpm sort-extensions`, run registry checks, push a dedicated branch, and open one PR for this extension. Include the exact commit, features, verification limits, and screenshot URL. The registry permits at most three open submissions per author.
6. Respond to maintainer feedback within three weeks. Acceptance and publishing are maintainer-controlled.

The full Unlicense is present as both `UNLICENSE` and `LICENSE` so the registry's prefix-based license scanner recognizes the accepted Unlicense option. `LICENSE-CC0` preserves the alternative CC0 grant. The complete first-party and upstream notices are retained in `docs/`. The registry packager copies each language directory and drops a loose `languages/LICENSES` file, so the shipped copies are `languages/angelscript/LICENSES` (Unlicense, CC0, BSD-3-Clause) and `languages/openplanet_manifest/LICENSES` (Unlicense, CC0, MIT). Run `python3 scripts/check-package-notices.py /path/to/archive.tar.gz` and `python3 -m unittest tests/test_package_notices.py` against every candidate. The real `zed-extension` CLI archive was checked at SHA-256 `3a39d1bc78f9784a5ed66d7b49d41702c63cc46b91110bb15b3f27f3824c2a7c`; it contains both in-language notice files and no loose `languages/LICENSES`.

For updates, bump `Cargo.toml` and `extension.toml` together, regenerate Cargo.lock, validate, push, then submit a registry PR updating this extension's submodule commit and version.

Primary references checked 2026-10-19:

- https://zed.dev/docs/extensions/publishing/prerequisites
- https://zed.dev/docs/extensions/publishing/license-requirements
- https://zed.dev/docs/extensions/publishing/publishing-guide
- https://zed.dev/docs/extensions/developing-extensions

## Prepared human handoff

The original temporary registry checkout and CLI archive from the 2026-10-19 verification are no longer present. Recreate the submission from the released `v0.1.1` source using the procedure above, pin its exact commit, and run the registry validators and real packager again before submitting. The recorded `6fd4e4a` pin is historical evidence, not the current release candidate. No registry fork, upstream comment, or PR was created by the agent.

To finish, a human who has reviewed and understands this extension can:

```sh
cd /path/to/your/zed-extensions-checkout
gh repo fork zed-industries/extensions --clone=false --remote=true
# If needed, add the fork URL as a separate remote first.
git push <your-fork-remote> add-openplanet
```

Then open a PR against `zed-industries/extensions:main`, writing its title/body in your own words. Explain that this adds AngelScript/Openplanet language support, link the public extension repository, identify the submodule commit, and describe the verified Linux GUI/download behavior and untested operating systems. Do not copy agent prose into maintainer communication; their policy requires human-authored explanations. If this temporary checkout is no longer present, use the complete release procedure above.

## GitHub source releases

GitHub tags and releases provide versioned source and release notes. They do not submit the extension to Zed's registry. For a patch release, update `Cargo.toml`, `Cargo.lock`, and `extension.toml` together, add the changelog entry, run the documented checks, then tag the reviewed commit. Create the GitHub release with notes from `CHANGELOG.md`; Zed builds its distributable from the registry submodule after acceptance. This repository does not publish a raw WASM file as a standalone installable extension.

#!/usr/bin/env python3
"""Build exactly the pinned grammars and Rust component for a local Zed profile."""
import argparse
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import tomllib

ROOT = Path(__file__).resolve().parents[1]

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--wasi-sdk", required=True, type=Path, help="Extracted WASI SDK directory, containing bin/clang")
    parser.add_argument("--user-data-dir", type=Path, help="Optional isolated Zed user-data directory for a dev symlink")
    args = parser.parse_args()
    clang = args.wasi_sdk.resolve() / "bin/clang"
    if not clang.is_file():
        parser.error(f"missing {clang}")
    subprocess.run(["cargo", "build", "--locked", "--release", "--target", "wasm32-wasip2", "--target-dir", str(ROOT / "target"), "-j", "2"], cwd=ROOT, check=True)
    shutil.copyfile(ROOT / "target/wasm32-wasip2/release/openplanet_zed.wasm", ROOT / "extension.wasm")
    manifest = tomllib.loads((ROOT / "extension.toml").read_text())
    (ROOT / "grammars").mkdir(exist_ok=True)
    with tempfile.TemporaryDirectory(prefix="openplanet-zed-grammars-") as temporary:
        for name, grammar in manifest["grammars"].items():
            repository = Path(temporary) / name
            subprocess.run(["git", "init", "-q", str(repository)], check=True)
            subprocess.run(["git", "-C", str(repository), "fetch", "-q", "--depth=1", grammar["repository"], grammar["commit"]], check=True)
            subprocess.run(["git", "-C", str(repository), "checkout", "-q", "FETCH_HEAD"], check=True)
            source = repository / "src"
            command = [str(clang), "-fPIC", "-shared", "-Os", f"-Wl,--export=tree_sitter_{name}", "-o", str(ROOT / "grammars" / f"{name}.wasm"), "-I", str(source), str(source / "parser.c")]
            if (source / "scanner.c").exists():
                command.append(str(source / "scanner.c"))
            subprocess.run(command, check=True)
    if args.user_data_dir:
        installed = args.user_data_dir.resolve() / "extensions/installed"
        installed.mkdir(parents=True, exist_ok=True)
        destination = installed / manifest["id"]
        if os.path.lexists(destination):
            if not destination.is_symlink() or destination.resolve() != ROOT:
                parser.error(f"refusing to replace existing extension: {destination}")
        else:
            destination.symlink_to(ROOT, target_is_directory=True)
    print("Built extension.wasm and grammar modules; reload extensions or restart Zed.")

if __name__ == "__main__":
    main()

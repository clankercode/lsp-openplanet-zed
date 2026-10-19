#!/usr/bin/env python3
"""Check that a packaged extension carries every required license notice."""

from __future__ import annotations

import argparse
import tarfile
from pathlib import Path


def archive_text(archive_path: Path) -> tuple[set[str], str]:
    with tarfile.open(archive_path, "r:gz") as archive:
        files = [
            member
            for member in archive.getmembers()
            if member.isfile() and archive.extractfile(member) is not None
        ]
        return {member.name for member in archive.getmembers()}, "\n".join(
            archive.extractfile(member).read().decode("utf-8", errors="ignore")
            for member in files
        )


def notice_is_present(package_text: str, notice: str) -> bool:
    return all(
        line in package_text or f"; {line}" in package_text
        for line in notice.splitlines()
        if line
    )


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("archive", type=Path)
    parser.add_argument("--repo", type=Path, default=Path(__file__).parent.parent)
    args = parser.parse_args()

    package_members, package_text = archive_text(args.archive)
    if "./languages/LICENSES" not in package_members and "languages/LICENSES" not in package_members:
        raise SystemExit("missing packaged license bundle: languages/LICENSES")
    notices = {
        "first-party Unlicense": args.repo / "UNLICENSE",
        "first-party CC0": args.repo / "LICENSE-CC0",
        "AngelScript BSD-3-Clause": args.repo / "docs/grammar-LICENSE-BSD-3-Clause",
        "TOML MIT": args.repo / "docs/toml-LICENSE-MIT",
    }
    missing = [
        name for name, path in notices.items() if not notice_is_present(package_text, path.read_text())
    ]
    if missing:
        raise SystemExit("missing packaged notices: " + ", ".join(missing))
    print("all required license notices are present in the package")


if __name__ == "__main__":
    main()

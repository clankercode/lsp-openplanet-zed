#!/usr/bin/env python3
"""Check that a real Zed registry archive carries every required license notice.

The registry packager copies each language directory (a directory that contains
config.toml) and drops a loose languages/LICENSES file. Notices therefore have
to live inside the shipped language directories, in their original order.
"""

from __future__ import annotations

import argparse
import tarfile
from pathlib import Path


def archive_members(archive_path: Path) -> dict[str, str]:
    texts: dict[str, str] = {}
    with tarfile.open(archive_path, "r:gz") as archive:
        for member in archive.getmembers():
            if not member.isfile():
                continue
            extracted = archive.extractfile(member)
            if extracted is None:
                continue
            name = member.name.removeprefix("./")
            texts[name] = extracted.read().decode("utf-8", errors="ignore")
    return texts


def notice_is_present(package_text: str, notice: str) -> bool:
    body = notice.strip()
    if not body:
        return False
    commented = [
        "\n".join(f"{prefix}{line}" if line else prefix.rstrip() for line in body.splitlines())
        for prefix in ("; ", "# ")
    ]
    return body in package_text or any(block in package_text for block in commented)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("archive", type=Path)
    parser.add_argument("--repo", type=Path, default=Path(__file__).parent.parent)
    args = parser.parse_args()

    members = archive_members(args.archive)
    required = {
        "languages/angelscript/LICENSES": [
            "first-party Unlicense",
            "first-party CC0",
            "AngelScript BSD-3-Clause",
        ],
        "languages/openplanet_manifest/LICENSES": [
            "first-party Unlicense",
            "first-party CC0",
            "TOML MIT",
        ],
    }
    notices = {
        "first-party Unlicense": args.repo / "UNLICENSE",
        "first-party CC0": args.repo / "LICENSE-CC0",
        "AngelScript BSD-3-Clause": args.repo / "docs/grammar-LICENSE-BSD-3-Clause",
        "TOML MIT": args.repo / "docs/toml-LICENSE-MIT",
    }
    missing: list[str] = []
    for path, notice_names in required.items():
        text = members.get(path)
        if text is None:
            missing.append(f"missing packaged file: {path}")
            continue
        for name in notice_names:
            if not notice_is_present(text, notices[name].read_text()):
                missing.append(f"{path} missing {name}")
    if missing:
        raise SystemExit("missing packaged notices: " + "; ".join(missing))
    print("all required license notices are present in the packaged language directories")


if __name__ == "__main__":
    main()

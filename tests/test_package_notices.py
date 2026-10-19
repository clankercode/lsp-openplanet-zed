"""Regression tests for the package-notice checker command-line boundary."""

from __future__ import annotations

import io
import subprocess
import sys
import tarfile
import tempfile
import unittest
from pathlib import Path


REPO = Path(__file__).resolve().parents[1]
CHECKER = REPO / "scripts/check-package-notices.py"
FIRST_PARTY = ("UNLICENSE", "LICENSE-CC0")
LANGUAGE_NOTICES = {
    "languages/angelscript/LICENSES": (
        *FIRST_PARTY,
        "docs/grammar-LICENSE-BSD-3-Clause",
    ),
    "languages/openplanet_manifest/LICENSES": (*FIRST_PARTY, "docs/toml-LICENSE-MIT"),
}


def notice_text(path: str) -> str:
    return (REPO / path).read_text().strip()


def valid_bundles() -> dict[str, str]:
    return {
        path: "\n\n".join(notice_text(notice) for notice in notices) + "\n"
        for path, notices in LANGUAGE_NOTICES.items()
    }


class PackageNoticeTests(unittest.TestCase):
    def check_archive(self, files: dict[str, str], prefix: str = "") -> subprocess.CompletedProcess:
        # Synthetic archives exercise checker failures; release artifacts come from the real CLI.
        with tempfile.TemporaryDirectory(prefix="zed-notice-test-") as temporary:
            archive_path = Path(temporary) / "archive.tar.gz"
            with tarfile.open(archive_path, "w:gz") as archive:
                for path, text in files.items():
                    data = text.encode()
                    member = tarfile.TarInfo(prefix + path)
                    member.size = len(data)
                    archive.addfile(member, io.BytesIO(data))
            return subprocess.run(
                [sys.executable, str(CHECKER), str(archive_path), "--repo", str(REPO)],
                capture_output=True,
                text=True,
                check=False,
            )

    def test_complete_in_language_bundles_pass_with_both_tar_path_styles(self):
        for prefix in ("", "./"):
            with self.subTest(prefix=prefix):
                result = self.check_archive(valid_bundles(), prefix)
                self.assertEqual(result.returncode, 0, result.stderr)
                self.assertIn("all required license notices", result.stdout)

    def test_loose_bundle_alone_fails(self):
        result = self.check_archive({"languages/LICENSES": (REPO / "languages/LICENSES").read_text()})
        self.assertNotEqual(result.returncode, 0)
        for path in LANGUAGE_NOTICES:
            self.assertIn(path, result.stderr)

    def test_each_language_bundle_is_required_even_with_loose_copy(self):
        for missing_path in LANGUAGE_NOTICES:
            with self.subTest(path=missing_path):
                files = valid_bundles()
                del files[missing_path]
                files["languages/LICENSES"] = (REPO / "languages/LICENSES").read_text()
                result = self.check_archive(files)
                self.assertNotEqual(result.returncode, 0)
                self.assertIn(missing_path, result.stderr)

    def test_every_complete_notice_is_required_in_its_own_language_bundle(self):
        for path, notices in LANGUAGE_NOTICES.items():
            for notice in notices:
                for mutation in ("missing", "truncated", "reordered"):
                    with self.subTest(path=path, notice=notice, mutation=mutation):
                        files = valid_bundles()
                        original = notice_text(notice)
                        lines = original.splitlines()
                        changed = {
                            "missing": "",
                            "truncated": "\n".join(lines[:-1]),
                            "reordered": "\n".join(reversed(lines)),
                        }[mutation]
                        files[path] = files[path].replace(original, changed)
                        # An intact copy elsewhere must not conceal an incomplete language bundle.
                        files["languages/LICENSES"] = (REPO / "languages/LICENSES").read_text()
                        result = self.check_archive(files)
                        self.assertNotEqual(result.returncode, 0)
                        self.assertIn(path, result.stderr)

    def test_upstream_notices_must_match_the_language(self):
        paths = list(LANGUAGE_NOTICES)
        files = valid_bundles()
        files[paths[0]], files[paths[1]] = files[paths[1]], files[paths[0]]
        files["languages/LICENSES"] = (REPO / "languages/LICENSES").read_text()
        result = self.check_archive(files)
        self.assertNotEqual(result.returncode, 0)
        for path in paths:
            self.assertIn(path, result.stderr)

    def test_repository_language_bundles_contain_complete_notices(self):
        for path, notices in LANGUAGE_NOTICES.items():
            with self.subTest(path=path):
                bundle = (REPO / path).read_text()
                for notice in notices:
                    self.assertIn(notice_text(notice), bundle, notice)


if __name__ == "__main__":
    unittest.main()

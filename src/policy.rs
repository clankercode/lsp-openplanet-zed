use zed_extension_api::{Architecture, Os, Result};

pub(crate) struct Platform {
    pub target: &'static str,
    pub windows: bool,
}

impl Platform {
    pub fn from_zed(os: Os, arch: Architecture) -> Result<Self> {
        let target =
            match (os, arch) {
                (Os::Linux, Architecture::X8664) => "x86_64-unknown-linux-gnu",
                (Os::Linux, Architecture::Aarch64) => "aarch64-unknown-linux-gnu",
                (Os::Mac, Architecture::X8664) => "x86_64-apple-darwin",
                (Os::Mac, Architecture::Aarch64) => "aarch64-apple-darwin",
                (Os::Windows, Architecture::X8664) => "x86_64-pc-windows-msvc",
                (Os::Windows, Architecture::Aarch64) => "aarch64-pc-windows-msvc",
                _ => return Err(
                    "No official openplanet-lsp build for this platform; configure binary.path."
                        .into(),
                ),
            };
        Ok(Self {
            target,
            windows: matches!(os, Os::Windows),
        })
    }

    pub fn executable(&self) -> &'static str {
        if self.windows {
            "openplanet-lsp.exe"
        } else {
            "openplanet-lsp"
        }
    }

    pub fn archive_name(&self, version: &str) -> Result<String> {
        let version = version.strip_prefix('v').unwrap_or(version);
        if version.is_empty()
            || !version
                .bytes()
                .all(|c| c.is_ascii_alphanumeric() || b".-+".contains(&c))
        {
            return Err("Invalid official release version".into());
        }
        let extension = if self.windows { "zip" } else { "tar.gz" };
        Ok(format!(
            "openplanet-lsp-v{version}-{}.{}",
            self.target, extension
        ))
    }
}

pub(crate) fn configured_path(path: Option<&str>) -> Result<Option<&str>> {
    match path {
        Some(path) if path.trim().is_empty() => {
            Err("lsp.openplanet-lsp.binary.path must not be empty".into())
        }
        path => Ok(path),
    }
}

pub(crate) fn server_args(extra: Option<&[String]>) -> Vec<String> {
    let mut args = vec!["--lsp".to_owned()];
    if let Some(extra) = extra {
        args.extend(extra.iter().filter(|arg| arg.as_str() != "--lsp").cloned());
    }
    args
}

/// Retain the last *successful* install for a fresh extension instance when
/// GitHub is unavailable. Never treat a partial extraction as a cache entry.
pub(crate) fn download_with_cache(
    work_directory: &std::path::Path,
    platform: &Platform,
    download: impl FnOnce() -> Result<String>,
) -> Result<String> {
    let marker = work_directory.join(format!("last-successful-{}.txt", platform.target));
    match download() {
        Ok(binary) => {
            // Cache persistence is best effort; a read-only cache must not stop
            // an otherwise successful server launch.
            let _ = std::fs::write(marker, &binary);
            Ok(binary)
        }
        Err(download_error) => {
            if let Ok(binary) = std::fs::read_to_string(marker)
                && valid_cached_path(platform, &binary)
                && work_directory.join(&binary).is_file()
            {
                return Ok(binary);
            }
            Err(download_error)
        }
    }
}

fn valid_cached_path(platform: &Platform, binary: &str) -> bool {
    // The marker is data, not an arbitrary path supplied to the host.
    let Some(directory) = binary.strip_suffix(&format!("/{}", platform.executable())) else {
        return false;
    };
    let Some(version) = directory
        .strip_prefix("openplanet-lsp-")
        .and_then(|value| value.strip_suffix(&format!("-{}", platform.target)))
    else {
        return false;
    };
    platform.archive_name(version).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn official_assets_cover_all_six_platforms() {
        for (os, arch, target, extension, executable) in [
            (
                Os::Linux,
                Architecture::X8664,
                "x86_64-unknown-linux-gnu",
                "tar.gz",
                "openplanet-lsp",
            ),
            (
                Os::Linux,
                Architecture::Aarch64,
                "aarch64-unknown-linux-gnu",
                "tar.gz",
                "openplanet-lsp",
            ),
            (
                Os::Mac,
                Architecture::X8664,
                "x86_64-apple-darwin",
                "tar.gz",
                "openplanet-lsp",
            ),
            (
                Os::Mac,
                Architecture::Aarch64,
                "aarch64-apple-darwin",
                "tar.gz",
                "openplanet-lsp",
            ),
            (
                Os::Windows,
                Architecture::X8664,
                "x86_64-pc-windows-msvc",
                "zip",
                "openplanet-lsp.exe",
            ),
            (
                Os::Windows,
                Architecture::Aarch64,
                "aarch64-pc-windows-msvc",
                "zip",
                "openplanet-lsp.exe",
            ),
        ] {
            let platform = Platform::from_zed(os, arch).unwrap();
            for version in ["0.8.0", "v0.8.0"] {
                assert_eq!(
                    platform.archive_name(version).unwrap(),
                    format!("openplanet-lsp-v0.8.0-{target}.{extension}")
                );
            }
            assert_eq!(platform.executable(), executable);
        }
    }

    #[test]
    fn rejects_unsupported_platform_and_unsafe_versions() {
        assert!(Platform::from_zed(Os::Linux, Architecture::X86).is_err());
        let platform = Platform::from_zed(Os::Linux, Architecture::X8664).unwrap();
        for version in ["", "v", "../0.8.0", "0.8/0", "0.8.0\n"] {
            assert!(platform.archive_name(version).is_err());
        }
    }

    #[test]
    fn explicit_paths_are_authoritative_and_empty_is_a_visible_error() {
        assert_eq!(configured_path(None).unwrap(), None);
        for path in [
            "/missing/server",
            "relative/server",
            "C:\\tools\\openplanet-lsp.exe",
            "/a path/server",
        ] {
            assert_eq!(configured_path(Some(path)).unwrap(), Some(path));
        }
        for path in ["", " ", "\t"] {
            assert!(configured_path(Some(path)).is_err());
        }
    }

    #[test]
    fn stdio_is_mandatory_and_extra_arguments_are_preserved() {
        assert_eq!(server_args(None), ["--lsp"]);
        assert_eq!(
            server_args(Some(&[
                "--lsp".into(),
                "--log-level".into(),
                "debug".into()
            ])),
            ["--lsp", "--log-level", "debug"]
        );
    }

    #[test]
    fn fresh_resolver_uses_last_successful_install_when_release_lookup_fails() {
        let root =
            std::env::temp_dir().join(format!("openplanet-zed-cache-{}", std::process::id()));
        std::fs::create_dir_all(&root).unwrap();
        for (os, arch) in [
            (Os::Linux, Architecture::X8664),
            (Os::Windows, Architecture::Aarch64),
        ] {
            let platform = Platform::from_zed(os, arch).unwrap();
            let binary = format!(
                "openplanet-lsp-v0.8.0-{}/{}",
                platform.target,
                platform.executable()
            );
            std::fs::create_dir_all(root.join(&binary).parent().unwrap()).unwrap();
            std::fs::write(root.join(&binary), "completed download").unwrap();
            assert_eq!(
                download_with_cache(&root, &platform, || Ok(binary.clone())).unwrap(),
                binary
            );
            // A new call has no state from the first invocation.
            assert_eq!(
                download_with_cache(&root, &platform, || Err("offline".into())).unwrap(),
                binary
            );
            std::fs::remove_file(root.join(&binary)).unwrap();
            assert_eq!(
                download_with_cache(&root, &platform, || Err("offline".into())).unwrap_err(),
                "offline"
            );
        }
        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn cached_marker_rejects_paths_outside_the_expected_platform_install() {
        let platform = Platform::from_zed(Os::Linux, Architecture::X8664).unwrap();
        for path in [
            "/bin/openplanet-lsp",
            "../openplanet-lsp",
            "openplanet-lsp-v../x86_64-unknown-linux-gnu/openplanet-lsp",
            "openplanet-lsp-v0.8.0-aarch64-unknown-linux-gnu/openplanet-lsp",
        ] {
            assert!(!valid_cached_path(&platform, path), "accepted {path}");
        }
    }
}

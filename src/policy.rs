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
}

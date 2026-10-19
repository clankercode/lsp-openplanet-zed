mod policy;

use std::{fs, path::Path};
use zed_extension_api::{self as zed, Result, settings::LspSettings};

#[derive(Default)]
struct Openplanet {
    downloaded_binary: Option<String>,
}

impl Openplanet {
    fn binary(
        &mut self,
        id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
        settings: &LspSettings,
    ) -> Result<String> {
        let configured = settings.binary.as_ref().and_then(|b| b.path.as_deref());
        // An explicit path is authoritative, including a bad path. Let Zed display
        // the spawn error instead of silently selecting a different installation.
        if let Some(path) = policy::configured_path(configured)? {
            return Ok(path.to_owned());
        }
        if let Some(path) = worktree.which("openplanet-lsp") {
            return Ok(path);
        }
        if let Some(path) = &self.downloaded_binary {
            if Path::new(path).is_file() {
                return Ok(path.clone());
            }
        }
        zed::set_language_server_installation_status(
            id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );
        let result = self.download(id);
        if result.is_err() {
            zed::set_language_server_installation_status(
                id,
                &zed::LanguageServerInstallationStatus::None,
            );
        }
        result
    }

    fn download(&mut self, id: &zed::LanguageServerId) -> Result<String> {
        let (os, arch) = zed::current_platform();
        let platform = policy::Platform::from_zed(os, arch)?;
        let release = zed::latest_github_release(
            "clankercode/lsp-openplanet",
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;
        let archive = platform.archive_name(&release.version)?;
        let asset = release.assets.iter().find(|asset| asset.name == archive)
            .ok_or_else(|| format!("Official release {} has no {archive}. Set lsp.openplanet-lsp.binary.path to an installed server.", release.version))?;
        let directory = format!("openplanet-lsp-{}-{}", release.version, platform.target);
        let binary = format!("{directory}/{}", platform.executable());
        if !Path::new(&binary).is_file() {
            // A failed extraction must not become a successful cache hit.
            if Path::new(&directory).exists() {
                fs::remove_dir_all(&directory).map_err(|err| err.to_string())?;
            }
            zed::set_language_server_installation_status(
                id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );
            zed::download_file(
                &asset.download_url,
                &directory,
                if platform.windows {
                    zed::DownloadedFileType::Zip
                } else {
                    zed::DownloadedFileType::GzipTar
                },
            )?;
            if !Path::new(&binary).is_file() {
                return Err(format!(
                    "{archive} did not contain {} at its root",
                    platform.executable()
                ));
            }
        }
        zed::make_file_executable(&binary)?;
        self.downloaded_binary = Some(binary.clone());
        zed::set_language_server_installation_status(
            id,
            &zed::LanguageServerInstallationStatus::None,
        );
        Ok(binary)
    }
}

impl zed::Extension for Openplanet {
    fn new() -> Self {
        Self::default()
    }

    fn language_server_command(
        &mut self,
        id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let settings = LspSettings::for_worktree(id.as_ref(), worktree)?;
        let command = self.binary(id, worktree, &settings)?;
        let args = policy::server_args(
            settings
                .binary
                .as_ref()
                .and_then(|b| b.arguments.as_deref()),
        );
        let env = settings
            .binary
            .and_then(|b| b.env)
            .unwrap_or_default()
            .into_iter()
            .collect();
        Ok(zed::Command { command, args, env })
    }

    fn language_server_initialization_options(
        &mut self,
        id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<zed::serde_json::Value>> {
        Ok(LspSettings::for_worktree(id.as_ref(), worktree)?.initialization_options)
    }
}

zed::register_extension!(Openplanet);

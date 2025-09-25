use clap::{Args, Parser, Subcommand};
use dagger_lib::{InstallableMod, ListableMod, Str, UpgradableMod};

#[derive(Debug, Clone, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub cmd: Commands,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Commands {
    /// Install a Balatro mod.
    Install(InstallCommandArgs),
    /// Uninstall a Balatro mod.
    Uninstall(UninstallCommandArgs),
    /// Update Balatro mods.
    Update(UpdateCommandArgs),
    /// Lists all installed Balatro mods.
    List(ListCommandArgs),
}

#[derive(Debug, Clone, Args)]
pub struct InstallCommandArgs {
    /// The URL of the mod repository. A URL is valid if it links to a Git repository.
    /// "Maintainer/Repository" is also valid, and is processed as a link to Github.
    url: String,
    #[arg(long)]
    /// The name of the folder to install the mod in. Defaults to the name of the repository.
    /// Dagger will use the name of the folder as the mod's id.
    pub id: Option<String>,
    #[arg(short, long)]
    /// The branch of the mod that you want to install. Defaults to the main branch.
    pub branch: Option<String>,
    #[arg(short, long)]
    /// The tag corresponding to the version of the mod that you want to install.
    /// Defaults to the latest commit. Use * to get the latest tag/release.
    pub tag: Option<String>,
}

#[derive(Debug, Clone, Args)]
pub struct UninstallCommandArgs {
    /// ID of the mod to be uninstalled.
    pub id: String,
}

#[derive(Debug, Clone, Args)]
pub struct UpdateCommandArgs {
    #[command(subcommand)]
    pub cmd: UpdateCommands,
}

#[derive(Debug, Clone, Subcommand)]
pub enum UpdateCommands {
    /// Update a specified mod.
    Item(UpdateItem),
    /// Update all installed mods.
    All,
}

impl UpdateCommands {
    pub fn as_item(&self) -> Option<&UpdateItem> {
        if let Self::Item(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[must_use]
    pub fn is_all(&self) -> bool {
        matches!(self, Self::All)
    }
}

#[derive(Debug, Clone, Args)]
pub struct UpdateItem {
    /// The name of the folder that contains the mod you want to update.
    pub id: String,
    #[arg(short, long)]
    /// The branch of the mod that you want to install the update from. Defaults to the main branch.
    pub branch: Option<String>,
    #[arg(short, long)]
    /// The tag corresponding to the version of the mod that you want to update to.
    /// Defaults to the latest commit. Use * to get the latest tag/release.
    pub tag: Option<String>,
}

impl InstallCommandArgs {
    pub fn get_id(&mut self) -> &str {
        if self.id.is_none() {
            let id = self
                .url
                .split("/")
                .last()
                .map(|s| s.trim_end_matches(".git"))
                .unwrap_or_default();

            self.id = Some(id.to_string());

            id
        } else if let Some(s) = self.id.as_deref() {
            s
        } else {
            unreachable!()
        }
    }

    pub fn url(&self) -> String {
        if self.url.starts_with("https://") || self.url.starts_with("http://") {
            return self.url.clone();
        }

        format!(
            "https://www.github.com/{}.git",
            self.url.as_str().trim_end_matches(".git")
        )
    }
}

#[derive(Debug, Clone, Args)]
pub struct ListCommandArgs {
    #[command(subcommand)]
    pub cmd: ListCommands,
    #[arg(short, long, default_value_t = false)]
    /// Displays every tag that the mod repository has.
    pub list_tags: bool,
}

#[derive(Debug, Clone, Subcommand)]
pub enum ListCommands {
    /// Lists the installation information of all installed mods.
    All,
    /// Lists the installation information of a specific mod.
    Item(ListArgs),
}

impl ListCommands {
    #[must_use]
    pub fn is_all(&self) -> bool {
        matches!(self, Self::All)
    }

    pub fn as_item(&self) -> Option<&ListArgs> {
        if let Self::Item(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Args)]
pub struct ListArgs {
    /// The ID of the mod whose information you want to get.
    id: String,
}

impl ListArgs {
    pub fn id(&self) -> &str {
        &self.id
    }
}

impl InstallableMod for InstallCommandArgs {
    fn get_url(&self) -> Str<'_> {
        Str::Owned(self.url())
    }

    fn get_id(&self) -> Str<'_> {
        Str::Owned(self.id.clone().unwrap_or_else(|| {
            self.url()
                .split('/')
                .next_back()
                .map(|s| s.trim_end_matches(".git"))
                .unwrap_or_default()
                .to_string()
        }))
    }

    fn get_branch(&self) -> Option<Str<'_>> {
        self.branch.as_deref().map(Str::Borrowed)
    }

    fn get_tag(&self) -> Option<Str<'_>> {
        self.tag.as_deref().map(Str::Borrowed)
    }
}

impl UpgradableMod for UpdateItem {
    fn get_id(&self) -> Str<'_> {
        Str::Borrowed(self.id.as_str())
    }

    fn get_branch(&self) -> Option<Str<'_>> {
        self.branch.as_deref().map(Str::Borrowed)
    }

    fn get_tag(&self) -> Option<Str<'_>> {
        self.tag.as_deref().map(Str::Borrowed)
    }
}

impl UpgradableMod for &UpdateItem {
    fn get_id(&self) -> Str<'_> {
        Str::Borrowed(self.id.as_str())
    }

    fn get_branch(&self) -> Option<Str<'_>> {
        self.branch.as_deref().map(Str::Borrowed)
    }

    fn get_tag(&self) -> Option<Str<'_>> {
        self.tag.as_deref().map(Str::Borrowed)
    }
}

impl ListableMod for ListCommandArgs {
    fn get_id(&self) -> Option<Str<'_>> {
        self.cmd.as_item().map(|arg| arg.id()).map(Str::Borrowed)
    }

    fn list_tags(&self) -> bool {
        self.list_tags
    }
}

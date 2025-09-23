use clap::{Args, Parser, Subcommand};

#[derive(Debug, Clone, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub cmd: Commands,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Commands {
    Install(InstallCommandArgs),
    Update(UpdateCommandArgs),
    List,
}

#[derive(Debug, Clone, Args)]
pub struct InstallCommandArgs {
    #[arg(short, long)]
    pub url: String,
    #[arg(long)]
    pub id: Option<String>,
    #[arg(short, long)]
    pub branch: Option<String>,
    #[arg(short, long)]
    pub tag: Option<String>,
}

#[derive(Debug, Clone, Args)]
pub struct UpdateCommandArgs {
    #[command(subcommand)]
    pub cmd: UpdateCommands,
}

#[derive(Debug, Clone, Subcommand)]
pub enum UpdateCommands {
    Item(UpdateItem),
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
    #[arg(short, long)]
    pub id: String,
    #[arg(short, long)]
    pub branch: Option<String>,
    #[arg(short, long)]
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
}

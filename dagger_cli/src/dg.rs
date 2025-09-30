use clap::Parser;
use dagger_lib::Result;

use crate::{cli::*, spinner::*};

mod cli;
mod git;
mod manager;
mod spinner;

fn main() -> Result<()> {
    Ok(())
}

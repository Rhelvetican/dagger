use std::{
    cell::RefCell,
    fs::{create_dir_all, File},
    io::Read,
    rc::Rc,
};

use clap::Parser;
use mlua::Lua;

use dagger::*;

fn main() -> Result<(), DaggerError> {
    let args = CliArgs::parse();

    let specs = Rc::new(RefCell::new(DaggerSpecManager::new()));
    let lua = Lua::new();
    load_dagger_lua_api(&lua, Rc::clone(&specs))?;

    let mut script = Directories::config_dir();
    let _ = create_dir_all(&script);
    script.push("init.lua");

    if let Ok(mut f) = File::open(&script) {
        println!("[Lua] Executing script...");

        let mut buf = Vec::new();
        f.read_to_end(&mut buf)?;
        lua.load(&buf).exec()?;
    }

    dbg!(&specs);

    let path = Directories::mod_dir();
    let updater =
        DaggerModManager::new(&path, Rc::unwrap_or_clone(specs).into_inner().into_inner());

    match args.cmd {
        Commands::Update(update_args) => {
            if update_args.all() {
                updater.update_all()?;
            } else if let Some(update_entry) = update_args.get_mod() {
                updater.update_single(update_entry)?;
            }
        }
        _ => println!("Wait for the next update!"),
    }

    Ok(())
}

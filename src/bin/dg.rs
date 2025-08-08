use std::{
    error::Error,
    fs::{create_dir_all, File},
    io::Read,
};

use dagger::{load_dagger_lua_api, Directories};
use mlua::Lua;

fn main() -> Result<(), Box<dyn Error>> {
    let lua = Lua::new();
    load_dagger_lua_api(&lua)?;

    let mut script = Directories::config_dir();
    let _ = create_dir_all(&script);

    println!("{}", script.display());

    script.push("init.lua");

    if let Ok(mut f) = File::open(&script) {
        println!("Lua script found. Executing...");

        let mut buf = Vec::new();
        f.read_to_end(&mut buf)?;
        lua.load(&buf).exec()?;
    }

    println!("{}", Directories::mod_dir().display());

    Ok(())
}

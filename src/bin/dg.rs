use std::{
    cell::RefCell,
    error::Error,
    fs::{create_dir_all, File},
    io::Read,
    rc::Rc,
};

use dagger::{load_dagger_lua_api, DaggerSpecManager, Directories};
use mlua::Lua;

mod cli;

fn main() -> Result<(), Box<dyn Error>> {
    let specs = Rc::new(RefCell::new(DaggerSpecManager::new()));
    let lua = Lua::new();
    load_dagger_lua_api(&lua, Rc::clone(&specs))?;

    let mut script = Directories::config_dir();
    let _ = create_dir_all(&script);
    script.push("init.lua");

    if let Ok(mut f) = File::open(&script) {
        let mut buf = Vec::new();
        f.read_to_end(&mut buf)?;
        lua.load(&buf).exec()?;
    }

    dbg!(&specs);

    Ok(())
}

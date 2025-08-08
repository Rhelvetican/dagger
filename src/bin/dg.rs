use std::{error::Error, fs::File, io::Read};

use dagger::{load_dagger_lua_api, Directories};
use mlua::Lua;

fn main() -> Result<(), Box<dyn Error>> {
    let lua = Lua::new();
    load_dagger_lua_api(&lua)?;

    if let Ok(mut f) = File::open(Directories::config_dir()) {
        let mut buf = Vec::new();
        f.read_to_end(&mut buf)?;
        lua.load(&buf).exec()?;
    }

    Ok(())
}

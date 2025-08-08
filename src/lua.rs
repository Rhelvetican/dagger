use mlua::{Lua, Result};

pub fn load_dagger_lua_api(lua: &Lua) -> Result<()> {
    let global = lua.globals();
    let tbl = lua.create_table()?;

    global.set("Dagger", tbl)
}

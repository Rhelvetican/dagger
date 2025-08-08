use mlua::{Lua, Result, Value};

pub fn load_dagger_lua_api(lua: &Lua) -> Result<()> {
    let global = lua.globals();
    let tbl = lua.create_table()?;

    let add_func = lua.create_function_mut(|_, spec: Value| Ok(()))?;
    tbl.set("add", add_func)?;

    global.set("Dagger", tbl)
}

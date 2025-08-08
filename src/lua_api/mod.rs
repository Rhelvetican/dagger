use std::{cell::RefCell, rc::Rc};

use mlua::{FromLua, Lua, Result, Value};

use crate::{models::DaggerSpecManager, DaggerSpecification};

pub fn load_dagger_lua_api(lua: &Lua, specs: Rc<RefCell<DaggerSpecManager>>) -> Result<()> {
    let global = lua.globals();
    let api = lua.create_table()?;

    let add_func = lua.create_function_mut(move |_, arg: Value| {
        DaggerSpecification::from_value(arg, Rc::clone(&specs))
    })?;

    api.set("add", &add_func)?;
    api.set("register", &add_func)?;
    global.set("Dagger", api)?;
    Ok(())
}

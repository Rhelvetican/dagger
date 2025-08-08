use std::{cell::RefCell, rc::Rc};

use mlua::{FromLua, Lua, Result, Value};

use crate::{models::DaggerSpecManager, DaggerSpecification};

pub fn load_dagger_lua_api(lua: &Lua, specs: Rc<RefCell<DaggerSpecManager>>) -> Result<()> {
    let global = lua.globals();
    let api = lua.create_table()?;

    let add_func = lua.create_function_mut(move |lua, arg: Value| {
        if let Ok(spec) = DaggerSpecification::from_lua(arg, lua)
            && let Ok(mut mut_guard) = specs.try_borrow_mut()
        {
            mut_guard.push(spec);
        };

        Ok(())
    })?;

    api.set("add", &add_func)?;
    api.set("register", &add_func)?;
    global.set("Dagger", api)?;
    Ok(())
}

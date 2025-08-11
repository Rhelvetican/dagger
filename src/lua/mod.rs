use std::{cell::RefCell, collections::HashMap, rc::Rc};

use mlua::{Lua, Result};

use crate::{specification::DaggerSpecification, BoxedStr};

pub fn load_dagger_lua_api(
    lua: &Lua,
    specs: Rc<RefCell<HashMap<BoxedStr, DaggerSpecification>>>,
) -> Result<()> {
    let global = lua.globals();
    let api = lua.create_table()?;

    global.set("Dagger", api)?;
    Ok(())
}

use mlua::{FromLua, Lua, Value};

#[inline]
pub fn lua_optional<T: FromLua>(v: Value, lua: &Lua) -> Option<T> {
    match v {
        Value::Nil => None,
        other => T::from_lua(other, lua).ok(),
    }
}

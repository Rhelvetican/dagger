use dagger_balatro::err::DaggerError;
use mimalloc_rust::*;
use mlua::Lua;

#[global_allocator]
static GLOBAL_MIMALLOC: GlobalMiMalloc = GlobalMiMalloc;

fn main() -> Result<(), DaggerError> {
    let lua = Lua::new();
    Ok(())
}

use dagger_balatro::err::DaggerError;
use mimalloc_rust::*;

#[global_allocator]
static GLOBAL_MIMALLOC: GlobalMiMalloc = GlobalMiMalloc;

fn main() -> Result<(), DaggerError> {
    Ok(())
}

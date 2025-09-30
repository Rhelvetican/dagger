#[macro_export]
macro_rules! sub_mod {
    ($($id:ident),+) => {
        $(
            mod $id;
            pub use $id::*;
        )+
    };
}

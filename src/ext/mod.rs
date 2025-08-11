use crate::BoxedStr;

pub trait IntoBoxedStr {
    fn into_boxed_str(self) -> BoxedStr;
}

impl IntoBoxedStr for &str {
    fn into_boxed_str(self) -> BoxedStr {
        BoxedStr::from(self)
    }
}

impl IntoBoxedStr for String {
    fn into_boxed_str(self) -> BoxedStr {
        BoxedStr::from(self)
    }
}

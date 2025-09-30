use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq)]
pub struct CowStr<'a>(Cow<'a, str>);

impl<'a> std::ops::Deref for CowStr<'a> {
    type Target = Cow<'a, str>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> std::ops::DerefMut for CowStr<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

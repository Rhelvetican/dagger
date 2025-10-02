use std::{
    borrow::Cow,
    fmt::{Display, Formatter, Result},
    ops::Deref,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CowStr<'a>(Cow<'a, str>);

impl<'a> CowStr<'a> {
    #[inline]
    pub fn as_str(&self) -> &str {
        self
    }
}

impl<'a> From<&'a str> for CowStr<'a> {
    #[inline]
    fn from(value: &'a str) -> Self {
        Self(Cow::Borrowed(value))
    }
}

impl From<String> for CowStr<'_> {
    #[inline]
    fn from(value: String) -> Self {
        Self(Cow::Owned(value))
    }
}

impl<'a> CowStr<'a> {
    #[inline]
    pub fn new<S: Into<CowStr<'a>>>(s: S) -> Self {
        s.into()
    }
}

impl<'a> std::ops::Deref for CowStr<'a> {
    type Target = Cow<'a, str>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> std::ops::DerefMut for CowStr<'a> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a, T> AsRef<T> for CowStr<'a>
where
    str: AsRef<T>,
{
    #[inline]
    fn as_ref(&self) -> &T {
        <str as AsRef<T>>::as_ref(self)
    }
}

impl<'a, 'b: 'a> PartialEq<&'b str> for CowStr<'a> {
    #[inline]
    fn eq(&self, other: &&'b str) -> bool {
        self.deref() == other
    }
}

impl PartialEq<str> for CowStr<'_> {
    #[inline]
    fn eq(&self, other: &str) -> bool {
        self.deref() == other
    }
}

impl Display for CowStr<'_> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.deref())
    }
}

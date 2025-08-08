use std::ops::{Deref, DerefMut};

use crate::models::DaggerSpecification;

#[derive(Clone, Debug)]
pub struct DaggerSpecManager {
    inner: Vec<DaggerSpecification>,
}

impl DaggerSpecManager {
    #[inline]
    pub fn new() -> Self {
        Self { inner: vec![] }
    }
}

impl Deref for DaggerSpecManager {
    type Target = Vec<DaggerSpecification>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for DaggerSpecManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

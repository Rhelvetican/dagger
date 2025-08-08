use std::{
    collections::HashSet,
    ops::{Deref, DerefMut},
};

use crate::models::DaggerSpecification;

#[derive(Clone, Debug)]
pub struct DaggerSpecManager {
    inner: HashSet<DaggerSpecification>,
}

impl DaggerSpecManager {
    #[inline]
    pub fn new() -> Self {
        Self {
            inner: HashSet::new(),
        }
    }
}

impl Deref for DaggerSpecManager {
    type Target = HashSet<DaggerSpecification>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for DaggerSpecManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use crate::models::DaggerSpecification;

#[derive(Clone, Debug, Default)]
pub struct DaggerSpecManager {
    inner: HashMap<Box<str>, DaggerSpecification>,
}

impl DaggerSpecManager {
    #[inline]
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    #[inline]
    pub fn insert(&mut self, key: &str, spec: DaggerSpecification) {
        self.inner.insert(Box::from(key), spec);
    }

    #[inline]
    pub fn insert_owned(&mut self, key: Box<str>, spec: DaggerSpecification) {
        self.inner.insert(key, spec);
    }

    #[inline]
    pub fn into_inner(self) -> HashMap<Box<str>, DaggerSpecification> {
        self.inner
    }
}

impl Deref for DaggerSpecManager {
    type Target = HashMap<Box<str>, DaggerSpecification>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for DaggerSpecManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

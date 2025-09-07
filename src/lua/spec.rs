use std::{
    collections::HashMap,
    fs::File,
    io::BufReader,
    ops::{Deref, DerefMut},
    path::Path,
    str::FromStr,
};

use serde_json::from_reader;
use url::Url;

use crate::{core::metadata::SmodsMetadata, err::DaggerError};

#[derive(Debug, Clone, Default)]
pub enum ModSourceType {
    #[default]
    Github,
    Local,
    GitService {
        fmt: String,
    },
}

#[derive(Debug, Clone, Default)]
pub enum GitRef {
    Commit(String),
    Tag(String),
    #[default]
    Latest,
}

#[derive(Debug, Clone, Default)]
pub struct ModSpec {
    pub src_ty: ModSourceType,
    branch: Option<String>,
    objref: Option<GitRef>,
    pub directory: String,
}

#[derive(Debug, Clone, Default)]
pub struct ModSpecifications {
    inner: HashMap<String, ModSpec>,
}

impl ModSpecifications {
    pub fn new() -> Self {
        Self::default()
    }
}

impl DerefMut for ModSpecifications {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl Deref for ModSpecifications {
    type Target = HashMap<String, ModSpec>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl FromStr for ModSpec {
    type Err = DaggerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mod_path = AsRef::<Path>::as_ref(s);
        if mod_path.is_dir()
            && let Some(name) = mod_path
                .components()
                .next_back()
                .map(|s| s.as_os_str().to_string_lossy().into_owned())
        {
            let x = mod_path.join(format!("{}.json", name));

            if File::open(&x)
                .map(BufReader::new)
                .map_err(DaggerError::from)
                .and_then(|rdr| from_reader::<_, SmodsMetadata>(rdr).map_err(DaggerError::from))
                .is_ok()
            {
                return Ok(Self {
                    src_ty: ModSourceType::Local,
                    directory: name,
                    ..Default::default()
                });
            };
        };

        if let Some((i, dir)) = s.split('/').enumerate().last()
            && i == 2
            && !dir.is_empty()
        {
            Ok(Self {
                src_ty: ModSourceType::Github,
                directory: dir.to_string(),
                ..Default::default()
            })
        } else if let Ok(url) = Url::parse(s) {
            Ok(Self {
                src_ty: ModSourceType::GitService {
                    fmt: format!("{}://{}/%s", url.scheme(), url.domain().unwrap_or("")),
                },
                directory: url
                    .path_segments()
                    .and_then(|s| s.filter(|s| !s.is_empty()).next_back().map(str::to_string))
                    .unwrap_or_default(),
                ..Default::default()
            })
        } else {
            Err(DaggerError::runtime("Invalid URL were supplied."))
        }
    }
}

impl ModSpec {
    #[inline]
    pub fn objref(&self) -> Option<&GitRef> {
        self.objref.as_ref()
    }

    #[inline]
    pub fn branch(&self) -> Option<&str> {
        self.branch.as_deref()
    }
}

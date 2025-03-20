use std::ops::{Deref, DerefMut};

use super::lib_package::LibPackage;
use crate::config::Opts;
use crate::internal_prelude::*;
use cargo_metadata::Metadata;

use super::{project::ProjectDefinition, ProjectConfig};

pub struct ExtraLibPackages(pub Vec<LibPackage>);

impl ExtraLibPackages {
    pub fn resolve(
        cli: &Opts,
        metadata: &Metadata,
        project: &ProjectDefinition,
        config: &ProjectConfig,
    ) -> Result<Self> {
        let extras = project
            .extra_lib_packages
            .iter()
            .map(|name| LibPackage::resolve(cli, metadata, project, config, Some(name.clone())))
            .collect::<Result<Vec<_>>>();
        Ok(Self(extras?))
    }
}

impl std::fmt::Debug for ExtraLibPackages {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.iter().map(|l| l.fmt(f)).nth(0).unwrap_or(Ok(()))
    }
}

impl Deref for ExtraLibPackages {
    type Target = Vec<LibPackage>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ExtraLibPackages {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

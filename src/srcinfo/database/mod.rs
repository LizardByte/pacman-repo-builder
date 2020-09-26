pub mod insert_srcinfo;
pub mod package_build_order;
pub mod text_wrapper;

use super::SrcInfo;
use package_build_order::PackageBuildOrder;
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

#[derive(Debug, Default)]
pub struct Database<PkgBase, PkgName, SrcInfoContent>
where
    PkgBase: AsRef<str> + Hash + Eq + Clone,
    PkgName: AsRef<str> + Hash + Eq + Clone,
    SrcInfoContent: AsRef<str>,
{
    base_to_name: HashMap<PkgBase, HashSet<PkgName>>,
    name_to_base: HashMap<PkgName, PkgBase>,
    infos: HashMap<PkgBase, SrcInfo<SrcInfoContent>>,
    dependencies: HashMap<PkgBase, HashSet<PkgBase>>,
    build_order: PackageBuildOrder,
}

impl<PkgBase, PkgName, SrcInfoContent> Database<PkgBase, PkgName, SrcInfoContent>
where
    PkgBase: AsRef<str> + Default + Hash + Eq + Clone,
    PkgName: AsRef<str> + Default + Hash + Eq + Clone,
    SrcInfoContent: AsRef<str> + Default,
{
    pub fn new() -> Self {
        Default::default()
    }

    pub fn base_to_name(&self) -> &HashMap<PkgBase, HashSet<PkgName>> {
        &self.base_to_name
    }

    pub fn name_to_base(&self) -> &HashMap<PkgName, PkgBase> {
        &self.name_to_base
    }

    pub fn infos(&self) -> &HashMap<PkgBase, SrcInfo<SrcInfoContent>> {
        &self.infos
    }

    pub fn dependencies(&self) -> &HashMap<PkgBase, HashSet<PkgBase>> {
        &self.dependencies
    }

    pub fn build_order(&self) -> &PackageBuildOrder {
        &self.build_order
    }
}

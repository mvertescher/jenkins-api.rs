use std::collections::HashMap;

use serde::Deserialize;

use crate::helpers::Class;

use super::{Artifact, Build, BuildStatus};
use crate::action::CommonAction;
use crate::changeset;
use crate::job::{MavenModule, MavenModuleSet};
use crate::user::ShortUser;

build_with_common_fields_and_impl!(
    /// A `Build` of a MavenModuleSet
    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct MavenModuleSetBuild<ParentJob = MavenModuleSet> {
        /// Change set for this build
        pub change_set: changeset::CommonChangeSetList,
        /// Version of maven
        pub maven_version_used: String,
        /// Which slave was it build on
        pub built_on: String,
        /// Artifacts from maven
        pub maven_artifacts: HashMap<String, Vec<crate::action::maven::ShortMavenArtifactRecord>>,
        /// List of user ids who made a change since the last non-broken build
        pub culprits: Vec<ShortUser>,
    }
);
register_class!("hudson.maven.MavenModuleSetBuild" => MavenModuleSetBuild);

impl MavenModuleSetBuild {}

build_with_common_fields_and_impl!(
    /// A `Build` of a MavenModule
    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct MavenBuild<ParentJob = MavenModule> {
        /// Change set for this build
        pub change_set: changeset::CommonChangeSetList,
        /// Which slave was it build on
        pub built_on: String,
        /// Artifacts from maven
        pub maven_artifacts: crate::action::maven::ShortMavenArtifactRecord,
        /// List of user ids who made a change since the last non-broken build
        pub culprits: Vec<ShortUser>,
    }
);
register_class!("hudson.maven.MavenBuild" => MavenBuild);

impl MavenBuild {}

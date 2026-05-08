use semver::Version;

/// Semantic version of the plugin host ABI exposed to effects.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PluginApiVersion(Version);

impl PluginApiVersion {
    #[must_use]
    pub fn new(major: u64, minor: u64, patch: u64) -> Self {
        Self(Version::new(major, minor, patch))
    }

    #[must_use]
    pub fn as_semver(&self) -> &Version {
        &self.0
    }
}

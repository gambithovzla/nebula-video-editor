use thiserror::Error;

#[derive(Debug, Error)]
pub enum PluginError {
    #[error("incompatible plugin API: host {host}, plugin {plugin}")]
    IncompatibleApi { host: String, plugin: String },
    #[error("plugin load failed: {0}")]
    Load(String),
}

/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use crate::expand_config_string;
use serde::Deserialize;
use std::path::PathBuf;
use url::Url;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Deserialize, Clone)]
pub struct FileSystemConfig {
    path: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SqlConfig {
    connection: Url,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum StoreConfig {
    FileSystem(FileSystemConfig),
    Sql(SqlConfig),
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for FileSystemConfig {
    fn default() -> Self {
        Self {
            path: Default::default(),
        }
    }
}

impl From<String> for FileSystemConfig {
    fn from(path: String) -> Self {
        Self { path }
    }
}

impl FileSystemConfig {
    pub fn root_path(&self) -> PathBuf {
        match expand_config_string(&self.path) {
            Some(s) => PathBuf::from(s),
            None => PathBuf::from(&self.path),
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl From<Url> for SqlConfig {
    fn from(connection: Url) -> Self {
        Self { connection }
    }
}

impl SqlConfig {
    pub fn connection(&self) -> &Url {
        &self.connection
    }
}

// ------------------------------------------------------------------------------------------------

//  impl From<PathBuf> for StoreConfig {
//      fn from(path: PathBuf) -> Self {
//          Self::FileSystem(path.into())
//      }
//  }

impl From<FileSystemConfig> for StoreConfig {
    fn from(v: FileSystemConfig) -> Self {
        Self::FileSystem(v)
    }
}

impl From<Url> for StoreConfig {
    fn from(connection: Url) -> Self {
        Self::Sql(connection.into())
    }
}

impl From<SqlConfig> for StoreConfig {
    fn from(v: SqlConfig) -> Self {
        Self::Sql(v)
    }
}

impl StoreConfig {
    pub fn is_file_system(&self) -> bool {
        matches!(self, Self::FileSystem(_))
    }

    pub fn as_file_system(&self) -> Option<&FileSystemConfig> {
        match self {
            Self::FileSystem(v) => Some(v),
            _ => None,
        }
    }

    pub fn is_sql(&self) -> bool {
        matches!(self, Self::Sql(_))
    }

    pub fn as_sql(&self) -> Option<&SqlConfig> {
        match self {
            Self::Sql(v) => Some(v),
            _ => None,
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

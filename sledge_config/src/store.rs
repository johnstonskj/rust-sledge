use crate::expand_config_string;
use serde::Deserialize;
use std::path::PathBuf;
use std::time::Duration;
use url::Url;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum StoreConfig {
    FileSystem(FileSystemConfig),
    Sql(SqlConfig),
}

#[derive(Debug, Deserialize, Clone)]
pub struct FileSystemConfig {
    path: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum SqlConfig {
    Connection(Url),
    Sqlite(Sqlite),
}

#[derive(Debug, Deserialize, Clone)]
pub struct Sqlite {
    path: PathBuf,
    auto_vacuum: Option<SqliteAutoVacuum>,
    busy_timeout: Option<Duration>,
    create_if_missing: Option<bool>,
    foreign_keys: Option<bool>,
    immutable: Option<bool>,
    journal_mode: Option<SqliteJournalMode>,
    locking_mode: Option<SqliteLockingMode>,
    page_size: Option<u32>,
    read_only: Option<bool>,
    serialized: Option<bool>,
    shared_cache: Option<bool>,
    statement_cache_capacity: Option<usize>,
    synchronous: Option<SqliteSynchronous>,
}

#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum SqliteAutoVacuum {
    None,
    Full,
    Incremental,
}

#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum SqliteJournalMode {
    Delete,
    Truncate,
    Persist,
    Memory,
    Wal,
    Off,
}

#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum SqliteLockingMode {
    Normal,
    Exclusive,
}

#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum SqliteSynchronous {
    Off,
    Normal,
    Full,
    Extra,
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
    fn from(v: Url) -> Self {
        Self::Connection(v)
    }
}

impl From<Sqlite> for SqlConfig {
    fn from(v: Sqlite) -> Self {
        Self::Sqlite(v)
    }
}

impl SqlConfig {
    pub fn is_connection_url(&self) -> bool {
        matches!(self, Self::Connection(_))
    }

    pub fn as_connection_url(&self) -> Option<&Url> {
        match self {
            Self::Connection(v) => Some(v),
            _ => None,
        }
    }

    pub fn is_sqlite(&self) -> bool {
        matches!(self, Self::Sqlite(_))
    }

    pub fn as_sqlite(&self) -> Option<&Sqlite> {
        match self {
            Self::Sqlite(v) => Some(v),
            _ => None,
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl From<PathBuf> for Sqlite {
    fn from(v: PathBuf) -> Self {
        Self::new(v)
    }
}

impl Sqlite {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            busy_timeout: Default::default(),
            create_if_missing: Default::default(),
            immutable: Default::default(),
            read_only: Default::default(),
            serialized: Default::default(),
            shared_cache: Default::default(),
            statement_cache_capacity: Default::default(),
            auto_vacuum: Default::default(),
            journal_mode: Some(SqliteJournalMode::default_for(false)),
            locking_mode: Default::default(),
            synchronous: Default::default(),
            foreign_keys: Default::default(),
            page_size: Default::default(),
        }
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn auto_vacuum(&self) -> SqliteAutoVacuum {
        self.auto_vacuum.unwrap_or_default()
    }

    pub fn busy_timeout(&self) -> Duration {
        self.busy_timeout.unwrap_or_else(|| Duration::new(5, 0))
    }

    pub fn create_if_missing(&self) -> bool {
        self.create_if_missing.unwrap_or_default()
    }

    pub fn foreign_keys(&self) -> bool {
        self.foreign_keys.unwrap_or_default()
    }

    pub fn immutable(&self) -> bool {
        self.immutable.unwrap_or_default()
    }

    pub fn journal_mode(&self) -> SqliteJournalMode {
        self.journal_mode
            .unwrap_or_else(|| SqliteJournalMode::default_for(false))
    }

    pub fn locking_mode(&self) -> SqliteLockingMode {
        self.locking_mode.unwrap_or_default()
    }

    pub fn page_size(&self) -> u32 {
        self.page_size.unwrap_or(4096)
    }

    pub fn read_only(&self) -> bool {
        self.read_only.unwrap_or_default()
    }

    pub fn serialized(&self) -> bool {
        self.serialized.unwrap_or_default()
    }

    pub fn shared_cache(&self) -> bool {
        self.shared_cache.unwrap_or_default()
    }

    pub fn statement_cache_capacity(&self) -> usize {
        self.statement_cache_capacity.unwrap_or_default()
    }

    pub fn synchronous(&self) -> SqliteSynchronous {
        self.synchronous.unwrap_or_default()
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for SqliteAutoVacuum {
    fn default() -> Self {
        Self::None
    }
}

// ------------------------------------------------------------------------------------------------

impl SqliteJournalMode {
    pub fn default_for(in_memory: bool) -> Self {
        if in_memory {
            Self::Memory
        } else {
            Self::Delete
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for SqliteLockingMode {
    fn default() -> Self {
        Self::Normal
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for SqliteSynchronous {
    fn default() -> Self {
        Self::Full
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

use crate::db::Pool;
use crate::error::Result;
use crate::repositories::RepositoryStatsUpdater;
use crate::{BuildQueue, Config, Index, Metrics, Storage};
use std::sync::Arc;

pub trait Context {
    fn config(&self) -> Result<Arc<Config>>;
    fn build_queue(&self) -> Result<Arc<BuildQueue>>;
    fn storage(&self) -> Result<Arc<Storage>>;
    fn pool(&self) -> Result<Pool>;
    fn metrics(&self) -> Result<Arc<Metrics>>;
    fn index(&self) -> Result<Arc<Index>>;
    fn repository_stats_updater(&self) -> Result<Arc<RepositoryStatsUpdater>>;
}

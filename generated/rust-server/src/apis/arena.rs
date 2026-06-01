use async_trait::async_trait;
use axum::extract::*;
use axum_extra::extract::CookieJar;
use bytes::Bytes;
use headers::Host;
use http::Method;
use serde::{Deserialize, Serialize};

use crate::{models, types::*};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetArenaStatsResponse {
    /// Aggregated arena stats snapshot
    Status200_AggregatedArenaStatsSnapshot
    (models::ArenaPlayerStats)
}




/// Arena
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Arena<E: std::fmt::Debug + Send + Sync + 'static = ()>: super::ErrorHandler<E> {
    /// Fetch aggregated arena stats for a player.
    ///
    /// GetArenaStats - GET /api/v1/arena/stats
    async fn get_arena_stats(
    &self,
    
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      query_params: &models::GetArenaStatsQueryParams,
    ) -> Result<GetArenaStatsResponse, E>;
}

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
pub enum GetPlayerProfileResponse {
    /// Arena profile summary
    Status200_ArenaProfileSummary(models::PlayerProfileSummary),
    /// Player profile not found
    Status404_PlayerProfileNotFound(models::ApiError),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum SearchPlayerResponse {
    /// Resolved player identity
    Status200_ResolvedPlayerIdentity(models::PlayerSearchResult),
    /// Invalid player search query
    Status400_InvalidPlayerSearchQuery(models::ApiError),
    /// Player not found
    Status404_PlayerNotFound(models::ApiError),
}

/// Players
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Players<E: std::fmt::Debug + Send + Sync + 'static = ()>: super::ErrorHandler<E> {
    /// Fetch Arena-focused profile summary for a player.
    ///
    /// GetPlayerProfile - GET /api/v1/players/{puuid}/profile
    async fn get_player_profile(
        &self,

        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        path_params: &models::GetPlayerProfilePathParams,
        query_params: &models::GetPlayerProfileQueryParams,
    ) -> Result<GetPlayerProfileResponse, E>;

    /// Resolve a player identity from Riot ID or summoner lookup.
    ///
    /// SearchPlayer - GET /api/v1/players/search
    async fn search_player(
        &self,

        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::SearchPlayerQueryParams,
    ) -> Result<SearchPlayerResponse, E>;
}

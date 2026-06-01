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
    /// Player profile with recent matches
    Status200_PlayerProfileWithRecentMatches
    (models::PlayerProfileResponse)
    ,
    /// Player not found
    Status404_PlayerNotFound
    (models::ErrorResponse)
    ,
    /// Upstream Riot/API failure
    Status502_UpstreamRiot
    (models::ErrorResponse1)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum SearchPlayersResponse {
    /// Search results
    Status200_SearchResults
    (models::PlayerSearchResponse)
    ,
    /// Upstream Riot/API failure
    Status502_UpstreamRiot
    (models::ErrorResponse)
}




/// Players
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Players<E: std::fmt::Debug + Send + Sync + 'static = ()>: super::ErrorHandler<E> {
    /// Get player profile and recent Arena matches.
    ///
    /// GetPlayerProfile - GET /lol/players/{puuid}
    async fn get_player_profile(
    &self,
    
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      path_params: &models::GetPlayerProfilePathParams,
    ) -> Result<GetPlayerProfileResponse, E>;

    /// Search players by Riot ID game name and tagline.
    ///
    /// SearchPlayers - GET /lol/players/search
    async fn search_players(
    &self,
    
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      query_params: &models::SearchPlayersQueryParams,
    ) -> Result<SearchPlayersResponse, E>;
}

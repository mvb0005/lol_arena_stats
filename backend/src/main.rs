use arena_openapi::{
    apis::{
        self,
        arena::{Arena, GetArenaStatsResponse},
        players::{GetPlayerProfileResponse, Players, SearchPlayerResponse},
        system::{GetHealthResponse, System},
    },
    models, server,
};
use async_trait::async_trait;
use std::net::SocketAddr;
use tracing::info;

#[derive(Clone, Debug, Default)]
struct Api;

const DEFAULT_REGION: &str = "americas";
const MOCK_LAST_UPDATED: &str = "2026-06-01T00:00:00Z";

#[derive(Clone, Debug)]
struct MockPlayerProfile {
    player_name: String,
    tag_line: String,
    region: String,
    puuid: String,
    recent_placements: Vec<u8>,
    total_games: u32,
    win_rate: f64,
}

impl AsRef<Api> for Api {
    fn as_ref(&self) -> &Api {
        self
    }
}

fn mock_profiles() -> Vec<MockPlayerProfile> {
    vec![
        MockPlayerProfile {
            player_name: "ArenaAce".to_owned(),
            tag_line: "NA1".to_owned(),
            region: "americas".to_owned(),
            puuid: "puuid-arenaace-na1".to_owned(),
            recent_placements: vec![1, 2, 3, 1, 4],
            total_games: 58,
            win_rate: 0.34,
        },
        MockPlayerProfile {
            player_name: "BladeDancer".to_owned(),
            tag_line: "EUW".to_owned(),
            region: "europe".to_owned(),
            puuid: "puuid-bladedancer-euw".to_owned(),
            recent_placements: vec![4, 2, 5, 3, 2],
            total_games: 41,
            win_rate: 0.19,
        },
    ]
}

fn normalized(value: &str) -> String {
    value.trim().to_ascii_lowercase()
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let app = server::new(Api);

    let address = SocketAddr::from(([127, 0, 0, 1], 3001));
    info!("starting backend on {}", address);

    let listener = tokio::net::TcpListener::bind(address)
        .await
        .expect("bind backend listener");

    axum::serve(listener, app)
        .await
        .expect("serve backend application");
}

#[async_trait]
impl apis::ErrorHandler<()> for Api {}

#[async_trait]
impl System<()> for Api {
    async fn get_health(
        &self,
        _method: &http::Method,
        _host: &headers::Host,
        _cookies: &axum_extra::extract::CookieJar,
    ) -> Result<GetHealthResponse, ()> {
        Ok(GetHealthResponse::Status200_ServiceHealth(
            models::HealthStatus::new(
                "lol-arena-stats-backend".to_owned(),
                "ok".to_owned(),
                env!("CARGO_PKG_VERSION").to_owned(),
            ),
        ))
    }
}

#[async_trait]
impl Arena<()> for Api {
    async fn get_arena_stats(
        &self,
        _method: &http::Method,
        _host: &headers::Host,
        _cookies: &axum_extra::extract::CookieJar,
        query_params: &models::GetArenaStatsQueryParams,
    ) -> Result<GetArenaStatsResponse, ()> {
        Ok(
            GetArenaStatsResponse::Status200_AggregatedArenaStatsSnapshot(
                models::ArenaPlayerStats::new(
                    3.2,
                    MOCK_LAST_UPDATED.to_owned(),
                    42,
                    query_params.player_name.clone(),
                    query_params
                        .region
                        .clone()
                        .unwrap_or_else(|| "americas".to_owned()),
                    query_params
                        .tag_line
                        .clone()
                        .unwrap_or_else(|| "NA1".to_owned()),
                    0.71,
                    0.22,
                ),
            ),
        )
    }
}

#[async_trait]
impl Players<()> for Api {
    async fn get_player_profile(
        &self,
        _method: &http::Method,
        _host: &headers::Host,
        _cookies: &axum_extra::extract::CookieJar,
        path_params: &models::GetPlayerProfilePathParams,
        query_params: &models::GetPlayerProfileQueryParams,
    ) -> Result<GetPlayerProfileResponse, ()> {
        let region = query_params
            .region
            .clone()
            .unwrap_or_else(|| DEFAULT_REGION.to_owned());
        let profile = mock_profiles().into_iter().find(|candidate| {
            normalized(&candidate.puuid) == normalized(&path_params.puuid)
                && normalized(&candidate.region) == normalized(&region)
        });

        Ok(match profile {
            Some(player) => GetPlayerProfileResponse::Status200_ArenaProfileSummary(
                models::PlayerProfileSummary::new(
                    player.player_name,
                    player.tag_line,
                    player.region,
                    player.puuid,
                    player.recent_placements,
                    player.total_games,
                    player.win_rate,
                    MOCK_LAST_UPDATED.to_owned(),
                ),
            ),
            None => GetPlayerProfileResponse::Status404_PlayerProfileNotFound(models::ApiError::new(
                "PLAYER_NOT_FOUND".to_owned(),
                "No profile found for the requested player.".to_owned(),
            )),
        })
    }

    async fn search_player(
        &self,
        _method: &http::Method,
        _host: &headers::Host,
        _cookies: &axum_extra::extract::CookieJar,
        query_params: &models::SearchPlayerQueryParams,
    ) -> Result<SearchPlayerResponse, ()> {
        let player_name = query_params.player_name.trim();
        if player_name.is_empty() {
            return Ok(SearchPlayerResponse::Status400_InvalidPlayerSearchQuery(
                models::ApiError::new(
                    "INVALID_QUERY".to_owned(),
                    "playerName must not be blank.".to_owned(),
                ),
            ));
        }

        let region = query_params
            .region
            .clone()
            .unwrap_or_else(|| DEFAULT_REGION.to_owned());
        let tag_line = query_params
            .tag_line
            .as_ref()
            .map(|value| value.trim())
            .filter(|value| !value.is_empty());
        let has_supplied_tag_line = query_params.tag_line.is_some();
        if has_supplied_tag_line && tag_line.is_none() {
            return Ok(SearchPlayerResponse::Status400_InvalidPlayerSearchQuery(
                models::ApiError::new(
                    "INVALID_QUERY".to_owned(),
                    "tagLine must not be blank when provided.".to_owned(),
                ),
            ));
        }

        let player = mock_profiles().into_iter().find(|candidate| {
            if let Some(tag) = tag_line {
                normalized(&candidate.player_name) == normalized(player_name)
                    && normalized(&candidate.tag_line) == normalized(tag)
            } else {
                normalized(&candidate.player_name) == normalized(player_name)
                    && normalized(&candidate.region) == normalized(&region)
            }
        });

        Ok(match player {
            Some(found) => SearchPlayerResponse::Status200_ResolvedPlayerIdentity(
                models::PlayerSearchResult::new(
                    found.player_name,
                    found.tag_line,
                    found.region,
                    found.puuid,
                    MOCK_LAST_UPDATED.to_owned(),
                ),
            ),
            None => SearchPlayerResponse::Status404_PlayerNotFound(models::ApiError::new(
                "PLAYER_NOT_FOUND".to_owned(),
                "No player matched the supplied search query.".to_owned(),
            )),
        })
    }
}

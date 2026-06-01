use arena_openapi::{
    apis::{
        self,
        arena::{Arena, GetArenaStatsResponse},
        players::{GetPlayerProfileResponse, Players, SearchPlayersResponse},
        system::{GetHealthResponse, System},
    },
    models, server,
};
use async_trait::async_trait;
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, RwLock},
};
use tracing::info;

const MAX_RECENT_MATCHES: usize = 10;

#[derive(Clone, Debug)]
struct Api {
    state: Arc<RwLock<AppState>>,
}

#[derive(Debug, Default)]
struct AppState {
    players_by_puuid: HashMap<String, StoredPlayer>,
    player_match_ids: HashMap<String, Vec<String>>,
    recent_matches: HashMap<String, models::RecentArenaMatchSummary>,
    active_games: HashMap<String, models::ActiveArenaSummary>,
    arena_rankings: HashMap<String, models::ArenaRankSummary>,
}

#[derive(Clone, Debug)]
struct StoredPlayer {
    puuid: String,
    game_name: String,
    tagline: String,
    summoner_id: String,
    summoner_level: u32,
    profile_icon_id: u32,
}

impl Api {
    fn new() -> Self {
        Self {
            state: Arc::new(RwLock::new(AppState::seeded())),
        }
    }

    fn search_results(&self, game_name: &str, tagline: &str) -> Vec<models::PlayerSearchResult> {
        let state = self.state.read().expect("app state lock poisoned");
        state
            .players_by_puuid
            .values()
            .find(|player| {
                player.game_name.eq_ignore_ascii_case(game_name)
                    && player.tagline.eq_ignore_ascii_case(tagline)
            })
            .map(|player| {
                vec![models::PlayerSearchResult::new(
                    player.puuid.clone(),
                    player.game_name.clone(),
                    player.tagline.clone(),
                )]
            })
            .unwrap_or_default()
    }

    fn player_profile_response(&self, puuid: &str) -> Option<models::PlayerProfileResponse> {
        let state = self.state.read().expect("app state lock poisoned");
        let player = state.players_by_puuid.get(puuid)?;

        let mut profile = models::PlayerProfile::new(
            player.puuid.clone(),
            player.game_name.clone(),
            player.tagline.clone(),
            player.summoner_id.clone(),
            player.summoner_level,
            player.profile_icon_id,
        );
        profile.active_arena = state.active_games.get(&player.puuid).cloned();
        profile.arena_rank = state.arena_rankings.get(&player.summoner_id).cloned();

        let recent_matches = state
            .player_match_ids
            .get(&player.puuid)
            .map(|match_ids| {
                match_ids
                    .iter()
                    .filter_map(|id| state.recent_matches.get(id))
                    .take(MAX_RECENT_MATCHES)
                    .cloned()
                    .collect()
            })
            .unwrap_or_default();

        Some(models::PlayerProfileResponse::new(profile, recent_matches))
    }
}

impl AppState {
    fn seeded() -> Self {
        let player = StoredPlayer {
            puuid: "puuid-somename-na1".to_owned(),
            game_name: "SomeName".to_owned(),
            tagline: "NA1".to_owned(),
            summoner_id: "summoner-somename".to_owned(),
            summoner_level: 123,
            profile_icon_id: 456,
        };

        let mut players_by_puuid = HashMap::new();
        players_by_puuid.insert(player.puuid.clone(), player.clone());

        let player_match_ids = HashMap::from([(
            player.puuid.clone(),
            vec![
                "NA1_10012".to_owned(),
                "NA1_10011".to_owned(),
                "NA1_10010".to_owned(),
                "NA1_10009".to_owned(),
                "NA1_10008".to_owned(),
                "NA1_10007".to_owned(),
                "NA1_10006".to_owned(),
                "NA1_10005".to_owned(),
                "NA1_10004".to_owned(),
                "NA1_10003".to_owned(),
                "NA1_10002".to_owned(),
                "NA1_10001".to_owned(),
                "NA1_missing".to_owned(),
            ],
        )]);

        let recent_matches = (1_u32..=12)
            .map(|idx| {
                let match_id = format!("NA1_{:05}", 10000 + idx);
                let participants = vec![
                    models::RecentArenaParticipantSummary::new(
                        player.puuid.clone(),
                        (idx % 8) + 1,
                        266 + idx,
                        8 + idx,
                        2 + (idx % 4),
                        6 + idx,
                    ),
                    models::RecentArenaParticipantSummary::new(
                        format!("ally-{idx}"),
                        2,
                        64,
                        5,
                        5,
                        10,
                    ),
                ];

                (
                    match_id.clone(),
                    models::RecentArenaMatchSummary::new(
                        match_id,
                        1700,
                        1_711_111_111_111_i64 + (idx as i64 * 1_000_i64),
                        1_711_111_222_222_i64 + (idx as i64 * 1_000_i64),
                        participants,
                    ),
                )
            })
            .collect();

        let active_games = HashMap::from([(
            player.puuid.clone(),
            models::ActiveArenaSummary::new(true, 1700, 1_711_111_111_111_i64),
        )]);

        let arena_rankings = HashMap::from([(
            player.summoner_id.clone(),
            models::ArenaRankSummary::new("GOLD".to_owned(), "II".to_owned(), 75),
        )]);

        Self {
            players_by_puuid,
            player_match_ids,
            recent_matches,
            active_games,
            arena_rankings,
        }
    }
}

impl AsRef<Api> for Api {
    fn as_ref(&self) -> &Api {
        self
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let app = server::new(Api::new());

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
                    "2026-06-01T00:00:00Z".to_owned(),
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
    ) -> Result<GetPlayerProfileResponse, ()> {
        let response = match self.player_profile_response(&path_params.puuid) {
            Some(profile) => {
                GetPlayerProfileResponse::Status200_PlayerProfileWithRecentMatches(profile)
            }
            None => GetPlayerProfileResponse::Status404_PlayerNotFound(models::ErrorResponse::new(
                "player not found".to_owned(),
            )),
        };
        Ok(response)
    }

    async fn search_players(
        &self,
        _method: &http::Method,
        _host: &headers::Host,
        _cookies: &axum_extra::extract::CookieJar,
        query_params: &models::SearchPlayersQueryParams,
    ) -> Result<SearchPlayersResponse, ()> {
        let results = self.search_results(&query_params.game_name, &query_params.tagline);
        Ok(SearchPlayersResponse::Status200_SearchResults(
            models::PlayerSearchResponse::new(results),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn host() -> headers::Host {
        headers::Host::from(http::uri::Authority::from_static("localhost"))
    }

    #[tokio::test]
    async fn search_players_returns_single_result_when_found() {
        let api = Api::new();
        let response = api
            .search_players(
                &http::Method::GET,
                &host(),
                &axum_extra::extract::CookieJar::new(),
                &models::SearchPlayersQueryParams {
                    game_name: "SomeName".to_owned(),
                    tagline: "NA1".to_owned(),
                },
            )
            .await
            .expect("search_players should succeed");

        match response {
            SearchPlayersResponse::Status200_SearchResults(body) => {
                assert_eq!(body.results.len(), 1);
                assert_eq!(body.results[0].game_name, "SomeName");
                assert_eq!(body.results[0].tagline, "NA1");
            }
            other => panic!("unexpected response: {other:?}"),
        }
    }

    #[tokio::test]
    async fn search_players_returns_empty_when_not_found() {
        let api = Api::new();
        let response = api
            .search_players(
                &http::Method::GET,
                &host(),
                &axum_extra::extract::CookieJar::new(),
                &models::SearchPlayersQueryParams {
                    game_name: "Unknown".to_owned(),
                    tagline: "NA1".to_owned(),
                },
            )
            .await
            .expect("search_players should succeed");

        match response {
            SearchPlayersResponse::Status200_SearchResults(body) => {
                assert!(body.results.is_empty());
            }
            other => panic!("unexpected response: {other:?}"),
        }
    }

    #[tokio::test]
    async fn get_player_profile_enriches_and_limits_recent_matches() {
        let api = Api::new();
        let response = api
            .get_player_profile(
                &http::Method::GET,
                &host(),
                &axum_extra::extract::CookieJar::new(),
                &models::GetPlayerProfilePathParams {
                    puuid: "puuid-somename-na1".to_owned(),
                },
            )
            .await
            .expect("get_player_profile should succeed");

        match response {
            GetPlayerProfileResponse::Status200_PlayerProfileWithRecentMatches(body) => {
                assert_eq!(body.player.game_name, "SomeName");
                assert!(body.player.arena_rank.is_some());
                assert!(body.player.active_arena.is_some());
                assert_eq!(body.recent_matches.len(), MAX_RECENT_MATCHES);
                assert!(
                    body.recent_matches
                        .iter()
                        .all(|m| m.match_id != "NA1_missing")
                );
            }
            other => panic!("unexpected response: {other:?}"),
        }
    }

    #[tokio::test]
    async fn get_player_profile_returns_not_found_for_unknown_puuid() {
        let api = Api::new();
        let response = api
            .get_player_profile(
                &http::Method::GET,
                &host(),
                &axum_extra::extract::CookieJar::new(),
                &models::GetPlayerProfilePathParams {
                    puuid: "does-not-exist".to_owned(),
                },
            )
            .await
            .expect("get_player_profile should succeed");

        match response {
            GetPlayerProfileResponse::Status404_PlayerNotFound(body) => {
                assert_eq!(body.message, "player not found");
            }
            other => panic!("unexpected response: {other:?}"),
        }
    }
}

use arena_openapi::{
    apis::{
        self,
        arena::{Arena, GetArenaLeaderboardResponse, GetArenaStatsResponse},
        system::{GetHealthResponse, System},
    },
    models, server,
};
use async_trait::async_trait;
use std::net::SocketAddr;
use tracing::info;

#[derive(Clone, Debug, Default)]
struct Api;

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
    async fn get_arena_leaderboard(
        &self,
        _method: &http::Method,
        _host: &headers::Host,
        _cookies: &axum_extra::extract::CookieJar,
        query_params: &models::GetArenaLeaderboardQueryParams,
    ) -> Result<GetArenaLeaderboardResponse, ()> {
        let region = query_params
            .region
            .clone()
            .unwrap_or_else(|| "americas".to_owned());
        let sort_by = query_params
            .sort_by
            .clone()
            .unwrap_or_else(|| "winRate".to_owned());
        let sort_order = query_params
            .sort_order
            .clone()
            .unwrap_or_else(|| "desc".to_owned());
        let page = query_params.page.unwrap_or(1);
        let page_size = u32::from(query_params.page_size.unwrap_or(20));

        let mut leaderboard: Vec<LeaderboardRecord> = leaderboard_records()
            .into_iter()
            .filter(|entry| entry.region.eq_ignore_ascii_case(&region))
            .collect();

        let sort_metric = SortMetric::from_query(&sort_by);
        let sort_direction = SortDirection::from_query(&sort_order);

        leaderboard.sort_by(|a, b| {
            let metric_order = match sort_metric {
                SortMetric::WinRate => a.win_rate.total_cmp(&b.win_rate),
                SortMetric::TopFourRate => a.top_four_rate.total_cmp(&b.top_four_rate),
                SortMetric::AveragePlacement => a.average_placement.total_cmp(&b.average_placement),
                SortMetric::MatchesPlayed => a.matches_played.cmp(&b.matches_played),
            };

            let metric_order = match sort_direction {
                SortDirection::Asc => metric_order,
                SortDirection::Desc => metric_order.reverse(),
            };

            metric_order
                .then_with(|| a.player_name.cmp(b.player_name))
                .then_with(|| a.tag_line.cmp(b.tag_line))
        });

        let ranked_entries: Vec<models::ArenaLeaderboardEntry> = leaderboard
            .iter()
            .enumerate()
            .map(|(index, player)| {
                models::ArenaLeaderboardEntry::new(
                    index as u32 + 1,
                    player.average_placement,
                    player.last_updated.to_owned(),
                    player.matches_played,
                    player.player_name.to_owned(),
                    player.region.to_owned(),
                    player.tag_line.to_owned(),
                    player.top_four_rate,
                    player.win_rate,
                )
            })
            .collect();

        let total_items = ranked_entries.len() as u32;
        let total_pages = if total_items == 0 {
            0
        } else {
            total_items.div_ceil(page_size)
        };
        let start = ((page.saturating_sub(1)) * page_size) as usize;
        let paged_entries = if start >= ranked_entries.len() {
            Vec::new()
        } else {
            let end = (start + page_size as usize).min(ranked_entries.len());
            ranked_entries[start..end].to_vec()
        };

        Ok(
            GetArenaLeaderboardResponse::Status200_RankedAndPaginatedArenaLeaderboard(
                models::ArenaLeaderboardResponse::new(
                    paged_entries,
                    page,
                    page_size,
                    sort_metric.as_api_value().to_owned(),
                    sort_direction.as_api_value().to_owned(),
                    total_items,
                    total_pages,
                ),
            ),
        )
    }

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

#[derive(Clone, Debug)]
struct LeaderboardRecord {
    player_name: &'static str,
    tag_line: &'static str,
    region: &'static str,
    matches_played: u32,
    average_placement: f64,
    top_four_rate: f64,
    win_rate: f64,
    last_updated: &'static str,
}

#[derive(Copy, Clone, Debug)]
enum SortMetric {
    WinRate,
    TopFourRate,
    AveragePlacement,
    MatchesPlayed,
}

impl SortMetric {
    fn from_query(value: &str) -> Self {
        match value {
            "topFourRate" => Self::TopFourRate,
            "averagePlacement" => Self::AveragePlacement,
            "matchesPlayed" => Self::MatchesPlayed,
            _ => Self::WinRate,
        }
    }

    fn as_api_value(self) -> &'static str {
        match self {
            Self::WinRate => "winRate",
            Self::TopFourRate => "topFourRate",
            Self::AveragePlacement => "averagePlacement",
            Self::MatchesPlayed => "matchesPlayed",
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum SortDirection {
    Asc,
    Desc,
}

impl SortDirection {
    fn from_query(value: &str) -> Self {
        match value {
            "asc" => Self::Asc,
            _ => Self::Desc,
        }
    }

    fn as_api_value(self) -> &'static str {
        match self {
            Self::Asc => "asc",
            Self::Desc => "desc",
        }
    }
}

fn leaderboard_records() -> Vec<LeaderboardRecord> {
    vec![
        LeaderboardRecord {
            player_name: "PhoenixDuo",
            tag_line: "NA1",
            region: "americas",
            matches_played: 89,
            average_placement: 2.4,
            top_four_rate: 0.82,
            win_rate: 0.34,
            last_updated: "2026-06-01T00:00:00Z",
        },
        LeaderboardRecord {
            player_name: "StormWarden",
            tag_line: "EUW",
            region: "europe",
            matches_played: 94,
            average_placement: 2.1,
            top_four_rate: 0.85,
            win_rate: 0.37,
            last_updated: "2026-06-01T00:00:00Z",
        },
        LeaderboardRecord {
            player_name: "BladeEcho",
            tag_line: "NA2",
            region: "americas",
            matches_played: 76,
            average_placement: 2.9,
            top_four_rate: 0.79,
            win_rate: 0.28,
            last_updated: "2026-06-01T00:00:00Z",
        },
        LeaderboardRecord {
            player_name: "SkyLancer",
            tag_line: "KR1",
            region: "asia",
            matches_played: 102,
            average_placement: 2.0,
            top_four_rate: 0.87,
            win_rate: 0.39,
            last_updated: "2026-06-01T00:00:00Z",
        },
        LeaderboardRecord {
            player_name: "RuneCrafter",
            tag_line: "BR1",
            region: "americas",
            matches_played: 68,
            average_placement: 3.3,
            top_four_rate: 0.72,
            win_rate: 0.23,
            last_updated: "2026-06-01T00:00:00Z",
        },
        LeaderboardRecord {
            player_name: "AegisTwin",
            tag_line: "NA3",
            region: "americas",
            matches_played: 120,
            average_placement: 2.2,
            top_four_rate: 0.84,
            win_rate: 0.36,
            last_updated: "2026-06-01T00:00:00Z",
        },
    ]
}

mod contract;

use axum::{extract::Query, routing::get, Json, Router};
use serde::Deserialize;
use std::net::SocketAddr;
use tracing::info;

use crate::contract::{ArenaPlayerStats, HealthStatus};

#[derive(Debug, Deserialize)]
struct ArenaStatsQuery {
    #[serde(rename = "playerName")]
    player_name: String,
    #[serde(rename = "tagLine")]
    tag_line: Option<String>,
    region: Option<String>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let app = Router::new()
        .route("/health", get(health))
        .route("/api/v1/arena/stats", get(arena_stats));

    let address = SocketAddr::from(([127, 0, 0, 1], 3001));
    info!("starting backend on {}", address);

    let listener = tokio::net::TcpListener::bind(address)
        .await
        .expect("bind backend listener");

    axum::serve(listener, app)
        .await
        .expect("serve backend application");
}

async fn health() -> Json<HealthStatus> {
    Json(HealthStatus {
        service: "lol-arena-stats-backend".to_owned(),
        status: "ok".to_owned(),
        version: env!("CARGO_PKG_VERSION").to_owned(),
    })
}

async fn arena_stats(Query(query): Query<ArenaStatsQuery>) -> Json<ArenaPlayerStats> {
    Json(ArenaPlayerStats {
        average_placement: 3.2,
        last_updated: "2026-06-01T00:00:00Z".to_owned(),
        matches_played: 42,
        player_name: query.player_name,
        region: query.region.unwrap_or_else(|| "americas".to_owned()),
        tag_line: query.tag_line.unwrap_or_else(|| "NA1".to_owned()),
        top_four_rate: 0.71,
        win_rate: 0.22,
    })
}

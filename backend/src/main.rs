use arena_openapi::{
    apis::{
        self,
        arena::{Arena, GetArenaStatsResponse},
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

pub mod arena_stats {
    typify::import_types!(schema = "../schemas/arena-player-stats.json");
}

pub mod health {
    typify::import_types!(schema = "../schemas/health-status.json");
}

pub use arena_stats::ArenaPlayerStats;
pub use health::HealthStatus;

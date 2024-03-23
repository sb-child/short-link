use std::net::SocketAddr;

use axum::{
    routing::{get, post},
    Router,
};

use short_link::{
    config::{Cfg, LogLevel},
    database::{new_connection, DatabaseOptions},
    routes,
};

async fn main_func() -> anyhow::Result<()> {
    let cfg_file = "ShortLink.toml";
    println!("Reading config file `{}` ...", cfg_file);
    let cfg_file = tokio::fs::read_to_string(cfg_file).await?;
    let cfg = toml::from_str::<Cfg>(&cfg_file)?;

    tracing_subscriber::fmt()
        .with_max_level(match cfg.log_level {
            LogLevel::Trace => tracing::Level::TRACE,
            LogLevel::Debug => tracing::Level::DEBUG,
            LogLevel::Info => tracing::Level::INFO,
            LogLevel::Warn => tracing::Level::WARN,
            LogLevel::Error => tracing::Level::ERROR,
        })
        .init();

    println!("Starting ...");

    let db = new_connection(DatabaseOptions {
        url: cfg.database_url,
    })
    .await?;

    let app = Router::new()
        .nest(
            &cfg.base,
            Router::new()
                .route("/_/:id", get(routes::short_link::index))
                .route("/_update/:token/:id", post(routes::short_link::update))
                .route("/_challenge", get(routes::challenge::create))
                .route("/_challenge/:payload/:hash", get(routes::challenge::verify))
                .route(
                    "/_challenge_revoke/:nonce/:token",
                    get(routes::challenge::revoke),
                ),
        )
        .with_state((db, cfg.service));

    let addr = SocketAddr::new(cfg.host.parse()?, cfg.port);
    println!("listening on {addr} ...");
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

fn main() -> anyhow::Result<()> {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async { main_func().await })
}
use axum::http::{self, Method, StatusCode};
use axum::routing::{get, post};
use axum::Router;
use sea_orm::Database;
use std::vec;
use std::{error::Error, net::SocketAddr, sync::Arc};
use tower_http::cors::{Any, CorsLayer};
use tracing::info;

use app::controllers::{auth_controller, entry_controller};
use app::models::AppState;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().expect("Error reading environment variables.");

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_test_writer()
        .init();

    let db_url = dotenvy::var("DATABASE_URL").expect("No 'DATABASE_URL' var set.");
    let port: u16 = dotenvy::var("PORT")
        .expect("No 'PORT' var set.")
        .parse()
        .expect("Invalid 'PORT' var set.");
    dotenvy::var("TOKEN_KEY").expect("No 'TOKEN_KEY' var set.");

    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    let db = Database::connect(db_url).await.expect(
        "Unable to connect to database. Check your 'DATABASE_URL' var and ensure the database is running.",
    );

    let state = Arc::new(AppState::new(db));

    let auth_router = Router::new()
        .route("/authorize", post(auth_controller::authorize))
        .route("/register", post(auth_controller::register))
        .route("/validate_token", get(auth_controller::validate_token));

    let entry_router = Router::new().route("/", get(entry_controller::get_entries));

    let app = Router::new()
        .nest("/auth", auth_router)
        .nest("/entry", entry_router)
        .with_state(state)
        .fallback(|| async { (StatusCode::NOT_FOUND, "Resource was not found.") })
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(vec![Method::POST, Method::GET, Method::PUT, Method::DELETE])
                .allow_headers(vec![
                    http::header::CONTENT_TYPE,
                    http::header::AUTHORIZATION,
                ]),
        );

    info!("Listening on port {port}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

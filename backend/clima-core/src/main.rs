use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};

use axum::extract::State;

use sqlx::SqlitePool;

async fn get_assortment(State(db): State<SqlitePool>) -> impl IntoResponse {
    match sqlx::query_file!("queries/get_assortment.sql")
        .fetch_all(&db)
        .await
    {
        Err(e) => {
            tracing::error!(
                "Error while querying database during assortment getting. Error = [{e:?}]"
            );

            (StatusCode::INTERNAL_SERVER_ERROR).into_response()
        }
        Ok(result) => {
            let options: Vec<String> = result.into_iter().map(|record| record.name).collect();

            (
                StatusCode::OK,
                Json(serde_json::json!({ "assortment": options })),
            )
                .into_response()
        }
    }
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();

    let database_pool = sqlx::sqlite::SqlitePool::connect(
        "sqlite:/home/fuchczyk/Programming/clima/clima-core/clima.sqlite",
    )
    .await
    .unwrap();

    let main_router = axum::Router::new()
        .route("/assortment", get(get_assortment))
        .with_state(database_pool)
        .layer(tower_http::cors::CorsLayer::very_permissive());

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 39632));

    axum::Server::bind(&addr)
        .serve(main_router.into_make_service())
        .await
        .unwrap();
}

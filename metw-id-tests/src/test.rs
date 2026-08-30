use crate::TestId;
use axum::{
    Json, Router,
    extract::{Path, Request},
    http::{Method, header::CONTENT_TYPE},
    routing::get,
};
use http_body_util::BodyExt;
use tower::ServiceExt;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(info(description = "test api"), paths(ping))]
#[allow(dead_code)]
struct ApiDoc;

/// Returns back sent id.
#[utoipa::path(
    get, path = "/{id}",
    params(("id" = TestId, Path)),
    responses(
        (status = OK, body = TestId)
    )
)]
async fn ping(Path(test_id): Path<TestId>) -> Json<TestId> {
    Json(test_id)
}

fn app() -> Router {
    Router::new().route("/{id}", get(ping))
}

#[test]
fn ordering() {
    for _ in 0..1023 {
        let first = TestId::unique();
        let second = TestId::unique();

        assert!(first < second);
    }
}

#[tokio::test]
async fn axum() {
    let id = TestId::unique();

    let request = Request::builder()
        .method(Method::GET)
        .uri(format!("/{}", id))
        .header(CONTENT_TYPE, "application/json")
        .body(axum::body::Body::empty())
        .unwrap();

    let router = app();

    let response = router.oneshot(request).await.unwrap();

    let body = response.into_body();

    let api_response: TestId =
        serde_json::from_slice(&body.collect().await.unwrap().to_bytes()).unwrap();

    assert_eq!(api_response, id);
}

#[tokio::test]
#[ignore]
async fn postgres() {
    dotenvy::dotenv().ok();

    let id = TestId::unique();

    let datbase_url = std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable");

    let pool = sqlx::PgPool::connect(&datbase_url).await.unwrap();

    let postgres_response = sqlx::query_scalar::<_, TestId>("SELECT $1 AS \"id: TestId\"")
        .bind(id)
        .fetch_one(&pool)
        .await
        .unwrap();

    assert_eq!(postgres_response, id)
}

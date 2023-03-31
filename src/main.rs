use std::error::Error;
use std::sync::Arc;
use std::time::Instant;

use axum::extract::Extension;
use axum::http::StatusCode;
use axum::response::{Response, Html};
use axum::Json;
use axum::{response::IntoResponse, routing::get, Router, Server};
use color_eyre::Report;
use serde::Serialize;
use tokio::sync::Mutex;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing::info;

pub mod tracing_stuff;
mod youtube;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    color_eyre::install()?;
    tracing_stuff::setup()?;

    // Do some work here.
    run_server().await?;

    tracing_stuff::teardown();

    Ok(())
}

#[axum::debug_handler]
async fn index_html() -> impl IntoResponse {
    let markup = tokio::fs::read_to_string("index/index.html").await.unwrap();

    Html(markup)
}


#[axum::debug_handler]
async fn index_mjs() -> impl IntoResponse {
    let markup = tokio::fs::read_to_string("index/index.mjs").await.unwrap();

    Response::builder()
        .header("content-type", "application/javascript;charset=utf-8")
        .body(markup)
        .unwrap()
}

#[axum::debug_handler]
async fn index_css() -> impl IntoResponse {
    let markup = tokio::fs::read_to_string("index/index.css").await.unwrap();

    Response::builder()
        .header("content-type", "text/css;charset=utf-8")
        .body(markup)
        .unwrap()
}

async fn run_server() -> Result<(), Box<dyn Error>> {
    let addr = "0.0.0.0:3779".parse()?;
    info!("Listening on http://{}", addr);

    let app = Router::new()
    // route / for get the index.html
        .route("/", get(index_html))
        // route /index.mjs for get the index.mjs
        .route("/index.mjs", get(index_mjs))
        // route /index.css for get the index.css
        .route("/index.css", get(index_css))
        // .route("/", get(root))
        .layer(
            ServiceBuilder::new().layer(TraceLayer::new_for_http()), // .into_inner(), // into_inner 可能影响性能
        )
        // .共用一个 request 对象
        .layer(Extension(reqwest::Client::new()))
        .layer(Extension(CachedLastedVideo::default()));

    Server::bind(&addr).serve(app.into_make_service()).await?;

    Ok(())
}

// 定义一个缓存结构体
#[derive(Clone, Default)]
struct CachedLastedVideo {
    // arc 是多个线程访问的引用计数
    // mutex 是多个线程访问的互斥锁
    // option 是实际的value 的可选数据（可以为None）
    // 有效的数据为一个元组（时间，当前的值）
    value: Arc<Mutex<Option<(Instant, String)>>>,
}

#[tracing::instrument(skip(client, cached))]
async fn root(
    client: Extension<reqwest::Client>,
    cached: Extension<CachedLastedVideo>,
) -> Result<impl IntoResponse, ReportError> {
    // client 对象从 extension 中提取出来
    #[derive(Serialize)]
    struct Response {
        video_id: String,
    }

    // 正常情况 返回一个string， 错误情况返回一个错误码和错误信息
    // 对于返回错误的情况 可以进行封装
    // // Ok(res) 每次直接调接口进行刷新获取数据
    // Ok(Json(Response {
    //     video_id: youtube::fetch_video_id(&client).await?,
    // }))

    // 使用缓存获取数据

    let mut cached_value = cached.value.lock().await;
    {
        if let Some((cached_at, video_id)) = cached_value.as_ref() {
            if cached_at.elapsed() < std::time::Duration::from_secs(5) {
                return Ok(Json(Response {
                    video_id: video_id.clone(),
                }));
            } else {
                dbg!("cache expired, let's refresh");
            }
        } else {
            dbg!("cache empty, let's refresh");
        }
    }

    let video_id = youtube::fetch_video_id(&client).await?;
    // mutex 是同步的， 不能希望在等待时间保持锁的状态，不然可能会死锁
    // So, we lock it, check if there's a value we can use - if not, we start a fetch, and then lock it again to store it in the cache.
    cached_value.replace((Instant::now(), video_id.clone()));

    Ok(Json(Response { video_id }))
}

struct ReportError(Report);

// 实现 From<Report> trait
impl From<Report> for ReportError {
    fn from(err: Report) -> Self {
        Self(err)
    }
}

// 实现IntoResponse trait
impl IntoResponse for ReportError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Internal Server Error: {:?}", self.0),
        )
            .into_response()
    }
}

use std::{io, net::TcpListener};
use actix_web::{
    get,
    http::{self},
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};
use mobc_redis::redis::{AsyncCommands, RedisError};
use postgres::{build_dataase_pool, Link};
use redis::{connect, RedisPool};
use sqlx::{query, PgPool, Postgres};
use tokio::select;

mod postgres;
mod redis;

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("io error: {0}")]
    IoError(#[from] io::Error),

    #[error("redis error: {0}")]
    RedisError(#[from] RedisError),

    #[error("redis error: {0}")]
    MobcRedis(#[from] mobc::Error<RedisError>),
}

#[derive(Clone)]
struct AppState {
    redis_pool: RedisPool,
    postgres_pool: PgPool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let redis_pool = connect("redis://localhost:6379").unwrap();
    let postgres_pool = build_dataase_pool().await.unwrap();
    let app_state = AppState {
        redis_pool,
        postgres_pool,
    };
    let listener = TcpListener::bind("127.0.0.1:9090").unwrap();
    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .service(health)
            .service(redirect)
    })
    .listen(listener)
    .unwrap()
    .run();

    select! {
        _ = tokio::signal::ctrl_c() => {
            println!("Received Ctrl+C, shutting down...");
        }
        _ = tokio::spawn(server) => {
            println!("Server stopped");
        }
    }

    Ok(())
}

async fn retrieve_from_database(
    postgres: &PgPool,
    id: &str,
) -> Result<Option<Link>> {
    Ok(sqlx::query_as::<Postgres, Link>(
        "select * from links where short_id = $1",
    )
    .bind(id)
    .fetch_optional(postgres)
    .await
    .unwrap())
}

async fn retrieve_link(
    id: &str,
    redis: &RedisPool,
    postgres: &PgPool,
) -> Result<Option<String>> {
    let mut conn = redis.get().await.unwrap();

    if let Some(url) = conn.get(id).await.unwrap() {
        return Ok(Some(url));
    }

    if let Some(link) = retrieve_from_database(postgres, id).await.unwrap() {
        let _: () = conn
            .set_ex(&link.short_id, &link.original_url, 10)
            .await
            .unwrap();
        return Ok(Some(link.original_url.clone()));
    }

    Ok(None)
}

#[get("/l/{id}")]
async fn redirect(
    id: web::Path<String>,
    state: Data<AppState>,
) -> impl Responder {
    let id = id.into_inner();
    match retrieve_link(&id, &state.redis_pool, &state.postgres_pool).await {
        Ok(Some(link)) => HttpResponse::Found()
            .append_header((http::header::LOCATION, link.to_string()))
            .finish(),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(e) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/health")]
async fn health() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[cfg(test)]
mod tests {
    use mobc_redis::redis::{Client, Commands};

    #[tokio::test]
    async fn test_redis() {
        let mut conn = Client::open("redis://localhost:6379").unwrap();
        let x: Option<String> = conn.get("key").unwrap();
    }
}

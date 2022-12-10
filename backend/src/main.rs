use actix_web::{get, web/*, guard*/, App, HttpServer, Responder};
// use chrono::Utc;
// use aof::{BlogEntry, Author};
use sqlx::PgPool;
use sqlx::types::JsonValue;

#[macro_export]
macro_rules! sql_json {
    ( $pool: expr, $structure: path, $file: expr, $( $params:expr ),* ) => {
        {
            web::Json(
                sqlx::query_file_as!($structure, $file, $($params),* )
                    .fetch_one($pool.as_ref())
                    .await
                    .expect(&web::Json(r#"{"error": "sql error"}"#)))
        }
    };
}

#[derive(Clone,Debug,Eq,PartialEq)]
struct Mocs {
    jsonb_build_object: Option<JsonValue>
}

#[get("/blog/{id}")]
async fn blog_entry(pool: web::Data<PgPool>, path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();
    let r = sqlx::query_file_as!(Mocs, "sql/blog_entry2.psql", id )
                    .fetch_one(pool.as_ref())
                    .await
                    .expect(&web::Json(r#"{"error": "sql error"}"#));
    web::Json(r.jsonb_build_object)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = PgPool::connect("postgresql://artoffeeling@localhost/aof")
        .await
        .expect("Failed creation of database pool.");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(
                web::scope("/api")
                .service(blog_entry)
            )
            .service(web::resource("/").to(|| async { "Hello world!" }))
    })
    .bind(("0.0.0.0", 1221))?
    .run()
    .await
}

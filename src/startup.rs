use crate::routes::{health_check, subscribe};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::{Pool, Postgres};
use std::net::TcpListener;

// main precisa ser async pq .run volta um Future :
// como em lib não é uma entrada de binário, pode colocar como async sem ter que usar proc-macro
// pub fn run(self) -> Server etc
// impl Future for Server etc
pub fn run(listener: TcpListener, pool: Pool<Postgres>) -> Result<Server, std::io::Error> {
    // Um wrap de ARC ao redor de connection, já que PgConnection não é Cloneável, mas ARC sempre é
    let db_pool = web::Data::new(pool);
    let server = HttpServer::new(move || {
        // Fluent API : chaining methods
        // App é onde a lógica da aplicação vive, routing, middlewares, request handlers etc
        App::new()
            // route pede path e um handler (Route) . O route pode tb ter guards , Ex:
            // web::resource("/ guarded").route(
            //     web::route()
            //         .guard(guard::Any(guard::Get()).or(guard::Post()))
            //         .guard(guard::Header("x-guarded", "secret"))
            //         .to(|| HttpResponse::Ok())
            // );
            // .route("/", web::get().to(greet))
            // .route("/{name}", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}

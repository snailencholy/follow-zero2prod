//! src/lib.rs

pub mod configuration;
pub mod routes;
pub mod startup;

// use actix_web::dev::Server;

// use actix_web::{web, App, HttpResponse, HttpServer};
// use std::net::TcpListener;

// #[derive(serde::Deserialize)]
// pub struct FormData {
//     email: String,
//     name: String,
// }

// async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
//     HttpResponse::Ok().finish()
// }

// pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
//     let server = HttpServer::new(|| {
//         App::new()
//             .route("/health_check", web::get().to(health_check))
//             // A new entry in our routing table for POST /subscriptions requests
//             .route("/subscriptions", web::post().to(subscribe))
//     })
//     //.bind(address)?
//     .listen(listener)?
//     .run();
//     // No .await
//     Ok(server)
// }

use actix::prelude::*;
use actix_web::{get, post, web, HttpResponse, Responder};

use crate::db::DbExecutor;
use crate::models::Status;

pub struct AppState {
    pub db: Addr<DbExecutor>,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn status(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(Status {
        status: format!("{:?}", &data.db.connected()),
    })
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

pub fn configure_app(app_cfg: &mut web::ServiceConfig, db: Addr<DbExecutor>) {
    app_cfg
        .data(AppState { db })
        .service(hello)
        .service(echo)
        .route("/hey", web::get().to(manual_hello))
        .route("/status", web::get().to(status));
}

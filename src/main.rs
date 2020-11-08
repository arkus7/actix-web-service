mod app;
mod config;
mod db;
mod models;

use crate::app::configure_app;
use crate::db::DbExecutor;
use actix::{Addr, SyncArbiter};
use actix_web::{App, HttpServer};
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config = config::Config::from_env().unwrap();

    // let _sys = actix::System::new("AuthService");

    let manager = ConnectionManager::<PgConnection>::new(format!(
        "postgres://{}:{}@{}:{}/{}",
        config.db.user, config.db.password, config.db.host, config.db.port, config.db.database_name
    ));
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    let db_address: Addr<DbExecutor> = SyncArbiter::start(4, move || DbExecutor(pool.clone()));

    HttpServer::new(move || App::new().configure(|cfg| configure_app(cfg, db_address.clone())))
        .bind(format!("{}:{}", config.server.host, config.server.port))?
        .run()
        .await
}

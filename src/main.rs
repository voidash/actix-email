use std::net::TcpListener;
use sqlx::PgPool;
use z2p::startup::run;
use z2p::configuration::{get_configuration, self};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("failed to read configuration file");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
                     .await
                     .expect("failed to connect to postgres");

    let listener = TcpListener::bind("127.0.0.1:8000").expect("failed to bind address");
    println!("{:?}", listener);
    run(listener,connection_pool)?.await
}

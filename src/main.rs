use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use union_back::create_app;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //let config = config::load_config()?;
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO) // Postavite maksimalnu razinu logiranja
        .init();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        //.connect(&config.database_url)
        .connect("postgres://spl:spl@localhost/lora_db")
        .await?;

    let app = create_app(pool);

    //let addr = SocketAddr::from(([127, 0, 0, 1], config.port));
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    let listener = TcpListener::bind(addr).await?;

    println!("🚀 Server running on http://{}", addr);
    axum::serve(listener, app).await?;

    Ok(())
}

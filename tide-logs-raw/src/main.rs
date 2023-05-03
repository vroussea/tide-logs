#[async_std::main]
async fn main() -> tide::Result<()> {
    dotenvy::dotenv()?;
    pretty_env_logger::init();

    let app = tide_app::app();
    app.listen("127.0.0.1:8080").await?;

    Ok(())
}

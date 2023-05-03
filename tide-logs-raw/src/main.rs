use tracing_subscriber::{layer::SubscriberExt, registry::Registry};
use tracing_tree::HierarchicalLayer;

#[async_std::main]
async fn main() -> tide::Result<()> {
    dotenvy::dotenv()?;

    let subscriber = Registry::default().with(HierarchicalLayer::new(2));
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let app = tide_app::app();
    app.listen("127.0.0.1:8080").await?;

    Ok(())
}

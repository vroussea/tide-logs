use tide::prelude::*;
use tide::Request;

#[derive(Debug, Deserialize)]
struct Body {
    something: Option<String>,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    pretty_env_logger::init();

    let mut app = tide::new();

    app.with(tide::log::LogMiddleware::new());
    app.at("/").get(|_| async { Ok("Hello, world!") });
    app.at("/helloworld").post(hello_world);
    app.listen("127.0.0.1:8080").await?;

    Ok(())
}

async fn hello_world(mut req: Request<()>) -> tide::Result {
    let Body { something } = req.body_json().await?;
    if let Some(text) = something {
        Ok(format!("Hello World ! {}\n", text).into())
    } else {
        Ok("Hello World !\n".into())
    }
}

use tide::{convert::Deserialize, Request};

#[derive(Debug, Deserialize)]
struct Body {
    something: Option<String>,
}

#[shuttle_runtime::main]
async fn tide() -> shuttle_tide::ShuttleTide<()> {
    let mut app = tide::new();

    app.with(tide::log::LogMiddleware::new());
    app.at("/").get(|_| async { Ok("Hello, world!") });
    app.at("/helloworld").post(hello_world);

    Ok(app.into())
}

async fn hello_world(mut req: Request<()>) -> tide::Result {
    let Body { something } = req.body_json().await?;
    if let Some(text) = something {
        Ok(format!("Hello World ! {}\n", text).into())
    } else {
        Ok("Hello World !\n".into())
    }
}

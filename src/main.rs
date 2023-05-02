use tide::Request;
use tide::prelude::*;

#[derive(Debug, Deserialize)]
struct Body {
    something: Option<String>,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/helloworld").post(hello_world);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

async fn hello_world(mut req: Request<()>) -> tide::Result {
    let Body { something } = req.body_json().await?;
    if let Some(text) = something {
        Ok(format!("Hello World ! {}\n", text).into())
    }
    else {
        Ok(format!("Hello World !\n").into())
    }
}
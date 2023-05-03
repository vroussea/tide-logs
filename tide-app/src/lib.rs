use tide_tracing::TraceMiddleware;

use tide::{
    convert::Deserialize,
    log::{debug, error, info, trace, warn},
    Request, Server,
};

#[derive(Debug, Deserialize)]
struct Body {
    some_string: Option<String>,
    number: Option<u64>,
}

pub fn app() -> Server<()> {
    let mut app = tide::new();

    app.with(TraceMiddleware::new());
    app.at("/").get(root);
    app.at("/post").post(post_handler);

    app
}

async fn root(req: Request<()>) -> tide::Result {
    debug!("got request: {req:?}");

    let headers = req
        .header_names()
        .zip(req.header_values())
        .map(|(name, value)| format!("{}: {}", name, value))
        .collect::<Vec<_>>();

    info!("got headers: {headers:?}");
    Ok("Hello World!".into())
}

async fn post_handler(mut req: Request<()>) -> tide::Result {
    trace!("started reading data");

    let body: Body = req.body_json().await.map_err(|e| {
        error!("failed to read body: {e}");
        e
    })?;

    trace!("finished reading data");
    debug!("got {body:?}");

    if let Some(number) = body.number {
        info!("got number: {number}");
    } else {
        warn!("got no number");
    }

    if let Some(text) = body.some_string {
        info!("got text: {text}");
        Ok(format!("Hello World ! {}\n", text).into())
    } else {
        warn!("got no text");
        Ok("Hello World !\n".into())
    }
}

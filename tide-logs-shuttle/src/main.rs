#[shuttle_runtime::main]
async fn tide() -> shuttle_tide::ShuttleTide<()> {
    let app = tide_app::app();

    Ok(app.into())
}

use eyre::Result;
use tinkc::{Tink, TinkCert, TinkConfigBuilder};
use tinql::{schema, Context};
use warp::{http::Response, Filter};

#[tokio::main]
async fn main() -> Result<()> {
    std::env::set_var("RUST_LOG", "warp_async");
    env_logger::init();

    let log = warp::log("warp_server");

    let homepage = warp::path::end().map(|| {
        Response::builder()
            .header("content-type", "text/html")
            .body(
                "<html><h1>juniper_warp</h1><div>visit <a href=\"/graphiql\">/graphiql</a></html>",
            )
    });

    log::info!("Listening on 127.0.0.1:8080");

    let tink = Tink::new(
        TinkConfigBuilder::default()
            .endpoint("http://[::1]:42113")
            .cert(TinkCert::File("tinkc/examples/data/tls/ca.pem"))
            .domain("localhost")
            .build()?,
    )
    .await?;
    let context = Context::new(tink);

    let state = warp::any().map(move || context.clone());
    let graphql_filter = juniper_warp::make_graphql_filter(schema(), state.boxed());
    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec![
            "Accept",
            "content-type",
            "User-Agent",
            "Sec-Fetch-Mode",
            "Referer",
            "Origin",
            "Access-Control-Request-Method",
            "Access-Control-Request-Headers",
        ])
        .allow_methods(vec!["GET", "POST", "DELETE", "OPTIONS"]);

    warp::serve(
        warp::get()
            .and(warp::path("graphiql"))
            .and(juniper_warp::graphiql_filter("/graphql", None))
            .or(homepage)
            .or(warp::path("graphql").and(graphql_filter))
            .with(log)
            .with(cors),
    )
    .run(([127, 0, 0, 1], 8080))
    .await;

    Ok(())
}

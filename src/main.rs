mod components;
mod views;

use actix_web::{get, web, web::ServiceConfig, HttpResponse};
use dioxus::prelude::*;
use shuttle_actix_web::ShuttleActixWeb;
use std::path::PathBuf;
use views::decode_tx_view::{DecodeTxView, PageProps};
use views::main_view::MainView;

#[get("/")]
async fn app_endpoint() -> HttpResponse {
    let mut view = VirtualDom::new(MainView);
    let _ = view.rebuild();

    HttpResponse::Ok()
        .content_type("text/html")
        .body(dioxus_ssr::render(&view).to_string())
}

#[get("/decode/{tx_hash}")]
async fn decode_tx(path: web::Path<String>) -> HttpResponse {
    let tx_hash = path.into_inner();

    let mut view = VirtualDom::new_with_props(DecodeTxView, PageProps { tx_hash });
    let _ = view.rebuild();

    HttpResponse::Ok()
        .content_type("text/html")
        .body(dioxus_ssr::render(&view).to_string())
}

#[shuttle_runtime::main]
async fn actix_web(
    #[shuttle_static_folder::StaticFolder(folder = "public")] public_folder: PathBuf,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(decode_tx)
            .service(app_endpoint)
            .service(actix_files::Files::new("/public", public_folder));
    };
    Ok(config.into())
}

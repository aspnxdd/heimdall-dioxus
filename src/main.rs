use actix_web::{get, web, web::ServiceConfig, HttpResponse};
use dioxus::prelude::*;
use heimdall::decode::decode_calldata;
use shuttle_actix_web::ShuttleActixWeb;

const CALLDATA: &str = "0xd57eafac000000000000000000000000b59419389d1fb089135a6a2c4bb32e5e8aa8b3330000000000000000000000001b84765de8b7566e4ceaf4d0fd3c5af52d3dde4f000000000000000000000000000000000000000000000f41a839dee4932bd176000000000000000000000000000000000000000000000004afd16002efcae82f0000000000000000000000001116898dda4015ed8ddefb84b6e8bc24528af2d80000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000101c599f9f0000000000000000000000000000000000000000000000000000000063eaa06d8514775c2c1663f55720b8c4291fbb33e6524316ebc6919a2d9a459811072867";

#[derive(PartialEq, Props)]
pub struct PageProps {
    tx_hash: String,
}

#[get("/decode/{tx_hash}")]
async fn hello_world(path: web::Path<String>) -> HttpResponse {
    fn app(cx: Scope<PageProps>) -> Element {
        let c = decode_calldata(&CALLDATA);
        println!("c: {:?}", c);
        let c = c.unwrap();
        let first_item = c.get(0).unwrap();
        let name: String = first_item.name.clone();
        let signature = first_item.signature.clone();
        let decoded_inputs = first_item.decoded_inputs.clone().unwrap();
        let inputs = first_item.inputs.clone();

        cx.render(rsx!(
            head {
                meta {
                    name: "viewport",
                    content: "width=device-width, initial-scale=1.0"
                }
                meta {
                    name: "description",
                    content: "Heimdall Dioxus"
                }
                link {
                    rel: "stylesheet",
                    href: "/public/style.css"
                }
            }
            body {
                main{
                    style: "display: flex; flex-direction: column; align-items: center; justify-content: center; height: 100vh;",
                    h1 {
                        "{cx.props.tx_hash}"
                    }
                    div {
                        name,
                    }
                    div {
                        signature,
                    }
                    div {
                        for input in inputs {
                            div {
                                input,
                            }
                        }
                    }
                    div {
                        for decoded_input in decoded_inputs {
                            div {
                                decoded_input.to_string(),
                            }
                        }
                    }
                }
            }
        ))
    }
    let tx_hash = path.into_inner();
    println!("tx_hash: {}", tx_hash);

    // create a VirtualDom with the app component
    let mut app = VirtualDom::new_with_props(app, PageProps { tx_hash });

    // rebuild the VirtualDom before rendering
    let _ = app.rebuild();

    // render the VirtualDom to HTML
    HttpResponse::Ok()
        .content_type("text/html")
        .body(dioxus_ssr::render(&app).to_string())
}

#[get("/")]
async fn app_endpoint() -> HttpResponse {
    // create a component that renders a div with the text "hello world"
    fn app(cx: Scope) -> Element {
        cx.render(rsx!(
                head {
                    meta {
                        name: "viewport",
                        content: "width=device-width, initial-scale=1.0"
                    }
                    meta {
                        name: "description",
                        content: "Heimdall Dioxus"
                    }
                    link {
                        rel: "stylesheet",
                        href: "/public/style.css"
                    }
                }
                body {
                    main{
                        style: "display: flex; flex-direction: column; align-items: center; justify-content: center; height: 100vh;",
                        input {
                            class: "text",
                            placeholder: "a",
                            id:"input"
                        },
                        button {
                            class: "button",
                            id: "button",
                            "Click me!"
                        }
                    }
                    script {
                        "
                        const input = document.getElementById(`input`);
                        const button = document.getElementById(`button`);
                        button.onclick = function() {{
                            location.replace(`/decode/${{input.value}}`);
                        }}
                    "
                    }
            }
        ))
    }
    // create a VirtualDom with the app component
    let mut app = VirtualDom::new(app);
    // rebuild the VirtualDom before rendering
    let _ = app.rebuild();

    // render the VirtualDom to HTML
    HttpResponse::Ok()
        .content_type("text/html")
        .body(dioxus_ssr::render(&app).to_string())
}

#[shuttle_runtime::main]
async fn actix_web() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(hello_world)
            .service(app_endpoint)
            .service(actix_files::Files::new("/public", "./public"));
    };
    Ok(config.into())
}

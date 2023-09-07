use dioxus::prelude::*;

use crate::components::footer::Footer;

#[derive(Props)]
pub struct PageProps<'a> {
    #[props(into)]
    children: Element<'a>,
}

#[allow(non_snake_case)]
pub fn Layout<'a>(cx: Scope<'a, PageProps<'a>>) -> Element {
    cx.render(rsx!(
        head {
            title {
                "Heimdall Dioxus"
            }
            link {
                rel: "icon",
                href: "/public/favicon.ico"
            }
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
            &cx.props.children,
            Footer {}
        }
    ))
}

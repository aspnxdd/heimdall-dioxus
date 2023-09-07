use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn Footer(cx: Scope) -> Element {
    render!(
        footer {
            p {
                "Heimdall Dioxus - Made by "
                a {
                    href: "https://github.com/aspnxdd?tab=repositories",
                    target: "_blank",
                    "aspnxdd"
                }
            }
            p {
                strong {
                    "Powered by "
                    a {
                        href: "https://github.com/Jon-Becker/heimdall-rs",
                        target: "_blank",
                        "Heimdall"
                    }
                }
            }
        }
    )
}

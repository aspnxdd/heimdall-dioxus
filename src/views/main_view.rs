use crate::components::layout::Layout;

use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn Main_view(cx: Scope) -> Element {
    cx.render(rsx!(
        Layout {
            main{
                h1 {
                    "Heimdall Dioxus"
                }
                h4 {
                    "Decode Ethereum transactions"
                }
                form {
                    id: "form",
                    input {
                        placeholder: "Tx hash...",
                        id:"input"
                    },
                    button {
                        id: "button",
                        disabled: true,
                        "Decode Transaction"
                    }
                }
            }
            script {
                "
                    const input = document.getElementById(`input`);
                    const form = document.getElementById(`form`);
                    const button = document.getElementById(`button`);
                    input.addEventListener(`input`, function() {{
                        if (input.value.length === 0) {{
                            button.disabled = true;
                        }} else {{
                            button.disabled = false;
                        }}
                    }});
                    form.onsubmit = function(e) {{
                        e.preventDefault();
                        const inputValue = input.value;
                        input.value = null;
                        location.pathname = `/decode/${{inputValue}}`;
                    }}
                    
                "
        }
    }))
}

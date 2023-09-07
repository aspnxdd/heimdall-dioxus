use std::panic;

use crate::components::layout::Layout;
use dioxus::prelude::*;
use heimdall::decode::decode_calldata;

// const CALLDATA: &str = "0xd57eafac000000000000000000000000b59419389d1fb089135a6a2c4bb32e5e8aa8b3330000000000000000000000001b84765de8b7566e4ceaf4d0fd3c5af52d3dde4f000000000000000000000000000000000000000000000f41a839dee4932bd176000000000000000000000000000000000000000000000004afd16002efcae82f0000000000000000000000001116898dda4015ed8ddefb84b6e8bc24528af2d80000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000101c599f9f0000000000000000000000000000000000000000000000000000000063eaa06d8514775c2c1663f55720b8c4291fbb33e6524316ebc6919a2d9a459811072867";

#[derive(PartialEq, Props)]
pub struct PageProps {
    pub tx_hash: String,
}

#[allow(non_snake_case)]
fn Error_view(cx: Scope) -> Element {
    cx.render(rsx!(
        Layout {
            main{
                h1 {
                    "Error decoding transaction"
                }
            }
        }
    ))
}

#[allow(non_snake_case)]
pub fn Decode_tx_view(cx: Scope<PageProps>) -> Element {
    println!("Rendering Decode_tx_view {}", cx.props.tx_hash);
    let result = panic::catch_unwind(|| decode_calldata(&cx.props.tx_hash));
    if result.is_err() {
        return cx.render(rsx!(Error_view {}));
    }
    match result.unwrap() {
        Some(decoded_calldata) => {
            let tx_hash_short = format!(
                "{}...{}",
                &cx.props.tx_hash[..6],
                &cx.props.tx_hash[&cx.props.tx_hash.len() - 6..]
            );
            let first_item = decoded_calldata.get(0).unwrap();
            let name = first_item.name.clone();
            // let signature = first_item.signature.clone();
            let decoded_inputs = first_item.decoded_inputs.clone().unwrap();
            let inputs = first_item.inputs.clone();
            let inputsWithTypes = inputs
                .iter()
                .zip(decoded_inputs.iter())
                .map(|(input, decoded_input)| format!("{}: {}", input, decoded_input.to_string()))
                .collect::<Vec<String>>();

            cx.render(rsx!(
                Layout {
                    main{
                        table {
                            thead {
                                tr{
                                    th {
                                        "Transaction Hash:"
                                    }
                                    th {
                                        "Function Name:"
                                    }

                                    th {
                                        "Inputs:"
                                    }
                                }
                            }
                            tbody {
                                tr {
                                    td {
                                        tx_hash_short
                                    }
                                    td {
                                        name,
                                    }

                                    td {
                                        for input in inputsWithTypes {
                                            div {
                                                input,
                                            }
                                        }
                                    }
                                }
                            }
                        }

                    }
                }
            ))
        }
        None => {
            return cx.render(rsx!(Error_view {}));
        }
    }
}

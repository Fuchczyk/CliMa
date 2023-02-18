#![allow(non_snake_case)]
use dioxus::html::*;
use dioxus::prelude::*;
use dioxus_router::{Link, Route, Router};

mod call;
mod client;
mod utils;

use call::CallPage;
use client::ClientPage;

#[derive(Clone, Copy)]
enum ActiveSite {
    MainPage,
    Client,
    Call,
    Cargo,
}

#[inline_props]
fn Navbar(cx: Scope) -> Element {
    let bar_options = [
        ("Strona główna", "/"),
        ("Klient", "/client"),
        ("Rozmowa", "/call"),
        ("Towar", "/cargo"),
    ];

    cx.render(rsx! {
        nav { class: "flex items-center bg-navigation",
            div { class: "flex items-center p-2",
                img { src: "https://www.svgrepo.com/show/411125/call.svg", class: "h-11" }
                span { class: "self-center whitespace-nowrap pl-1 text-2xl font-bold text-white",
                    "CliMa"
                }
            }
            div { class: "ml-10 flex grow flex-row items-center justify-end space-x-9 pr-5 text-white",
                bar_options.clone().into_iter().map(|(name, href)| {rsx!{
                    Link {
                        to: "{href}",
                        class: "hover:text-button hover:underline",
                        "{name}"
                    }
                }})
            }
        }
    })
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        Router { 
            Navbar {}
            div { class: "min-h-screen bg-background flex flex-row",
                Route { to: "/" }
                Route { to: "/client", ClientPage {} }
                Route { to: "/call", CallPage {} }
            }
        }
    })
}

fn main() {
    dioxus_web::launch(App)
}

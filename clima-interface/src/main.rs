mod client;
mod utils;

use sycamore::prelude::*;
use sycamore_router::{HistoryIntegration, Route, Router, RouterProps};

#[derive(Route)]
enum MainComponents {
    #[to("/")]
    MainPage,
    #[to("/client")]
    Client,
    #[not_found]
    NotFound,
}

#[component]
fn Navbar<G: Html>(cx: Scope) -> View<G> {
    let bar_options = [
        ("Strona główna", "/"),
        ("Klient", "/client"),
        ("Rozmowa", "/call"),
        ("Towar", "/cargo"),
    ];

    let bar_options_views = View::new_fragment(
        bar_options
            .into_iter()
            .map(|(name, reference)| {
                view! { cx,
                    a (
                        href=reference,
                        class="hover:text-button hover:underline"
                    ) {
                        (name)
                    }
                }
            })
            .collect(),
    );

    view! { cx,
        nav (
            class="flex items-center bg-navigation"
        ) {
            div (
                class="flex items-center p-2"
            ) {
                img (
                    src="https://www.svgrepo.com/show/411125/call.svg",
                    class="h-11"
                )
                span (
                        class="self-center whitespace-nowrap pl-1 text-2xl font-bold text-white"
                ) {
                    "CliMa"
                }
            }
            div (
                class="ml-10 flex grow flex-row items-center justify-end space-x-9 pr-5 text-white"
            ) {
                (bar_options_views)
            }
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    sycamore::render(|cx| {
        view! { cx,
            Router (
                integration=HistoryIntegration::new(),
                view=|cx, route: &ReadSignal<MainComponents>| {
                    view! { cx,
                        Navbar()
                        div (
                            class="min-h-screen bg-background flex flex-row"
                        ) {
                            (
                                match route.get().as_ref() {
                                    MainComponents::Client => {
                                        view! { cx, client::ClientPage() }
                                    }
                                    _ => {
                                        view! { cx, "Not found"}
                                    }
                                }
                            )
                        }
                    }
                }
            )
        }
    });
}

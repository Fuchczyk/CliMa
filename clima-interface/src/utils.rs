use dioxus::prelude::*;
use dioxus_router::Link;

pub use gloo_net::http::Response;
pub use gloo_net::Error as RequestError;

#[derive(Props)]
pub struct MinimalMenuProps<'a> {
    image: &'a str,
    options: Vec<(&'static str, &'static str)>,
}

pub fn MinimalMenu<'a>(cx: Scope<'a, MinimalMenuProps<'a>>) -> Element {
    cx.render(rsx! {
        div { class: "text-background flex border-spacing-2 flex-col items-center pt-5 pl-5",
            div { class: "bg-navigation rounded-md",
                div { class: "stroke-picture fill-picture", dangerous_inner_html: "{cx.props.image}" }
                cx.props.options.clone().into_iter().map(|(name, reference)| rsx! {
                div {
                    class: "hover:border-green-theme hover:text-green-theme border-background mx-4 my-4 rounded border text-center",
                    Link {
                    to: "{reference}",
                    class: "m-5",
                    "{name}"
                }
                }
            })
            }
        }
    })
}

pub async fn get<T: AsRef<str>>(url: T) -> Result<Response, RequestError> {
    gloo_net::http::Request::get(url.as_ref()).send().await
}

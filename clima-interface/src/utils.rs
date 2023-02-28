use sycamore::prelude::*;

#[derive(Prop)]
pub struct MinimalMenuProps<'a> {
    image_svg: &'static str,
    options: &'a [(&'static str, &'static str)],
}

#[component]
pub fn MinimalMenu<'a, 'b, G: Html>(cx: Scope<'a>, props: MinimalMenuProps<'b>) -> View<G> {
    let options = View::new_fragment(
        props
            .options
            .iter()
            .map(|(name, reference)| {
                let reference = reference.clone();
                let name = name.clone();

                view! { cx,
                    div (
                        class="hover:border-green-theme hover:text-green-theme border-background mx-4 my-4 rounded border text-center"
                    ) {
                        a (
                            href=(reference),
                            class="m-5"
                        ) {
                            (name)
                        }
                    }
                }
            })
            .collect(),
    );

    view! { cx,
        div (
            class="text-background flex border-spacing-2 flex-col items-center pt-5 pl-5",
        ) {
            div (
                class="bg-navigation rounded-md"
            ) {
                div (class="stroke-picture fill-picture", dangerously_set_inner_html=props.image_svg)
                (options)
            }
        }
    }
}

pub async fn get<T: AsRef<str>>(url: T) -> Result<gloo_net::http::Response, gloo_net::Error> {
    gloo_net::http::Request::get(url.as_ref()).send().await
}

use std::collections::HashMap;
use std::collections::HashSet;

use clima_types::ClientInfo;
use sycamore::futures::*;
use sycamore::prelude::*;
use sycamore::suspense::Suspense;

const ASSORTMENT_GET: &str = "http://127.0.0.1:39632/assortment";

#[derive(Prop)]
struct DisplayErrorProps {
    info: String,
}

#[component]
fn DisplayError<G: Html>(cx: Scope, props: DisplayErrorProps) -> View<G> {
    view! { cx,
        (props.info)
    }
}

#[derive(Prop)]
struct AssortmentMenuProps<'a> {
    initial_values: &'a HashSet<String>,
    checkbox_values: &'a Vec<(String, &'a Signal<bool>)>,
}

#[component]
fn AssortmentMenu<'a, G: Html>(cx: Scope<'a>, props: AssortmentMenuProps<'a>) -> View<G> {
    let menu_elements = View::new_fragment(
        props.checkbox_values.iter()
        .map(|(name, state)| {
            let name = std::rc::Rc::new(name.to_owned());
            let name1 = name.clone();
            let name2 = name.clone();
            let name3 = name.clone();

            view! { cx, 
                div (
                    class="col-span-1 mb-4 flex items-center"
                ) {
                    input (
                        id=name,
                        name=name1,
                        r#type="checkbox",
                        class="peer h-4 w-4 rounded border-background bg-gray-100",
                        bind:checked=state
                    ) {}
                    label (
                        r#for=name2,
                        class="peer-checked:text-yellow-theme ml-2 text-sm font-medium text-background peer-checked:font-semibold"
                    ) { (name3) }
                }
            }
        }).collect()
    );

    view! { cx,
        div (
            class="grid grid-cols-3"
        ) {
            (menu_elements)
        }
    }
}
/*
#[component]
async fn AssortmentMenuX<'a, G: Html>(cx: Scope<'a>) -> View<G> {
    let mut assortment_mapping = Vec::new();

    match crate::utils::get(ASSORTMENT_GET).await {
        Err(e) => {
            let error_message = format!("Error while getting assortment menu. Error = [{e:?}]");
            log::error!("{error_message}");

            view! { cx,
                DisplayError(
                    info=error_message
                )
            }
        }
        Ok(response) => match response.json::<Vec<String>>().await {
            Ok(assortment) => {
                log::debug!("Deserialized assortment.");
                let menu_elements = View::new_fragment(
                    assortment
                        .into_iter()
                        .map(|item| {
                            let element_signal = create_rc_signal(props.initial_values.contains(&item));
                            assortment_mapping.push((item.clone(), element_signal.clone()));

                            // FIXME: Boilerplate until
                            // https://github.com/rust-lang/rfcs/issues/2407 resolved.
                            let item_rc = std::rc::Rc::new(item);
                            let item_rc2 = item_rc.clone();
                            let item_rc3 = item_rc.clone();
                            let item_rc4 = item_rc.clone();

                            view!( cx,
                                div (
                                    class="col-span-1 mb-4 flex items-center"
                                ) {
                                    input (
                                        id=item_rc,
                                        name=item_rc2,
                                        r#type="checkbox",
                                        class="peer h-4 w-4 rounded border-background bg-gray-100",
                                        bind:checked=element_signal.clone()
                                    ) {}
                                    label (
                                        r#for=item_rc3,
                                        class="peer-checked:text-yellow-theme ml-2 text-sm font-medium text-background peer-checked:font-semibold"
                                    ) { (item_rc4) }
                                }
                            )
                        })
                        .collect(),
                );

                props.checkbox_values.set(assortment_mapping);

                view! { cx,
                    div (
                        class="grid grid-cols-3"
                    ) {
                        (menu_elements)
                    }
                }
            }
            Err(e) => {
                let error = format!("Error while deserializing assortment. Error = [{e:?}]");
                log::error!("{error}");

                view! { cx,
                    DisplayError(
                        info=error
                    )
                }
            }
        },
    }
} */

enum ClientFormError {}

#[derive(Prop)]
struct ClientFormProps<T, F>
where
    T: Fn(ClientInfo) -> F + Clone,
    F: std::future::Future<Output = Result<(), ClientFormError>>,
{
    initial_values: ClientInfo,
    callback: T,
}

enum ClientFormStatus {
    Ready,
    AwaitingResponse,
    Error(ClientFormError),
    Success,
}

#[component]
fn ClientForm<
    'a,
    G: Html,
    T: Fn(ClientInfo) -> F + Clone + 'a,
    F: std::future::Future<Output = Result<(), ClientFormError>>,
>(
    cx: Scope<'a>,
    props: ClientFormProps<T, F>,
) -> View<G> {
    let name = create_signal(cx, String::new());
    let email = create_signal(cx, String::new());
    let postcode = create_signal(cx, String::new());
    let city = create_signal(cx, String::new());
    let phone = create_signal(cx, String::new());
    let form_status = create_signal(cx, ClientFormStatus::Ready);

    let button_click = || {
        let mut assortment_option = HashSet::new();

        for assort in assortment
            .get()
            .iter()
            .filter(|(_, status): &&(String, RcSignal<bool>)| *status.get())
        {
            assortment_option.insert(assort.0.clone());
        }

        let client_info = ClientInfo::new(
            name.get().as_ref().to_owned(),
            email.get().as_ref().to_owned(),
            postcode.get().as_ref().to_owned(),
            city.get().as_ref().to_owned(),
            phone.get().as_ref().to_owned(),
            assortment_option,
        );

        form_status.set(ClientFormStatus::AwaitingResponse);

        let callback_clone = props.callback.clone();
        spawn_local_scoped(cx, async move {
            match (callback_clone)(client_info).await {
                Ok(_) => form_status.set(ClientFormStatus::Success),
                Err(e) => {
                    form_status.set(ClientFormStatus::Error(e));
                }
            }
        });
    };

    log::debug!("Form rendering");

    view! { cx,
        form (
            class="border-8 border-double border-background bg-navigation p-6"
        ) {
            div (
                class="group relative z-0 mb-6 w-full"
            ) {
                input (
                    type="text",
                    name="name",
                    id="name",
                    class="peer block w-full appearance-none border-0 border-b-2 border-background bg-transparent py-2.5 px-0 text-sm text-white focus:border-picture focus:outline-none focus:ring-0",
                    bind:value=name,
                    placeholder=" ",
                    required=true
                ) {}
                label (
                    for="name",
                    class="absolute top-3 -z-10 origin-[0] -translate-y-6 scale-75 transform text-sm text-background duration-300 peer-placeholder-shown:translate-y-0 peer-placeholder-shown:scale-100 peer-focus:left-0 peer-focus:-translate-y-6 peer-focus:scale-75 peer-focus:font-medium peer-focus:text-picture"
                ) {
                    "Nazwa klienta"
                }
            }

            div (
                class="group relative z-0 mb-6 w-full"
            ) {
                input (
                    type="text",
                    name="email",
                    id="email",
                    class="peer block w-full appearance-none border-0 border-b-2 border-background bg-transparent py-2.5 px-0 text-sm text-white focus:border-picture focus:outline-none focus:ring-0",
                    bind:value=email,
                    placeholder=" ",
                    required=true
                ) {}
                label (
                    for="email",
                    class="absolute top-3 -z-10 origin-[0] -translate-y-6 scale-75 transform text-sm text-background duration-300 peer-placeholder-shown:translate-y-0 peer-placeholder-shown:scale-100 peer-focus:left-0 peer-focus:-translate-y-6 peer-focus:scale-75 peer-focus:font-medium peer-focus:text-picture"
                ) {
                    "E-mail"
                }
            }

            div (
                class="grid md:grid-cols-3 md:gap-6"
            ) {
                div (
                    class="group relative z-0 mb-6 w-full"
                ) {
                    input (
                        type="text",
                        name="postcode",
                        id="postcode",
                        class="peer block w-full appearance-none border-0 border-b-2 border-background bg-transparent py-2.5 px-0 text-sm text-white focus:border-picture focus:outline-none focus:ring-0",
                        bind:value=postcode,
                        placeholder=" ",
                        required=true
                    ) {}
                    label (
                        for="postcode",
                        class="absolute top-3 -z-10 origin-[0] -translate-y-6 scale-75 transform text-sm text-background duration-300 peer-placeholder-shown:translate-y-0 peer-placeholder-shown:scale-100 peer-focus:left-0 peer-focus:-translate-y-6 peer-focus:scale-75 peer-focus:font-medium peer-focus:text-picture"
                    ) {
                        "Kod pocztowy"
                    }
                }

                div (
                    class="col-span-2 group relative z-0 mb-6 w-full"
                ) {
                    input (
                        type="text",
                        name="city",
                        id="city",
                        class="peer block w-full appearance-none border-0 border-b-2 border-background bg-transparent py-2.5 px-0 text-sm text-white focus:border-picture focus:outline-none focus:ring-0",
                        bind:value=city,
                        placeholder=" ",
                        required=true
                    ) {}
                    label (
                        for="city",
                        class="absolute top-3 -z-10 origin-[0] -translate-y-6 scale-75 transform text-sm text-background duration-300 peer-placeholder-shown:translate-y-0 peer-placeholder-shown:scale-100 peer-focus:left-0 peer-focus:-translate-y-6 peer-focus:scale-75 peer-focus:font-medium peer-focus:text-picture"
                    ) {
                        "Miasto"
                    }
                }
            }

            div (
                class="group relative z-0 mb-6 w-full"
            ) {
                input (
                    type="text",
                    name="phone",
                    id="phone",
                    class="peer block w-full appearance-none border-0 border-b-2 border-background bg-transparent py-2.5 px-0 text-sm text-white focus:border-picture focus:outline-none focus:ring-0",
                    bind:value=phone,
                    placeholder=" ",
                    required=true
                ) {}
                label (
                    for="phone",
                    class="absolute top-3 -z-10 origin-[0] -translate-y-6 scale-75 transform text-sm text-background duration-300 peer-placeholder-shown:translate-y-0 peer-placeholder-shown:scale-100 peer-focus:left-0 peer-focus:-translate-y-6 peer-focus:scale-75 peer-focus:font-medium peer-focus:text-picture"
                ) {
                    "Telefon"
                }
            }

        }
    }
}

#[component]
pub fn ClientPage<G: Html>(cx: Scope) -> View<G> {
    let client_options = [
        ("Informacje ogólne", "/client"),
        ("Wyszukaj klienta", "/client/search"),
        ("Dodaj klienta", "/client/add"),
    ];

    let clientinfo = ClientInfo::default();

    let handler = |xa| async move {
        log::debug!("Handler called. With = {xa:?}");
        Ok(())
    };

    view! { cx,
        crate::utils::MinimalMenu (
            options=client_options.as_slice(),
            image_svg=include_str!("../assets/client_menu_image.svg")
        )
        div (
            class="m-15 flex grow items-center justify-center"
        ) {
            ClientForm (
                initial_values=clientinfo,
                callback=handler
            )
        }
    }
}

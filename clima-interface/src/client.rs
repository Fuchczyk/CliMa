use std::{collections::HashSet, future::Future};

use dioxus::prelude::*;

use crate::utils;
use clima_types::ClientInfo;

const ASSORTMENT_GET: &str = "/assortment";

enum ClientFormError {}

enum SubmitStatus {
    Pending,
    Success,
    Error(ClientFormError),
}

#[derive(Props)]
struct ClientFormProps<F, T: Fn(ClientInfo) -> F + Clone>
where
    F: Future<Output = Result<(), ClientFormError>>,
    T: 'static,
{
    initial_state: ClientInfo,
    submit_handler: T,
}

fn ClientForm<
    'a,
    F: Future<Output = Result<(), ClientFormError>> + 'static,
    T: Fn(ClientInfo) -> F + Clone + 'static,
>(
    cx: Scope<'a, ClientFormProps<F, T>>,
) -> Element {
    let submit_action = cx.props.submit_handler.clone();
    let submit_status = use_state(cx, || None::<SubmitStatus>);
    let form_state = use_state(cx, || cx.props.initial_state.clone());

    let assortment: &UseFuture<Vec<String>> = use_future(cx, (), |_| async move {
        loop {
            match crate::utils::get(ASSORTMENT_GET).await {
                Ok(assortment) => {
                    match assortment.json().await {
                        Err(e) => {
                            //TODO: Report error.
                        }
                        Ok(value) => return value,
                    }
                }
                Err(e) => {
                    //TODO: Report error.
                }
            }
        }
    });

    let button_click = move || {
        let status = submit_status.to_owned();
        let form_state = form_state.get().clone();

        cx.spawn(async move {
            match submit_action(form_state).await {
                Ok(_) => status.set(Some(SubmitStatus::Success)),
                Err(e) => status.set(Some(SubmitStatus::Error(e))),
            }
        });
    };

    cx.render(rsx! {
        form { class: "border-8 border-double border-background bg-navigation p-6",
            div { class: "group relative z-0 mb-6 w-full",
                input {
                    "type": "text",
                    name: "name",
                    id: "name",
                    class: "peer block w-full appearance-none border-0 border-b-2 border-background bg-transparent py-2.5 px-0 text-sm text-white focus:border-picture focus:outline-none focus:ring-0",
                    placeholder: " ",
                    value: form_state.name(),
                    required: true
                }
                label {
                    "for": "name",
                    class: "absolute top-3 -z-10 origin-[0] -translate-y-6 scale-75 transform text-sm text-background duration-300 peer-placeholder-shown:translate-y-0 peer-placeholder-shown:scale-100 peer-focus:left-0 peer-focus:-translate-y-6 peer-focus:scale-75 peer-focus:font-medium peer-focus:text-picture",
                    "Nazwa klienta"
                }
            }
            div { class: "group relative z-0 mb-6 w-full",
                input {
                    "type": "email",
                    name: "email",
                    id: "email",
                    class: "peer block w-full appearance-none border-0 border-b-2 border-background bg-transparent py-2.5 px-0 text-sm text-white focus:border-picture focus:outline-none focus:ring-0",
                    placeholder: " ",
                    value: form_state.email(),
                    required: true
                }
                label {
                    "for": "email",
                    class: "absolute top-3 -z-10 origin-[0] -translate-y-6 scale-75 transform text-sm text-background duration-300 peer-placeholder-shown:translate-y-0 peer-placeholder-shown:scale-100 peer-focus:left-0 peer-focus:-translate-y-6 peer-focus:scale-75 peer-focus:font-medium peer-focus:text-picture",
                    "E-mail"
                }
            }
            div { class: "grid md:grid-cols-3 md:gap-6",
                div { class: "group relative z-0 mb-6 w-full",
                    input {
                        "type": "text",
                        name: "postcode",
                        id: "postcode",
                        class: "peer block w-full appearance-none border-0 border-b-2 border-background bg-transparent py-2.5 px-0 text-sm text-white focus:border-picture focus:outline-none focus:ring-0",
                        placeholder: " ",
                        value: form_state.postcode(),
                        required: true
                    }
                    label {
                        "for": "postcode",
                        class: "absolute top-3 -z-10 origin-[0] -translate-y-6 scale-75 transform text-sm text-background duration-300 peer-placeholder-shown:translate-y-0 peer-placeholder-shown:scale-100 peer-focus:left-0 peer-focus:-translate-y-6 peer-focus:scale-75 peer-focus:font-medium peer-focus:text-picture",
                        "Kod pocztowy"
                    }
                }
                div { class: "group relative col-span-2 z-0 mb-6 w-full",
                    input {
                        "type": "text",
                        name: "city",
                        id: "city",
                        class: "peer block w-full appearance-none border-0 border-b-2 border-background bg-transparent py-2.5 px-0 text-sm text-white focus:border-picture focus:outline-none focus:ring-0",
                        placeholder: " ",
                        value: form_state.city(),
                        required: true
                    }
                    label {
                        "for": "city",
                        class: "absolute top-3 -z-10 origin-[0] -translate-y-6 scale-75 transform text-sm text-background duration-300 peer-placeholder-shown:translate-y-0 peer-placeholder-shown:scale-100 peer-focus:left-0 peer-focus:-translate-y-6 peer-focus:scale-75 peer-focus:font-medium peer-focus:text-picture",
                        "Miasto"
                    }
                }
            }
            div { class: "group relative z-0 mb-6 w-full",
                input {
                    "type": "text",
                    name: "phone",
                    id: "phone",
                    class: "peer block w-full appearance-none border-0 border-b-2 border-background bg-transparent py-2.5 px-0 text-sm text-white focus:border-picture focus:outline-none focus:ring-0",
                    placeholder: " ",
                    value: form_state.phone(),
                    required: true
                }
                label {
                    "for": "phone",
                    class: "absolute top-3 -z-10 origin-[0] -translate-y-6 scale-75 transform text-sm text-background duration-300 peer-placeholder-shown:translate-y-0 peer-placeholder-shown:scale-100 peer-focus:left-0 peer-focus:-translate-y-6 peer-focus:scale-75 peer-focus:font-medium peer-focus:text-picture",
                    "Numer telefonu"
                }
            }
            match assortment.value() {
                None => {
                    cx.render(rsx! { div {}})
                }
                Some(assortment) => {
                    cx.render(rsx! {assortment.into_iter().map(|item| rsx! {
                        div {
                            class: "col-span-1 mb-4 flex items-center",
                            input {
                            id: "{item}",
                            "type": "checkbox",
                            value: "",
                            class: "peer h-4 w-4 rounded border-background bg-gray-100"
                        }
                            label {
                            "for": "{item}",
                            class: "peer-checked:text-yellow-theme ml-2 text-sm font-medium text-background peer-checked:font-semibold",
                            "{item}"
                        }
                        }
                    })})
}
            }
        }
    })
}

fn tak(cx: Scope) -> Element {
    let v = |info: ClientInfo| async move { Ok(()) };

    cx.render(rsx! { ClientForm { initial_state: ClientInfo::default(), submit_handler: v } })
}

#[inline_props]
pub fn ClientPage(cx: Scope) -> Element {
    let client_options = [
        ("Informacje ogólne", "/client"),
        ("Wyszukaj klienta", "/client/search"),
        ("Dodaj klienta", "/client/add"),
    ];
    let v = |info: ClientInfo| async move { Ok(()) };

    cx.render(rsx! {
        div { class: "text-background flex border-spacing-2 flex-col items-center pt-5 pl-5",
            utils::MinimalMenu {
                options: client_options.into(),
                image: r#"<svg class="mt-4 h-20 w-full fill-inherit" height="200px" width="200px" version="1.1" id="_x32_" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 0 512 512" xml:space="preserve" fill="" stroke="">
            <g id="SVGRepo_bgCarrier" stroke-width="0"></g>
            <g id="SVGRepo_tracerCarrier" stroke-linecap="round" stroke-linejoin="round"></g>
            <g id="SVGRepo_iconCarrier">
              <g>
                <path class="st0" d="M157.604,321.598c7.26-2.232,10.041-6.696,10.6-10.046c-0.559-4.469-3.143-6.279-3.986-14.404 c-0.986-9.457,6.91-32.082,9.258-36.119c-0.32-0.772-0.65-1.454-0.965-2.247c-11.002-6.98-22.209-19.602-27.359-42.416 c-2.754-12.197-0.476-24.661,6.121-35.287c0,0-7.463-52.071,3.047-86.079c-9.818-4.726-20.51-3.93-35.164-2.466 c-11.246,1.126-12.842,3.516-21.48,2.263c-9.899-1.439-17.932-4.444-20.348-5.654c-1.392-0.694-14.449,10.89-18.084,20.35 c-11.531,29.967-8.435,50.512-5.5,66.057c-0.098,1.592-0.224,3.178-0.224,4.787l2.68,11.386c0.01,0.12,0,0.232,0.004,0.346 c-5.842,5.24-9.363,12.815-7.504,21.049c3.828,16.934,12.07,23.802,20.186,26.777c5.383,15.186,10.606,24.775,16.701,31.222 c1.541,7.027,2.902,16.57,1.916,26.032C83.389,336.78,0,315.904,0,385.481c0,9.112,25.951,23.978,88.818,28.259 c-0.184-1.342-0.31-2.695-0.31-4.078C88.508,347.268,129.068,330.379,157.604,321.598z"></path>
                <path class="st0" d="M424.5,297.148c-0.986-9.457,0.371-18.995,1.912-26.011c6.106-6.458,11.328-16.052,16.713-31.246 c8.113-2.977,16.35-9.848,20.174-26.774c1.77-7.796-1.293-15.006-6.59-20.2c3.838-12.864,18.93-72.468-26.398-84.556 c-15.074-18.839-28.258-18.087-50.871-15.827c-11.246,1.126-12.844,3.516-21.477,2.263c-1.89-0.275-3.682-0.618-5.41-0.984 c1.658,2.26,3.238,4.596,4.637,7.092c15.131,27.033,11.135,61.27,6.381,82.182c5.67,10.21,7.525,21.944,4.963,33.285 c-5.15,22.8-16.352,35.419-27.348,42.4c-0.551,1.383-2.172,4.214,0.06,7.006c2.039,3.305,2.404,2.99,4.627,5.338 c1.539,7.027,2.898,16.57,1.91,26.032c-0.812,7.85-14.352,14.404-10.533,17.576c3.756,1.581,8.113,3.234,13,5.028 c28.025,10.29,74.928,27.516,74.928,89.91c0,1.342-0.117,2.659-0.291,3.96C486.524,409.195,512,394.511,512,385.481 C512,315.904,428.613,336.78,424.5,297.148z"></path>
                <path class="st0" d="M301.004,307.957c-1.135-10.885,0.432-21.867,2.201-29.956c7.027-7.423,13.047-18.476,19.244-35.968 c9.34-3.427,18.826-11.335,23.23-30.826c2.028-8.976-1.494-17.276-7.586-23.256c4.412-14.81,21.785-83.437-30.398-97.353 c-17.354-21.692-32.539-20.825-58.57-18.222c-12.951,1.294-14.791,4.048-24.731,2.603c-11.4-1.657-20.646-5.117-23.428-6.508 c-1.602-0.803-16.637,12.538-20.826,23.428c-13.27,34.5-9.705,58.159-6.33,76.056c-0.111,1.833-0.264,3.658-0.264,5.511 l3.092,13.11c0.01,0.135,0,0.264,0.004,0.399c-6.726,6.03-10.777,14.752-8.636,24.232c4.402,19.498,13.894,27.404,23.238,30.828 c6.199,17.485,12.207,28.533,19.231,35.956c1.773,8.084,3.34,19.076,2.205,29.966c-4.738,45.626-100.744,21.593-100.744,101.706 c0,12.355,41.4,33.902,144.906,33.902c103.506,0,144.906-21.547,144.906-33.902C401.748,329.549,305.742,353.583,301.004,307.957z M240.039,430.304l-26.276-106.728l32.324,13.453l-1.738,15.619l5.135-0.112L240.039,430.304z M276.209,430.304l-9.447-77.768 l5.135,0.112l-1.738-15.619l32.324-13.453L276.209,430.304z"></path>
              </g>
            </g>
          </svg>"#
            }
        }
        div { class: "m-15 flex grow items-center justify-center", ClientForm { initial_state: ClientInfo::default(), submit_handler: v } }
    })
}

use crate::{
    api::client::Client,
    async_event,
    components::atoms::{
        flag::{Country, FlagSelect},
        modal::{show_modal, show_modal_callback, Buttons, ModalButton, ModalData, ModalStore},
    },
    data::{
        editor::EditorStore,
        locales::{LocalesStore, TK},
        resources::{Key, ResourceStore},
        session::SessionStore,
    },
    handle_api_error, use_effect_deps,
};
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::{platform::spawn_local, prelude::*};
use yewdux::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct InnerControlProps {
    pub reskey: Key,
    pub state: String,
    pub modified: bool,
}

#[function_component(Control)]
pub fn control(props: &InnerControlProps) -> Html {
    let error_state = use_state_eq(|| None);
    let (locales_store, _) = use_store::<LocalesStore>();
    let (session_store, session_dispatch) = use_store::<SessionStore>();
    let navigator = use_navigator().unwrap();
    let (_, modal_dispatch) = use_store::<ModalStore>();
    let (_, dispatch) = use_store::<EditorStore>();
    let (res_store, _) = use_store::<ResourceStore>();
    let state = props.state.clone();
    let token = session_store.token.clone().unwrap_or_default();
    let save_available = res_store.get_state(&props.reskey) != Some(&state);
    let save = async_event!(|state, token, props, error_state, dispatch| {
        match Client::update_resource(
            token.as_str(),
            props.reskey.reskey.as_str(),
            props.reskey.lang.as_str(),
            state.as_str(),
        )
        .await
        {
            Ok(_) => dispatch.reduce_mut(|s| s.remove_state(&props.reskey)),
            Err(e) => error_state.set(Some(e)),
        }
    });
    let save = show_modal_callback(
        ModalData {
            title: locales_store.get(TK::SaveChanges),
            message: locales_store.get(TK::SaveChangesQuestion),
            buttons: Buttons::ConfirmCancel(
                ModalButton::new(locales_store.get(TK::Save), Some(save)),
                ModalButton::new(locales_store.get(TK::Cancel), None),
            ),
        },
        modal_dispatch.clone(),
    );
    let discard = {
        let reskey = props.reskey.clone();
        Callback::from(move |_| {
            dispatch.reduce_mut(|s| {
                s.remove_state(&reskey);
            });
        })
    };
    let discard = show_modal_callback(
        ModalData {
            title: locales_store.get(TK::DiscardChanges),
            message: locales_store.get(TK::DiscardChangesQuestion),
            buttons: Buttons::RiskyCancel(
                ModalButton::new(locales_store.get(TK::Discard), Some(discard)),
                ModalButton::new(locales_store.get(TK::Cancel), None),
            ),
        },
        modal_dispatch.clone(),
    );
    let lang = use_state(|| Country::try_from(props.reskey.lang.as_str()).unwrap());
    let onselectedchanged = {
        let locales_store = locales_store.clone();
        let navigator = navigator.clone();
        let modal_dispatch = modal_dispatch.clone();
        let lang = lang.clone();
        let props = props.clone();
        Callback::from(move |c: Country| {
            let onclick = {
                let props = props.clone();
                let navigator = navigator.clone();
                let lang = lang.clone();
                Callback::from(move |_| {
                    lang.set(c);
                    navigator.push(&Route::Editor {
                        key: props.reskey.reskey.clone(),
                        lang: c.key().to_string(),
                    });
                })
            };
            match props.modified {
                true => show_modal(
                    ModalData {
                        title: locales_store.get(TK::DiscardChanges),
                        message: locales_store.get(TK::DiscardChangesQuestion),
                        buttons: Buttons::RiskyCancel(
                            ModalButton::new(locales_store.get(TK::Discard), Some(onclick)),
                            ModalButton::new(locales_store.get(TK::Cancel), None),
                        ),
                    },
                    modal_dispatch.clone(),
                ),
                false => {
                    lang.set(c);
                    navigator.push(&Route::Editor {
                        key: props.reskey.reskey.clone(),
                        lang: c.key().to_string(),
                    })
                }
            }
        })
    };
    let (save_class, discard_class) = match save_available {
        true => ("btn btn-success btn-sm", "btn btn-warning btn-sm"),
        false => (
            "btn btn-success btn-sm btn-disabled",
            "btn btn-warning btn-sm btn-disabled",
        ),
    };

    handle_api_error!(error_state, session_dispatch, false);
    html! {
        <div class={"flex flex-col lg:flex-row w-full justify-between gap-4 lg:gap-2"}>
            <div class={"flex flex-row gap-4 lg:gap-2"}>
                <KeySelect reskey={props.reskey.clone()} modified={props.modified}/>
                <FlagSelect country={*lang} {onselectedchanged}/>
            </div>
            <div class={"flex flex-row justify-end gap-4 lg:gap-2"}>
            <button class={save_class} onclick={save}>{locales_store.get(TK::Save)}</button>
            <button class={discard_class} onclick={discard}>{locales_store.get(TK::Discard)}</button>
            </div>
        </div>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct KeySelectProps {
    pub reskey: Key,
    pub modified: bool,
}

#[function_component(KeySelect)]
pub fn key_select(props: &KeySelectProps) -> Html {
    let error_state = use_state_eq(|| None);
    let (session_store, session_dispatch) = use_store::<SessionStore>();
    let (_, modal_dispatch) = use_store::<ModalStore>();
    let (locales_store, _) = use_store::<LocalesStore>();
    let navigator = use_navigator().unwrap();
    let keys = use_state_eq(|| vec![props.reskey.reskey.clone()]);
    let token = session_store.token.clone().unwrap_or_default();
    use_effect_deps!(|keys, error_state, token| {
        spawn_local(async move {
            match Client::get_resource_keys(&token).await {
                Ok(k) => keys.set(k),
                Err(e) => error_state.set(Some(e)),
            }
        });
    });
    let get_onclick = |key: &str| {
        let navigator = navigator.clone();
        let reskey = props.reskey.clone();
        let key = key.to_string();
        let onclick = Callback::from(move |_| {
            if let Some(element) = web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .active_element()
            {
                element.unchecked_into::<HtmlElement>().blur().unwrap();
            }
            navigator.push(&Route::Editor {
                key: key.clone(),
                lang: reskey.lang.clone(),
            })
        });
        match props.modified {
            true => show_modal_callback(
                ModalData {
                    title: locales_store.get(TK::DiscardChanges),
                    message: locales_store.get(TK::DiscardChangesQuestion),
                    buttons: Buttons::RiskyCancel(
                        ModalButton::new(locales_store.get(TK::Discard), Some(onclick)),
                        ModalButton::new(locales_store.get(TK::Cancel), None),
                    ),
                },
                modal_dispatch,
            ),
            false => onclick,
        }
    };
    handle_api_error!(error_state, session_dispatch, !props.modified);
    html! {
        <div class={"dropdown"}>
        <label class={"btn btn-sm"} tabindex={"0"}>{&props.reskey.reskey}</label>
        <ul tabindex={"0"} class={"dropdown-content flex flex-col mt-1 gap-1 z-[1]"}>
            { for keys.iter()
                .filter(|key| key != &&props.reskey.reskey)
                .map(|key| html! { <li class={"btn btn-sm w-max"} onclick={get_onclick.clone()(key)}>{key.clone()}</li> }) }
        </ul>
        </div>
    }
}

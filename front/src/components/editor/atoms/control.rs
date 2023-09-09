use crate::{
    api::client::Client,
    async_event,
    components::{
        atoms::{
            flag::{Country, FlagSelect},
            modal::{show_modal_callback, Buttons, ModalButton, ModalData, ModalStore},
        },
        editor::editor::InnerProps,
    },
    data::{editor::EditorStore, resources::Key, session::SessionStore},
    handle_api_error, use_effect_deps,
};
use yew::{platform::spawn_local, prelude::*};
use yewdux::prelude::*;

#[function_component(Control)]
pub fn control(props: &InnerProps) -> Html {
    let error_state = use_state_eq(|| None);
    let (session_store, session_dispatch) = use_store::<SessionStore>();
    let navigator = use_navigator().unwrap();
    let (_, modal_dispatch) = use_store::<ModalStore>();
    let (_, dispatch) = use_store::<EditorStore>();
    let state = props.state.clone();
    let token = session_store.token.clone().unwrap_or_default();
    let save = async_event!(|state, token, props, error_state| {
        if let Err(e) = Client::update_resource(
            token.as_str(),
            props.reskey.reskey.as_str(),
            props.reskey.lang.as_str(),
            state.value.as_str(),
        )
        .await
        {
            error_state.set(Some(e));
        }
    });
    let save = show_modal_callback(
        ModalData {
            title: "Save changes?".to_string(),
            message: "Are you sure you want to save your changes?".to_string(),
            buttons: Buttons::ConfirmCancel(
                ModalButton {
                    text: "Save".to_string(),
                    onclick: Some(save),
                },
                ModalButton {
                    text: "Cancel".to_string(),
                    onclick: None,
                },
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
            title: "Discard changes?".to_string(),
            message: "Are you sure you want to discard your changes?".to_string(),
            buttons: Buttons::RiskyCancel(
                ModalButton {
                    text: "Discard".to_string(),
                    onclick: Some(discard),
                },
                ModalButton {
                    text: "Cancel".to_string(),
                    onclick: None,
                },
            ),
        },
        modal_dispatch,
    );
    let onselectedchanged = {
        let key = props.reskey.reskey.clone();
        Callback::from(move |c: Country| {
            navigator.push(&Route::Editor {
                key: key.clone(),
                lang: c.key().to_string(),
            })
        })
    };

    handle_api_error!(error_state, session_dispatch);
    html! {
        <div class={"flex flex-row w-full justify-between gap-2"}>
            <div class={"flex flex-row gap-2"}>
                <KeySelect reskey={props.reskey.clone()}/>
                <FlagSelect country={Country::try_from(props.reskey.lang.as_str()).unwrap()} {onselectedchanged}/>
            </div>
            <div class={"flex flex-row gap-2"}>
            <button class={"btn btn-success btn-sm"} onclick={save}>{"Save"}</button>
            <button class={"btn btn-warning btn-sm"} onclick={discard}>{"Discard"}</button>
            </div>
        </div>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct KeySelectProps {
    pub reskey: Key,
}

#[function_component(KeySelect)]
pub fn key_select(props: &KeySelectProps) -> Html {
    let error_state = use_state_eq(|| None);
    let (session_store, session_dispatch) = use_store::<SessionStore>();
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
        Callback::from(move |_| {
            navigator.push(&Route::Editor {
                key: key.clone(),
                lang: reskey.lang.clone(),
            })
        })
    };
    handle_api_error!(error_state, session_dispatch);
    html! {
        <div class={"dropdown"}>
        <label class={"btn btn-sm"} tabindex={"0"}>{&props.reskey.reskey}</label>
        <ul tabindex={"0"} class={"dropdown-content flex flex-col mt-1 gap-1 z-[1]"}>
            { for keys.iter()
                .filter(|key| key != &&props.reskey.reskey)
                .map(|key| html! { <li class={"btn btn-sm w-max"} onclick={get_onclick(key)}>{key.clone()}</li> }) }
        </ul>
        </div>
    }
}

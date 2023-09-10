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
    data::{
        editor::EditorStore,
        locales::{LocalesStore, TK},
        resources::Key,
        session::SessionStore,
    },
    handle_api_error, use_effect_deps,
};
use yew::{platform::spawn_local, prelude::*};
use yewdux::prelude::*;

#[function_component(Control)]
pub fn control(props: &InnerProps) -> Html {
    let error_state = use_state_eq(|| None);
    let (locales_store, _) = use_store::<LocalesStore>();
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
        <div class={"flex flex-col lg:flex-row w-full justify-between gap-4 lg:gap-2"}>
            <div class={"flex flex-row gap-4 lg:gap-2"}>
                <KeySelect reskey={props.reskey.clone()}/>
                <div class={"w-12 h-8"}>
                    <FlagSelect country={Country::try_from(props.reskey.lang.as_str()).unwrap()} {onselectedchanged}/>
                </div>
            </div>
            <div class={"flex flex-row justify-end gap-4 lg:gap-2"}>
            <button class={"btn btn-success btn-sm"} onclick={save}>{locales_store.get(TK::Save)}</button>
            <button class={"btn btn-warning btn-sm"} onclick={discard}>{locales_store.get(TK::Discard)}</button>
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

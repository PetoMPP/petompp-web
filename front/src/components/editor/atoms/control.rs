use crate::{
    api::client::Client,
    async_event,
    components::{
        atoms::{
            flag::{Country, Flag},
            modal::{show_modal_callback, Buttons, ModalButton, ModalData, ModalStore},
        },
        editor::{data::Store, editor::InnerProps},
    },
    data::session::SessionStore,
    handle_api_error,
};
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(Control)]
pub fn control(props: &InnerProps) -> Html {
    let error_state = use_state_eq(|| None);
    let (session_store, session_dispatch) = use_store::<SessionStore>();
    let (_, modal_dispatch) = use_store::<ModalStore>();
    let (_, dispatch) = use_store::<Store>();
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

    handle_api_error!(error_state, session_dispatch);
    html! {
        <div class={"flex flex-row w-full justify-between gap-2"}>
            <div class={"flex flex-row gap-2 text-2xl text-primary-content"}>
                <p>{"Editing:"}</p>
                <p class={"font-mono"}>{props.reskey.reskey.clone()}</p>
                <Flag country={Country::try_from(props.reskey.lang.as_str()).unwrap_or_default()} />
            </div>
            <div class={"flex flex-row gap-2"}>
            <button class={"btn btn-success btn-sm"} onclick={save}>{"Save"}</button>
            <button class={"btn btn-warning btn-sm"} onclick={discard}>{"Discard"}</button>
            </div>
        </div>
    }
}

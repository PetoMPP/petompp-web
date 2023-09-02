use crate::{
    api::client::Client,
    async_event,
    components::{
        atoms::flag::{Country, Flag},
        editor::{
            data::{get_or_create_state, Store},
            editor::EditorProps,
        },
    },
    handle_api_error, SessionStore,
};
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(Control)]
pub fn control(props: &EditorProps) -> Html {
    let error_state = use_state_eq(|| None);
    let (session_store, session_dispatch) = use_store::<SessionStore>();
    let (store, dispatch) = use_store::<Store>();
    let state = get_or_create_state(&props.reskey, &store, dispatch.clone());
    let token = session_store.token.clone().unwrap_or_default();
    let save = async_event!(state, token, props, error_state, {
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
    let discard = async_event!(props, error_state, dispatch, {
        match Client::get_resource(props.reskey.reskey.as_str(), props.reskey.lang.as_str()).await {
            Ok(resource) => {
                dispatch.reduce_mut(|s| {
                    s.values.get_mut(&props.reskey.to_string()).unwrap().value = resource.clone();
                });
            }
            Err(e) => error_state.set(Some(e)),
        }
    });
    handle_api_error!(error_state, session_dispatch);
    html! {
        <div class={"flex flex-row w-full justify-between gap-2"}>
            <div class={"flex flex-row gap-2 text-2xl text-primary-content"}>
                <p>{"Editing:"}</p>
                <p class={"font-mono"}>{props.reskey.reskey.clone()}</p>
                <Flag country={Country::from(props.reskey.lang.as_str())} />
            </div>
            <div class={"flex flex-row gap-2"}>
            <button class={"btn btn-success btn-sm"} onclick={save}>{"Save"}</button>
            <button class={"btn btn-warning btn-sm"} onclick={discard}>{"Discard"}</button>
            </div>
        </div>
    }
}

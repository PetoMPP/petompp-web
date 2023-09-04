use crate::{
    api::client::Client,
    async_event,
    components::{
        atoms::flag::{Country, Flag},
        editor::{data::Store, editor::InnerProps},
    },
    handle_api_error, SessionStore,
};
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(Control)]
pub fn control(props: &InnerProps) -> Html {
    let error_state = use_state_eq(|| None);
    let (session_store, session_dispatch) = use_store::<SessionStore>();
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
    let discard = {
        let reskey = props.reskey.clone();
        Callback::from(move |_| {
            dispatch.reduce_mut(|s| {
                s.remove_state(&reskey);
            });
        })
    };
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

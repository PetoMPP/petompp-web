use super::{
    atoms::{control::Control, display::Display, editor::Editor as EditorInner},
    data::{Key, Store},
};
use crate::{
    api::client::Client, components::editor::data::State, handle_api_error, use_effect_deps,
    SessionStore,
};
use yew::{platform::spawn_local, prelude::*};
use yewdux::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct EditorProps {
    pub reskey: Key,
}

#[derive(Clone, PartialEq, Properties)]
pub struct InnerProps {
    pub reskey: Key,
    pub state: State,
}

#[function_component(Editor)]
pub fn editor(props: &EditorProps) -> Html {
    let (_, session_dispatch) = use_store::<SessionStore>();
    let (store, dispatch) = use_store::<Store>();
    let error_state = use_state_eq(|| None);
    let state = use_state_eq(|| State::default());
    let reskey = props.reskey.clone();
    use_effect_deps!(|state, reskey, store, dispatch, error_state| {
        if let Some(s) = store.get_state(&reskey) {
            state.set(s.clone());
            return;
        }
        spawn_local(async move {
            match Client::get_resource(&reskey.reskey, &reskey.lang).await {
                Ok(s) => {
                    let s = State {
                        value: s,
                        ..Default::default()
                    };
                    dispatch.reduce_mut(|store| {
                        store.set_state(&reskey, s);
                    });
                }
                Err(e) => error_state.set(Some(e)),
            };
        });
    });
    handle_api_error!(error_state, session_dispatch);
    let onclick = {
        let dispatch = dispatch.clone();
        let props = props.clone();
        let state = state.clone();
        Callback::from(move |_| {
            dispatch.reduce_mut(|s| {
                s.get_state_mut(&props.reskey).unwrap().preview = !state.preview;
            });
        })
    };
    let btn_text = match state.preview {
        true => "Edit",
        false => "Preview",
    };
    html! {
        <div class={"bg-primary rounded-lg"}>
            <div class={"flex flex-row gap-2 p-2 rounded-t-lg"}>
            <Control reskey={props.reskey.clone()} state={(*state).clone()} />
            </div>
            <div class={"relative m-2 mt-0 bg-base-100"}>
                <a class={"absolute right-2 btn btn-sm btn-primary no-animation rounded-none rounded-b-md"} {onclick}>{btn_text}</a>
                {
                    if state.preview { html! { <Display reskey={props.reskey.clone()} state={(*state).clone()} /> } }
                    else { html! { <EditorInner reskey={props.reskey.clone()} state={(*state).clone()} /> } }
                }
            </div>
        </div>
    }
}

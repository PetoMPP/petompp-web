use super::atoms::{control::Control, display::Display, editor::Editor as EditorInner};
use crate::{
    api::client::Client,
    data::{
        editor::{EditorState, EditorStore},
        locales::{LocalesStore, TK},
        resources::Key,
        session::SessionStore,
    },
    handle_api_error, use_effect_deps,
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
    pub state: EditorState,
}

#[function_component(Editor)]
pub fn editor(props: &EditorProps) -> Html {
    let (_, session_dispatch) = use_store::<SessionStore>();
    let (store, dispatch) = use_store::<EditorStore>();
    let (locales_store, _) = use_store::<LocalesStore>();
    let error_state = use_state_eq(|| None);
    let preview = use_state_eq(|| false);
    let state = use_state_eq(|| EditorState::default());
    let reskey = props.reskey.clone();
    use_effect_deps!(|state, reskey, store, dispatch, error_state| {
        if let Some(s) = store.get_state(&reskey) {
            state.set(s.clone());
            return;
        }
        spawn_local(async move {
            match Client::get_resource(&reskey.reskey, &reskey.lang).await {
                Ok(value) => {
                    let s = EditorState {
                        value,
                        footprint: chrono::Utc::now().timestamp_millis(),
                    };
                    dispatch.reduce_mut(|store| {
                        store.add_state(&reskey, s);
                    });
                }
                Err(e) => error_state.set(Some(e)),
            };
        });
    });
    handle_api_error!(error_state, session_dispatch);
    let onclick = {
        let preview = preview.clone();
        Callback::from(move |_| preview.set(!*preview))
    };
    let btn_text = match *preview {
        true => locales_store.get(TK::Editor),
        false => locales_store.get(TK::Preview),
    };
    html! {
        <div class={"bg-primary rounded-lg"}>
            <div class={"flex flex-row gap-2 p-2 rounded-t-lg"}>
            <Control reskey={props.reskey.clone()} state={(*state).clone()} />
            </div>
            <div class={"relative m-2 mt-0 bg-base-100"}>
                <a class={"absolute right-2 btn btn-sm btn-primary no-animation rounded-none rounded-b-md"} {onclick}>{btn_text}</a>
                {
                    if *preview { html! { <Display reskey={props.reskey.clone()} state={(*state).clone()} /> } }
                    else { html! { <EditorInner reskey={props.reskey.clone()} state={(*state).clone()} /> } }
                }
            </div>
        </div>
    }
}

use super::atoms::{control::Control, editor::Editor as EditorInner};
use crate::{
    api::client::Client,
    components::editor::atoms::editor::save_editor_state,
    data::{
        editor::EditorStore,
        locales::{LocalesStore, TK},
        resources::{Key, ResourceStore},
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
    pub state: String,
}

#[function_component(Editor)]
pub fn editor(props: &EditorProps) -> Html {
    let (_, session_dispatch) = use_store::<SessionStore>();
    let (store, dispatch) = use_store::<EditorStore>();
    let (res_store, res_dispatch) = use_store::<ResourceStore>();
    let (locales_store, _) = use_store::<LocalesStore>();
    let error_state = use_state_eq(|| None);
    let preview = use_state_eq(|| false);
    let state = use_state_eq(String::new);
    let reskey = props.reskey.clone();
    use_effect_deps!(|state, reskey, store, error_state| {
        match store.get_state(&reskey) {
            Some(s) => state.set(s.clone()),
            None => match res_store.get_state(&reskey) {
                Some(s) => state.set(s.clone()),
                None => state.set(String::new()),
            },
        }
        spawn_local(async move {
            match Client::get_resource(reskey.reskey.as_str(), reskey.lang.as_str()).await {
                Ok(val) => {
                    if res_store.get_state(&reskey) != Some(&val) {
                        res_dispatch.reduce_mut(|store| {
                            store.add_or_update_state(&reskey, val);
                        });
                    }
                }
                Err(e) => error_state.set(Some(e)),
            }
        });
    });
    handle_api_error!(error_state, session_dispatch, false);
    let onclick = {
        let preview = preview.clone();
        Callback::from(move |_| preview.set(!*preview))
    };
    let btn_text = match *preview {
        true => locales_store.get(TK::Editor),
        false => locales_store.get(TK::Preview),
    };
    let local_changes = use_state_eq(|| false);
    let local_class = match &*local_changes {
        true => "lg:absolute lg:top-0 lg:left-0 lg:right-0 lg:w-fit lg:mx-auto shrink rounded-b-md btn btn-sm btn-secondary opacity-70 no-animation rounded-none",
        false => "hidden",
    };
    let save_local =
        Callback::from(move |_| save_editor_state(store.clone(), dispatch.clone(), reskey.clone()));
    let onmodifiedchanged = {
        let local_changes = local_changes.clone();
        Callback::from(move |modified| {
            local_changes.set(modified);
        })
    };
    html! {
        <div class={"bg-primary rounded-lg"}>
            <div class={"flex flex-row gap-2 p-2 rounded-t-lg"}>
            <Control reskey={props.reskey.clone()} state={(*state).clone()} modified={*local_changes} />
            </div>
            <div class={"relative m-2 mt-0 bg-base-100"}>
                <div class={"flex flex-row justify-end gap-2 px-2"}>
                    <a class={local_class} onclick={save_local}>{locales_store.get(TK::SaveDraft)}</a>
                    <a class={"lg:absolute right-2 top-0 rounded-b-md btn btn-sm btn-primary opacity-70 no-animation rounded-none"} {onclick}>{btn_text}</a>
                </div>
                <EditorInner reskey={props.reskey.clone()} state={(*state).clone()} preview={*preview} {onmodifiedchanged}/>
            </div>
        </div>
    }
}

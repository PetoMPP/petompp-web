use super::{
    atoms::{control::Control, display::Display, editor::Editor as EditorInner},
    data::{Key, Store},
};
use crate::components::editor::data::get_or_create_state;
use yew::prelude::*;
use yewdux::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct EditorProps {
    pub reskey: Key,
}

#[function_component(Editor)]
pub fn editor(props: &EditorProps) -> Html {
    let (store, dispatch) = use_store::<Store>();
    let state = get_or_create_state(&props.reskey, &store, dispatch.clone());
    let onclick = {
        let dispatch = dispatch.clone();
        let props = props.clone();
        Callback::from(move |_| {
            let props = props.clone();
            dispatch.reduce_mut(|s| {
                s.values.get_mut(&props.reskey.to_string()).unwrap().preview = !state.preview;
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
            <Control reskey={props.reskey.clone()} />
            </div>
            <div class={"relative m-2 mt-0 bg-base-100"}>
                <div class={"absolute right-2 flex flex-row justify-end gap-2"}>
                <a class={"btn btn-sm btn-primary no-animation rounded-none rounded-b-md"} {onclick}>{btn_text}</a>
                </div>
                {
                    if state.preview { html! { <Display reskey={props.reskey.clone()} /> } }
                    else { html! { <EditorInner reskey={props.reskey.clone()} /> } }
                }
            </div>
        </div>
    }
}

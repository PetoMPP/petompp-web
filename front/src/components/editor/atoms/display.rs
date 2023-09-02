use crate::components::{
    atoms::markdown::Markdown,
    editor::{
        data::{get_or_create_state, Store},
        editor::EditorProps,
    },
};
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(Display)]
pub fn display(props: &EditorProps) -> Html {
    let (store, dispatch) = use_store::<Store>();
    let state = get_or_create_state(&props.reskey, &store, dispatch);
    let value = match state.value.as_str() {
        "" => "# Hello World!".to_string(),
        val => val.to_string(),
    };

    html! {
        <div class={"p-4 rounded-b-lg"}><Markdown markdown={value} /></div>
    }
}

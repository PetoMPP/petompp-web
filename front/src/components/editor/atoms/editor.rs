use crate::components::editor::{
    data::{get_or_create_state, Store},
    editor::EditorProps,
};
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlInputElement};
use yew::prelude::*;
use yewdux::prelude::*;

const TEXTAREA_ID: &str = "editor-textarea";

#[function_component(Editor)]
pub fn editor(props: &EditorProps) -> Html {
    let (store, dispatch) = use_store::<Store>();
    let state = get_or_create_state(&props.reskey, &store, dispatch.clone());
    use_effect_with_deps(
        move |value| {
            set_textarea_text(value.clone());
            || {}
        },
        state.value.clone(),
    );
    let onchange = {
        let dispatch = dispatch.clone();
        let props = props.clone();
        Callback::from(move |e: Event| {
            let props = props.clone();
            let element: HtmlInputElement = e.target_unchecked_into();
            dispatch.reduce_mut(|s| {
                s.values.get_mut(&props.reskey.to_string()).unwrap().value = element.value();
            });
        })
    };
    let onkeydown = Callback::from(|e: KeyboardEvent| {
        let element: HtmlInputElement = e.target_unchecked_into();
        if e.key() == "Tab" {
            e.prevent_default();
            let start = element
                .selection_start()
                .unwrap_or_default()
                .unwrap_or_default();
            let end = element
                .selection_end()
                .unwrap_or_default()
                .unwrap_or_default();
            let value = element.value();
            let new_value = format!(
                "{}{}{}",
                &value.as_str()[..(start as usize)],
                "\t",
                &value[(end as usize)..]
            );
            element.set_value(new_value.as_str());
            element.set_selection_start(Some(start + 4)).unwrap();
            element.set_selection_end(Some(start + 4)).unwrap();
        }
    });
    let onkeyup = {
        let dispatch = dispatch.clone();
        let props = props.clone();
        Callback::from(move |e: KeyboardEvent| {
            let props = props.clone();
            let element: HtmlInputElement = e.target_unchecked_into();
            set_textarea_height(&element);
            dispatch.reduce_mut(|s| {
                s.values.get_mut(&props.reskey.to_string()).unwrap().value = element.value();
            });
        })
    };
    html! {
        <div class={"flex flex-col grow"}>
            <textarea id={TEXTAREA_ID} {onchange} {onkeydown} {onkeyup} class={"flex grow bg-base-100 outline-none p-2 rounded-b-lg overflow-hidden resize-none leading-normal"}></textarea>
        </div>
    }
}

fn set_textarea_text(value: String) {
    let element: HtmlInputElement = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id(TEXTAREA_ID)
        .unwrap()
        .unchecked_into();
    element.set_value(value.as_str());
    set_textarea_height(&element);
}

fn set_textarea_height(element: &Element) {
    let body = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .body()
        .unwrap();
    body.set_attribute(
        "style",
        format!("height: {}px;", body.client_height()).as_str(),
    )
    .unwrap();
    element.set_attribute("style", "height: auto;").unwrap();
    let scroll_height = element.scroll_height();
    if scroll_height > element.client_height() {
        element
            .set_attribute("style", format!("height: {}px;", scroll_height).as_str())
            .unwrap();
    }
    body.set_attribute("style", "height: auto;").unwrap();
}

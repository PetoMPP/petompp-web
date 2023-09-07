use crate::components::editor::data::{Key, State, Store};
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlInputElement};
use yew::prelude::*;
use yewdux::prelude::*;

const TEXTAREA_ID: &str = "editor-textarea";

#[derive(Clone, PartialEq, Properties)]
pub struct InnerEditorProps {
    pub reskey: Key,
    pub initial: Option<String>,
}

#[function_component(Editor)]
pub fn editor(props: &InnerEditorProps) -> Html {
    let props = props.clone();
    let (store, dispatch) = use_store::<Store>();
    use_effect_with_deps(
        move |props| {
            if let Some(value) = props.initial.clone() {
                set_textarea_text(value);
            }
            let props = props.clone();
            move || {
                if props.initial.is_some() {
                    save_editor_state(store, dispatch, props.reskey.clone());
                }
            }
        },
        props.clone(),
    );
    let onkeyup = Callback::from(move |e: KeyboardEvent| {
        let element: HtmlInputElement = e.target_unchecked_into();
        set_textarea_height(&element);
    });
    html! {
        <div class={"flex flex-col grow"}>
            <textarea id={TEXTAREA_ID} {onkeyup} class={"flex grow bg-base-100 outline-none p-2 rounded-b-lg overflow-hidden resize-none leading-normal"}></textarea>
        </div>
    }
}

pub fn save_editor_state(store: Rc<Store>, dispatch: Dispatch<Store>, reskey: Key) {
    if let Some(element) = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id(TEXTAREA_ID)
    {
        gloo::console::log!("Saving editor state");
        let element: HtmlInputElement = element.unchecked_into();
        let value = element.value();
        if value == store.get_state(&reskey).unwrap().value.as_str() {
            return;
        }
        let state = State { value };
        dispatch.reduce_mut(|store| {
            if let Some(s) = store.get_state_mut(&reskey) {
                *s = state;
            }
        });
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

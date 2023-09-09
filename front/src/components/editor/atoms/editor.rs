use crate::{
    components::editor::editor::InnerProps,
    data::{editor::EditorStore, resources::Key},
};
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlInputElement};
use yew::prelude::*;
use yewdux::prelude::*;

const TEXTAREA_ID: &str = "editor-textarea";

#[function_component(Editor)]
pub fn editor(props: &InnerProps) -> Html {
    let props = props.clone();
    let (store, dispatch) = use_store::<EditorStore>();
    let fp = use_state_eq(|| -1);
    use_effect_with_deps(
        move |(props, fp)| {
            if props.state.footprint != **fp {
                fp.set(props.state.footprint);
                set_textarea_text(props.state.value.clone());
            }
            move || {}
        },
        (props.clone(), fp.clone()),
    );
    let oninput = Callback::from(move |e: InputEvent| {
        let element: HtmlInputElement = e.target_unchecked_into();
        set_textarea_height(&element);
        save_editor_state(store.clone(), dispatch.clone(), props.reskey.clone());
    });
    html! {
        <div class={"flex flex-col grow"}>
            <textarea id={TEXTAREA_ID} {oninput} class={"flex grow bg-base-100 outline-none p-2 rounded-b-lg overflow-hidden resize-none leading-normal"}></textarea>
        </div>
    }
}

fn save_editor_state(store: Rc<EditorStore>, dispatch: Dispatch<EditorStore>, reskey: Key) {
    if let Some(element) = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id(TEXTAREA_ID)
    {
        let element: HtmlInputElement = element.unchecked_into();
        let value = element.value();
        if value == store.get_state(&reskey).unwrap().value.as_str() {
            return;
        }
        dispatch.reduce_mut(|store| {
            if let Some(s) = store.get_state_mut(&reskey) {
                s.value = value;
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

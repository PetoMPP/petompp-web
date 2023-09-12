use crate::{
    api::client::{ApiError, Client},
    components::editor::editor::InnerProps,
    data::{editor::EditorStore, resources::Key, session::SessionStore},
    handle_api_error,
};
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::{ClipboardEvent, Element, HtmlInputElement};
use yew::{platform::spawn_local, prelude::*};
use yewdux::prelude::*;

const TEXTAREA_ID: &str = "editor-textarea";

#[function_component(Editor)]
pub fn editor(props: &InnerProps) -> Html {
    let props = props.clone();
    let error_state = use_state_eq(|| None);
    let (store, dispatch) = use_store::<EditorStore>();
    let (session_store, session_dispatch) = use_store::<SessionStore>();
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

    let onpaste = {
        let props = props.clone();
        let store = store.clone();
        let dispatch = dispatch.clone();
        let session_store = session_store.clone();
        let error_state = error_state.clone();
        Callback::from(move |e: Event| {
            let props = props.clone();
            let store = store.clone();
            let dispatch = dispatch.clone();
            let session_store = session_store.clone();
            let error_state = error_state.clone();
            let Ok(e) = e.dyn_into::<ClipboardEvent>() else {
                return;
            };
            gloo::console::log!(&e);
            let Some(data) = e.clipboard_data() else {
                return;
            };
            let Some(files) = data.files() else {
                return;
            };
            let Some(file) = files.get(0) else {
                return;
            };
            gloo::console::log!(&file);
            spawn_local(async move {
                match Client::upload_img(
                    session_store
                        .token
                        .as_ref()
                        .map(|t| t.as_str())
                        .unwrap_or_default(),
                    file,
                )
                .await
                {
                    Ok(url) => {
                        let element: HtmlInputElement = e.target_unchecked_into();
                        let value = element.value();
                        let sel_start = element
                            .selection_start()
                            .unwrap_or_default()
                            .unwrap_or_default() as usize;
                        let sel_end = element
                            .selection_end()
                            .unwrap_or_default()
                            .unwrap_or_default() as usize;
                        let new_value = format!(
                            "{}![{}]({}){}",
                            &value.chars().take(sel_start).collect::<String>(),
                            &value
                                .chars()
                                .skip(sel_start)
                                .take(sel_end - sel_start)
                                .collect::<String>(),
                            url,
                            &value.chars().skip(sel_end).collect::<String>()
                        );
                        set_textarea_text(new_value);
                        save_editor_state(store.clone(), dispatch.clone(), props.reskey.clone());
                    }
                    Err(e) => {
                        gloo::console::log!(e.to_string());
                        if let ApiError::Endpoint(413, _) = e {
                            show_error("File too large");
                        } else {
                            error_state.set(Some(e))
                        }
                    }
                }
            })
        })
    };
    let oninput = Callback::from(move |e: InputEvent| {
        let element: HtmlInputElement = e.target_unchecked_into();
        set_textarea_height(&element);
        save_editor_state(store.clone(), dispatch.clone(), props.reskey.clone());
    });
    handle_api_error!(error_state, session_dispatch);
    html! {
        <div class={"flex flex-col grow"}>
            <textarea id={TEXTAREA_ID} {oninput} {onpaste} class={"flex grow font-mono bg-base-100 outline-none p-2 rounded-b-lg overflow-hidden resize-none leading-normal"}></textarea>
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

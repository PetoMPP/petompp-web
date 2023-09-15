use crate::{
    api::client::{ApiError, Client},
    components::atoms::markdown::Markdown,
    data::{editor::EditorStore, resources::Key, session::SessionStore},
    handle_api_error,
};
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::{ClipboardEvent, Element, HtmlInputElement};
use yew::{platform::spawn_local, prelude::*};
use yewdux::prelude::*;

const TEXTAREA_ID: &str = "editor-textarea";

#[derive(Clone, PartialEq, Properties)]
pub struct InnerEditProps {
    pub reskey: Key,
    pub state: String,
    pub preview: bool,
    pub onmodifiedchanged: Callback<bool>,
}

#[function_component(Editor)]
pub fn editor(props: &InnerEditProps) -> Html {
    const UPLOAD_FOLDER: &str = "editor";
    let props = props.clone();
    let error_state = use_state_eq(|| None);
    let current_val = use_mut_ref(|| String::new());
    let last_state = use_mut_ref(|| props.state.clone());
    let last_mod_state = use_mut_ref(|| false);
    let markdown = use_state(|| String::new());
    let (session_store, session_dispatch) = use_store::<SessionStore>();
    use_effect_with_deps(
        move |(props, current_val, last_state, markdown, last_mod_state)| {
            let val = match !current_val.borrow().is_empty() && last_state.borrow().eq(&props.state)
            {
                true => current_val.borrow().to_string(),
                false => {
                    current_val.borrow_mut().clear();
                    let mut last_mod = last_mod_state.borrow_mut();
                    if *last_mod {
                        props.onmodifiedchanged.emit(false);
                        *last_mod = false;
                    }
                    props.state.clone()
                }
            };
            set_textarea_text(val.as_str());
            markdown.set(val);
            last_state.replace(props.state.clone());
            move || {}
        },
        (
            props.clone(),
            current_val.clone(),
            last_state.clone(),
            markdown.clone(),
            last_mod_state.clone(),
        ),
    );

    let onpaste = {
        let last_state = last_state.clone();
        let last_mod_state = last_mod_state.clone();
        let props = props.clone();
        let current_val = current_val.clone();
        let session_store = session_store.clone();
        let error_state = error_state.clone();
        Callback::from(move |e: Event| {
            let last_state = last_state.clone();
            let last_mod_state = last_mod_state.clone();
            let props = props.clone();
            let current_val = current_val.clone();
            let session_store = session_store.clone();
            let error_state = error_state.clone();
            let Ok(e) = e.dyn_into::<ClipboardEvent>() else {
                return;
            };
            let Some(data) = e.clipboard_data() else {
                return;
            };
            let Some(files) = data.files() else {
                return;
            };
            let Some(file) = files.get(0) else {
                return;
            };
            spawn_local(async move {
                match Client::upload_img(
                    session_store
                        .token
                        .as_ref()
                        .map(|t| t.as_str())
                        .unwrap_or_default(),
                    file,
                    UPLOAD_FOLDER,
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
                        set_textarea_text(new_value.as_str());
                        let mut last_mod = last_mod_state.borrow_mut();
                        let changed = last_state.borrow().ne(&new_value);
                        if changed != *last_mod {
                            props.onmodifiedchanged.emit(changed);
                            *last_mod = changed;
                        }
                        *current_val.borrow_mut() = new_value;
                    }
                    Err(e) => {
                        gloo::console::log!(e.to_string());
                        if let ApiError::Endpoint(413, e) = e {
                            show_error(e.to_string(), false);
                        } else {
                            error_state.set(Some(e))
                        }
                    }
                }
            })
        })
    };
    let oninput = {
        let last_mod_state = last_mod_state.clone();
        Callback::from(move |e: InputEvent| {
            let element: HtmlInputElement = e.target_unchecked_into();
            let value = element.value();
            let mut last_mod = last_mod_state.borrow_mut();
            let changed = last_state.borrow().ne(&value);
            if changed != *last_mod {
                props.onmodifiedchanged.emit(changed);
                *last_mod = changed;
            }
            *current_val.borrow_mut() = value;
            set_textarea_height(&element);
        })
    };
    handle_api_error!(error_state, session_dispatch, !*last_mod_state.borrow());
    let (edit_class, display_class) = match props.preview {
        true => ("hidden", "p-4 rounded-b-lg"),
        false => ("flex flex-col grow", "hidden"),
    };
    html! {
        <>
        <div class={edit_class}>
            <textarea id={TEXTAREA_ID} {oninput} {onpaste} class={"flex grow font-mono bg-base-100 outline-none p-2 rounded-b-lg overflow-hidden resize-none leading-normal"}></textarea>
        </div>
        <div class={display_class}><Markdown markdown={(*markdown).clone()} allowhtml={true} /></div>
        </>
    }
}

pub fn save_editor_state(store: Rc<EditorStore>, dispatch: Dispatch<EditorStore>, reskey: Key) {
    if let Some(element) = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id(TEXTAREA_ID)
    {
        let element: HtmlInputElement = element.unchecked_into();
        let value = element.value();
        if Some(&value) == store.get_state(&reskey) {
            return;
        }
        dispatch.reduce_mut(|store| {
            store.add_or_update_state(&reskey, value);
        });
    }
}

fn set_textarea_text(value: &str) {
    let element: HtmlInputElement = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id(TEXTAREA_ID)
        .unwrap()
        .unchecked_into();
    element.set_value(value);
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

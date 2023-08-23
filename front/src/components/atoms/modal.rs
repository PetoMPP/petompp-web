use wasm_bindgen::JsCast;
use web_sys::HtmlDialogElement;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{router::Route, utils::ext::Mergable};

#[derive(PartialEq, Clone)]
pub struct ModalButton {
    pub text: String,
    pub onclick: Option<Callback<MouseEvent>>,
}

impl ModalButton {
    pub fn new(text: impl Into<String>, onclick: Option<Callback<MouseEvent>>) -> Self {
        Self {
            text: text.into(),
            onclick,
        }
    }
}

#[derive(PartialEq, Clone)]
pub enum Buttons {
    Confirm(ModalButton),
    ConfirmCancel(ModalButton, ModalButton),
    RiskyCancel(ModalButton, ModalButton),
}

#[derive(PartialEq, Properties)]
pub struct ModalProps {
    pub id: String,
    pub title: String,
    pub message: String,
    pub buttons: Buttons,
    pub children: Children,
}

#[function_component(Modal)]
pub fn modal(props: &ModalProps) -> Html {
    html! {
        <>
        {props.children.clone()}
        <dialog id={props.id.clone()} class={"modal"}>
        <form method={"dialog"} class={"modal-box"}>
            <h3 class={"font-bold text-lg"}>{&props.title}</h3>
            <p class={"py-4"}>{{&props.message}}</p>
            <div class={"flex flex-row-reverse justify-between"}>
                {get_buttons(props)}
            </div>
        </form>
        </dialog>
        </>
    }
}

#[derive(PartialEq, Properties)]
pub struct ErrorModalProps {
    pub id: String,
    pub title: String,
    pub message: String,
    pub children: Children,
}

pub const ERROR_MODAL_ID: &str = "error_modal";
pub const ERROR_MODAL_MSG_ID: &str = "error_modal_msg";

#[function_component(ErrorModal)]
pub fn error_modal() -> Html {
    let navigator = use_navigator().unwrap();
    html! {
        <>
        <dialog id={ERROR_MODAL_ID} class={"modal"}>
        <form method={"dialog"} class={"modal-box bg-warning text-warning-content"}>
            <h3 class={"font-bold text-lg"}>{"An error has occured!"}</h3>
            <p id={ERROR_MODAL_MSG_ID} class={"py-4"}></p>
            <div class={"flex flex-row-reverse justify-between"}>
                <button class="btn btn-error" onclick={Callback::from(move |_| navigator.push(&Route::Home)).merge(get_modal_close_callback(ERROR_MODAL_ID))}>{"OK"}</button>
            </div>
        </form>
        </dialog>
        </>
    }
}

pub fn show_error(msg: impl Into<String>) {
    let msg: String = msg.into();
    let modal: HtmlDialogElement = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id(ERROR_MODAL_ID)
        .unwrap()
        .unchecked_into();
    let msg_el: web_sys::HtmlElement = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id(ERROR_MODAL_MSG_ID)
        .unwrap()
        .unchecked_into();
    msg_el.set_inner_text(msg.as_str());
    modal.show_modal().unwrap();
}

pub fn get_modal_open_callback(id: impl Into<String>) -> Callback<MouseEvent> {
    let id: String = id.into();
    Callback::from(move |_: MouseEvent| {
        let modal: HtmlDialogElement = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id(id.as_str())
            .unwrap()
            .unchecked_into();
        modal.show_modal().unwrap();
    })
}

pub fn get_modal_close_callback(id: impl Into<String>) -> Callback<MouseEvent> {
    let id = id.into();
    Callback::from(move |_: MouseEvent| {
        let modal: HtmlDialogElement = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id(id.as_str())
            .unwrap()
            .unchecked_into();
        modal.close();
    })
}

fn get_buttons(props: &ModalProps) -> Html {
    match props.buttons.clone() {
        Buttons::Confirm(button) => {
            let onclick = into_modal_onclick(button.onclick, &props.id);
            html! {
                <button class="btn btn-primary" {onclick}>{&button.text}</button>
            }
        }
        Buttons::ConfirmCancel(confirm_button, cancel_button) => {
            let confirm_onclick = into_modal_onclick(confirm_button.onclick, &props.id);
            let cancel_onclick = into_modal_onclick(cancel_button.onclick, &props.id);
            html! {
                <>
                <button class="btn btn-neutral" onclick={cancel_onclick}>{&cancel_button.text}</button>
                <button class="btn btn-primary" onclick={confirm_onclick}>{&confirm_button.text}</button>
                </>
            }
        }
        Buttons::RiskyCancel(risky_button, cancel_button) => {
            let risky_onclick = into_modal_onclick(risky_button.onclick, &props.id);
            let cancel_onclick = into_modal_onclick(cancel_button.onclick, &props.id);
            html! {
                <>
                <button class="btn btn-neutral" onclick={cancel_onclick}>{&cancel_button.text}</button>
                <button class="btn btn-warning" onclick={risky_onclick}>{&risky_button.text}</button>
                </>
            }
        }
    }
}

fn into_modal_onclick(
    onclick: Option<Callback<MouseEvent>>,
    id: impl Into<String>,
) -> Option<Callback<MouseEvent>> {
    match onclick {
        Some(onclick) => Some(onclick.merge(get_modal_close_callback(id.into()))),
        None => Some(get_modal_close_callback(id.into())),
    }
}

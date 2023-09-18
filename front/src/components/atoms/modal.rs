use deref_derive::{Deref, DerefMut};
use wasm_bindgen::JsCast;
use web_sys::HtmlDialogElement;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

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

impl Default for Buttons {
    fn default() -> Self {
        Self::Confirm(ModalButton::new("OK", None))
    }
}

#[derive(PartialEq, Clone, Store, Default, Deref, DerefMut)]
pub struct ModalStore(pub ModalData);

#[derive(PartialEq, Clone, Default)]
pub struct ModalData {
    pub title: String,
    pub message: String,
    pub buttons: Buttons,
}

const MODAL_ID: &str = "modal";

pub fn show_modal(data: ModalData, dispatch: Dispatch<ModalStore>) {
    dispatch.reduce(|_| ModalStore(data).into());
    let modal: HtmlDialogElement = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id(MODAL_ID)
        .unwrap()
        .unchecked_into();
    modal.show_modal().unwrap();
}

pub fn show_modal_callback<T>(data: ModalData, dispatch: Dispatch<ModalStore>) -> Callback<T> {
    Callback::from(move |_| show_modal(data.clone(), dispatch.clone()))
}

#[derive(PartialEq, Properties)]
pub struct ModalProps {
    pub id: String,
    pub title: String,
    pub message: String,
    pub buttons: Buttons,
}

#[function_component(Modal)]
pub fn modal() -> Html {
    let (store, _) = use_store::<ModalStore>();
    html! {
        <>
        <dialog id={MODAL_ID} class={"modal z-80"}>
        <form method={"dialog"} class={"modal-box"}>
            <h3 class={"font-bold text-lg"}>{&store.title}</h3>
            <p class={"py-4"}>{&store.message}</p>
            <div class={"flex flex-row-reverse justify-between"}>
                {get_buttons(&store.buttons)}
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
pub const ERROR_MODAL_BTN_ID: &str = "error_modal_btn";
pub const ERROR_MODAL_REDIRECTING_BTN_ID: &str = "error_modal_redirecting_btn";

#[function_component(ErrorModal)]
pub fn error_modal() -> Html {
    let navigator = use_navigator().unwrap();
    html! {
        <>
        <dialog id={ERROR_MODAL_ID} class={"modal z-100"}>
        <form method={"dialog"} class={"modal-box bg-warning text-warning-content"}>
            <h3 class={"font-bold text-lg"}>{"An error has occured!"}</h3>
            <p id={ERROR_MODAL_MSG_ID} class={"py-4"}></p>
            <div class={"flex flex-row-reverse justify-between"}>
                <button id={ERROR_MODAL_REDIRECTING_BTN_ID} class="btn btn-error" onclick={Callback::from(move |_| navigator.push(&Route::Home)).merge(get_error_close_callback())}>{"OK"}</button>
                <button id={ERROR_MODAL_BTN_ID} class="btn btn-error hidden" onclick={get_error_close_callback()}>{"OK"}</button>
            </div>
        </form>
        </dialog>
        </>
    }
}

pub fn show_error(msg: impl Into<String>, redirect: bool) {
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
    let btn = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id(ERROR_MODAL_BTN_ID)
        .unwrap();
    let redirect_btn = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id(ERROR_MODAL_REDIRECTING_BTN_ID)
        .unwrap();
    let (btn_class, redirect_btn_class) = match redirect {
        true => ("btn btn-error hidden", "btn btn-error"),
        false => ("btn btn-error", "btn btn-error hidden"),
    };
    btn.set_attribute("class", btn_class).unwrap();
    redirect_btn
        .set_attribute("class", redirect_btn_class)
        .unwrap();
    modal.show_modal().unwrap();
}

pub fn get_error_close_callback() -> Callback<MouseEvent> {
    get_close_callback(ERROR_MODAL_ID)
}

pub fn get_modal_close_callback() -> Callback<MouseEvent> {
    get_close_callback(MODAL_ID)
}

fn get_close_callback(id: &str) -> Callback<MouseEvent> {
    let id = id.to_string();
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

fn get_buttons(buttons: &Buttons) -> Html {
    match buttons.clone() {
        Buttons::Confirm(button) => {
            let onclick = into_modal_onclick(button.onclick);
            html! {
                <button class="btn btn-primary" {onclick}>{&button.text}</button>
            }
        }
        Buttons::ConfirmCancel(confirm_button, cancel_button) => {
            let confirm_onclick = into_modal_onclick(confirm_button.onclick);
            let cancel_onclick = into_modal_onclick(cancel_button.onclick);
            html! {
                <>
                <button class="btn btn-neutral" onclick={cancel_onclick}>{&cancel_button.text}</button>
                <button class="btn btn-primary" onclick={confirm_onclick}>{&confirm_button.text}</button>
                </>
            }
        }
        Buttons::RiskyCancel(risky_button, cancel_button) => {
            let risky_onclick = into_modal_onclick(risky_button.onclick);
            let cancel_onclick = into_modal_onclick(cancel_button.onclick);
            html! {
                <>
                <button class="btn btn-neutral" onclick={cancel_onclick}>{&cancel_button.text}</button>
                <button class="btn btn-warning" onclick={risky_onclick}>{&risky_button.text}</button>
                </>
            }
        }
    }
}

fn into_modal_onclick(onclick: Option<Callback<MouseEvent>>) -> Option<Callback<MouseEvent>> {
    match onclick {
        Some(onclick) => Some(onclick.merge(get_modal_close_callback())),
        None => Some(get_modal_close_callback()),
    }
}

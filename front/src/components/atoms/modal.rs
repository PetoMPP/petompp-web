use wasm_bindgen::JsCast;
use web_sys::HtmlDialogElement;
use yew::prelude::*;

use crate::utils::ext::Mergable;

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
pub enum ButtonMode {
    Confirm(ModalButton),
    ConfirmCancel(ModalButton, ModalButton),
    RiskyCancel(ModalButton, ModalButton),
}

#[derive(PartialEq)]
pub struct Button {
    pub text: String,
    pub mode: ButtonMode,
    pub onclick: Option<Callback<MouseEvent>>,
}

#[derive(PartialEq, Properties)]
pub struct ModalProps {
    pub id: String,
    pub title: String,
    pub message: String,
    pub mode: ButtonMode,
    pub children: Children,
}

#[function_component(Modal)]
pub fn modal(props: &ModalProps) -> Html {
    html! {
        <>
        {props.children.clone()}
        <dialog id={props.id.clone()} class="modal">
        <form method="dialog" class="modal-box">
            <h3 class="font-bold text-lg">{&props.title}</h3>
            <p class="py-4">{{&props.message}}</p>
            <div class="flex flex-row-reverse justify-between">
                {get_buttons(props)}
            </div>
        </form>
        </dialog>
        </>
    }
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
    match props.mode.clone() {
        ButtonMode::Confirm(button) => {
            let onclick = into_modal_onclick(button.onclick, &props.id);
            html! {
                <button class="btn btn-primary" {onclick}>{&button.text}</button>
            }
        }
        ButtonMode::ConfirmCancel(confirm_button, cancel_button) => {
            let confirm_onclick = into_modal_onclick(confirm_button.onclick, &props.id);
            let cancel_onclick = into_modal_onclick(cancel_button.onclick, &props.id);
            html! {
                <>
                <button class="btn btn-neutral" onclick={cancel_onclick}>{&cancel_button.text}</button>
                <button class="btn btn-primary" onclick={confirm_onclick}>{&confirm_button.text}</button>
                </>
            }
        }
        ButtonMode::RiskyCancel(risky_button, cancel_button) => {
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

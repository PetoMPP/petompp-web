use wasm_bindgen::JsCast;
use web_sys::HtmlDialogElement;
use yew::prelude::*;

#[derive(PartialEq, Clone)]
pub struct ModalButton(pub String, pub Option<Callback<MouseEvent>>);

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

pub fn get_modal_open_callback(id: &String) -> Callback<MouseEvent> {
    let id = id.clone();
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

pub fn get_modal_close_callback(id: &String) -> Callback<MouseEvent> {
    let id = id.clone();
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
        ButtonMode::Confirm(onclick) => {
            let close = get_modal_close_callback(&props.id);
            let text = onclick.0.clone();
            let onclick = onclick.1.unwrap_or_else(|| Callback::noop());
            let onclick = Callback::from(move |e: MouseEvent| {
                onclick.emit(e.clone());
                close.emit(e);
            });
            html! {
                <button class="btn btn-primary" onclick={onclick}>{text}</button>
            }
        }
        ButtonMode::ConfirmCancel(ok, cancel) => {
            let close = get_modal_close_callback(&props.id);
            let ok_text = ok.0.clone();
            let ok = ok.1.unwrap_or_else(|| Callback::noop());
            let ok = {
                let close = close.clone();
                Callback::from(move |e: MouseEvent| {
                    ok.emit(e.clone());
                    close.emit(e);
                })
            };
            let cancel_text = cancel.0.clone();
            let cancel = cancel.1.unwrap_or_else(|| Callback::noop());
            let cancel = Callback::from(move |e: MouseEvent| {
                cancel.emit(e.clone());
                close.emit(e);
            });
            html! {
                <>
                <button class="btn btn-neutral" onclick={cancel}>{cancel_text}</button>
                <button class="btn btn-primary" onclick={ok}>{ok_text}</button>
                </>
            }
        }
        ButtonMode::RiskyCancel(risky, cancel) => {
            let close = get_modal_close_callback(&props.id);
            let risky_text = risky.0.clone();
            let risky = risky.1.unwrap_or_else(|| Callback::noop());
            let risky = {
                let close = close.clone();
                Callback::from(move |e: MouseEvent| {
                    risky.emit(e.clone());
                    close.emit(e);
                })
            };
            let cancel_text = cancel.0.clone();
            let cancel = cancel.1.unwrap_or_else(|| Callback::noop());
            let cancel = Callback::from(move |e: MouseEvent| {
                cancel.emit(e.clone());
                close.emit(e);
            });
            html! {
                <>
                <button class="btn btn-neutral" onclick={cancel}>{cancel_text}</button>
                <button class="btn btn-warning" onclick={risky}>{risky_text}</button>
                </>
            }
        }
    }
}

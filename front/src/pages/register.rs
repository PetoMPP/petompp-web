use crate::{
    api, assign_value_event, async_event,
    components::atoms::modal::show_error,
    data::locales::{LocalesStore, TK},
    models::credentials::Credentials,
    pages::page_base::PageBase,
    router::Route,
};
use std::fmt::Display;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

#[derive(Clone, Debug, PartialEq)]
enum Error {
    Global(String),
    Username(String),
    Password(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Global(e) => write!(f, "{}", e),
            Error::Username(e) => write!(f, "{}", e),
            Error::Password(e) => write!(f, "{}", e),
        }
    }
}

#[function_component(Register)]
pub fn register() -> Html {
    let form_data = use_mut_ref(|| Credentials::default());
    let error_state = use_state_eq(|| Option::None);
    let (locales_store, _) = use_store::<LocalesStore>();
    let history = use_navigator().unwrap();

    let onchange_username = assign_value_event!(form_data.name);
    let onchange_password = assign_value_event!(form_data.password);
    let onsubmit = async_event!(
    [prevent SubmitEvent] |form_data, history, error_state| {
        match api::client::Client::register(form_data.borrow().clone()).await {
            Ok(()) => {
                error_state.set(Option::None);
                history.push(&Route::Login);
            }
            Err(error) => match error {
                api::client::Error::Endpoint(_, error) if error.starts_with("Name") => error_state.set(Some(Error::Username(error))),
                api::client::Error::Endpoint(_, error) if error.starts_with("Password") => error_state.set(Some(Error::Password(error))),
                api::client::Error::Endpoint(_, error) => error_state.set(Some(Error::Global(error))),
                e => {
                    show_error(e.to_string());
                }
            },
        }
    });
    html! {
        <PageBase>
        <form class={"form-control m-auto w-5/6 lg:w-3/4 xl:w-1/2"} {onsubmit}>
            <label class={"label"}>
                <span class={"label-text text-lg lg:text-2xl"}>{locales_store.get(TK::Register)}</span>
                <span class={"label-text-alt text-warning lg:text-lg"}>{if let Some(Error::Global(error)) = &(*error_state) {error.clone() } else { "".to_string() }}</span>
            </label>
            <label class={"label"}>
                <span class={"label-text lg:text-lg"}>{locales_store.get(TK::Username)}</span>
                <span class={"label-text-alt text-warning lg:text-lg"}>{if let Some(Error::Username(error)) = &(*error_state) {error.clone() } else { "".to_string() }}</span>
            </label>
            <input class={"input input-bordered"} placeholder={locales_store.get(TK::TypeUsername)} type={"text"} onchange={onchange_username}/>
            <label class={"label"}>
                <span class={"label-text lg:text-lg"}>{locales_store.get(TK::Password)}</span>
                <span class={"label-text-alt text-warning lg:text-lg"}>{if let Some(Error::Password(error)) = &(*error_state) {error.clone() } else { "".to_string() }}</span>
            </label>
            <input class={"input input-bordered"} placeholder={locales_store.get(TK::TypePassword)} type={"password"} onchange={onchange_password}/>
            <button class={"btn btn-primary lg:text-xl mt-4"}>{locales_store.get(TK::Register)}</button>
        </form>
        </PageBase>
    }
}

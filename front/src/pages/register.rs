use crate::api::error::{
    validation::{Error as ValidationError, UsernameError},
    ApiError as AppError,
};
use crate::components::atoms::text_input::TextInput;
use crate::{
    api::{self, client::RequestError},
    async_event,
    components::atoms::modal::show_error,
    data::locales::{LocalesStore, TK},
    models::credentials::Credentials,
    pages::page_base::PageBase,
    router::Route,
};
use std::fmt::Display;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

#[derive(Clone, Debug, PartialEq)]
enum Error {
    Username(String),
    Password(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Username(e) => write!(f, "{}", e),
            Error::Password(e) => write!(f, "{}", e),
        }
    }
}

#[function_component(Register)]
pub fn register() -> Html {
    let form_data = use_mut_ref(Credentials::default);
    let error_state = use_state_eq(|| Option::None);
    let (locales_store, _) = use_store::<LocalesStore>();
    let history = use_navigator().unwrap();

    let onchange_username = {
        let error_state = error_state.clone();
        let form_data = form_data.clone();
        Callback::from(move |e: InputEvent| {
            let target_element = e.target_unchecked_into::<HtmlInputElement>();
            form_data.borrow_mut().name = target_element.value();
            error_state.set(Option::None);
        })
    };
    let onchange_password = {
        let error_state = error_state.clone();
        let form_data = form_data.clone();
        Callback::from(move |e: InputEvent| {
            let target_element = e.target_unchecked_into::<HtmlInputElement>();
            form_data.borrow_mut().password = target_element.value();
            error_state.set(Option::None);
        })
    };
    let onsubmit = async_event!(
    [prevent SubmitEvent] |form_data, history, error_state, locales_store| {
            let creds = form_data.borrow().clone();
            match api::client::Client::register(creds).await {
            Ok(()) => {
                error_state.set(Option::None);
                history.push(&Route::Login);
            },
            Err(error) => {
                match error {
                    RequestError::Endpoint(_, error) => {
                        match &error {
                            AppError::UserNameTaken(_) => error_state.set(Some(Error::Username(error.into_localized(locales_store.clone())))),
                            AppError::ValidationError(ve) => match ve {
                                ValidationError::Username(ue) => match ue {
                                    UsernameError::InvalidLength(_, _) |
                                    UsernameError::InvalidCharacters(_) => error_state.set(Some(Error::Username(error.into_localized(locales_store.clone())))),
                                },
                                ValidationError::Password(_) => error_state.set(Some(Error::Password(error.into_localized(locales_store.clone())))),
                                _ => show_error(error.into_localized(locales_store.clone()), true),
                            },
                            _ => show_error(error.into_localized(locales_store.clone()), true),
                        }
                    }
                    RequestError::Parse(error) | RequestError::Network(error) => {
                        show_error(error, true)
                    }
                }
            }
        }
    });
    let username_error = match &*error_state {
        Some(Error::Username(error)) => Some(error.clone()),
        _ => None,
    };
    let password_error = match &*error_state {
        Some(Error::Password(error)) => Some(error.clone()),
        _ => None,
    };
    html! {
        <PageBase>
        <form class={"form-control mx-auto mt-8 lg:mt-16 w-5/6 lg:w-3/4 xl:w-1/2"} {onsubmit}>
            <label class={"label"}>
                <span class={"label-text text-lg lg:text-2xl"}>{locales_store.get(TK::Register)}</span>
            </label>
            <TextInput
                label={locales_store.get(TK::Username)} itype={"text".to_string()}
                placeholder={locales_store.get(TK::TypeUsername)} autocomplete={"username"}
                onchange={onchange_username} error={username_error}/>
            <TextInput
                label={locales_store.get(TK::Password)} itype={"password".to_string()}
                placeholder={locales_store.get(TK::TypePassword)} autocomplete={"new-password"}
                onchange={onchange_password} error={password_error}/>
            <button class={"btn btn-primary shadow-md lg:text-xl mt-4"}>{locales_store.get(TK::Register)}</button>
        </form>
        </PageBase>
    }
}

use crate::{
    api, async_event,
    components::atoms::{modal::show_error, text_input::TextInput},
    data::{
        locales::{LocalesStore, TK},
        session::SessionStore,
    },
    models::credentials::Credentials,
    pages::page_base::PageBase,
    router::Route,
};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::use_store;

#[function_component(Login)]
pub fn login() -> Html {
    let form_data = use_mut_ref(|| Credentials::default());
    let error_state = use_state_eq(|| Option::None);
    let history = use_navigator().unwrap();
    let (locales_store, _) = use_store::<LocalesStore>();
    let (_, session_dispatch) = use_store::<SessionStore>();

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
        [prevent SubmitEvent] |form_data, error_state, history, session_dispatch, locales_store| {
            match api::client::Client::login(form_data.borrow().clone()).await {
                Ok(response) => {
                    session_dispatch.reduce(|_| {
                        SessionStore {
                            token: Some(response.token),
                            user: Some(response.user),
                        }
                        .into()
                    });
                    error_state.set(Option::None);
                    history.push(&Route::Home);
                }
                Err(error) => match error {
                    api::client::ApiError::Endpoint(_, message) => error_state.set(Some(message.into_localized(locales_store.clone()))),
                    api::client::ApiError::Parse(message) | api::client::ApiError::Network(message) => {
                        show_error(message)
                    }
                },
            }
        }
    );
    html! {
        <PageBase>
        <form class={"form-control m-auto w-5/6 lg:w-3/4 xl:w-1/2"} {onsubmit}>
            <label class={"label"}>
                <span class={"label-text text-lg lg:text-2xl"}>{locales_store.get(TK::Login)}</span>
            </label>
            <TextInput
                label={locales_store.get(TK::Username)} itype={"text".to_string()}
                placeholder={locales_store.get(TK::TypeUsername)} autocomplete={"username"}
                onchange={onchange_username} error={(*error_state).clone().map(|_| String::new())}/>
            <TextInput
                label={locales_store.get(TK::Password)} itype={"password".to_string()}
                placeholder={locales_store.get(TK::TypePassword)} autocomplete={"current-password"}
                onchange={onchange_password} error={(*error_state).clone()}/>
            <button class={"btn btn-primary shadow-md lg:text-xl mt-4"}>{locales_store.get(TK::Login)}</button>
        </form>
        </PageBase>
    }
}

use crate::pages::page_base::EditablePage;
use yew::prelude::*;

#[function_component(Contact)]
pub fn contact() -> Html {
    const RES_KEY: &str = "contact-content";
    html! {
        <EditablePage reskey={RES_KEY.to_string()}/ >
    }
}

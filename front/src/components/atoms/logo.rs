use crate::router::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Logo)]
pub fn logo() -> Html {
    let navigator = use_navigator().unwrap();
    let onclick = Callback::from(move |_| navigator.push(&Route::Home));
    html! {
        <button class={"btn btn-ghost text-xl"} {onclick}>{"PetoMPP.NET"}</button>
    }
}

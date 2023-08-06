use crate::router::Route;
use stylist::yew::styled_component;
use yew::prelude::*;
use yew_router::prelude::*;

/// A component to display a user's information.
/// Or login/register if not logged in.
#[styled_component(UserBox)]
pub fn user_box() -> Html {
    let class = classes!(
        "flex",
        "grow",
        "items-center",
        "px-1",
        "-from-20%",
        "hover:bg-gradient-to-r",
        "hover:from-blue-500",
        "hover:to-cyan-300",
        "bg-gradient-to-r",
        "from-blue-300",
        "to-cyan-200",
        "rounded-md",
        "shadow-sm"
    );
    html! {
        <div class={"flex flex-col text-lg md:text-2xl my-1 gap-1"}>
            <button class={class.clone()}><Link<Route> to={Route::Login}>{"Login"}</Link<Route>></button>
            <button class={class.clone()}><Link<Route> to={Route::Register}>{"Register"}</Link<Route>></button>
        </div>
    }
}

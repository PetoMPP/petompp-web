use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::Route;

#[function_component(Logo)]
pub fn logo() -> Html {
    let banner_class = classes!(
        "flex",
        "items-center",
        "justify-center",
        "bg-gradient-to-r",
        "from-blue-300",
        "via-blue-400",
        "via-25%",
        "to-cyan-300",
        "px-2",
        "mt-1",
        "text-xl",
        "font-semibold",
        "rounded-lg",
        "shadow-md"
    );
    html! {
        <Link<Route> to={Route::Home} classes={banner_class}>{"PetoMPP.NET"}</Link<Route>>
    }
}
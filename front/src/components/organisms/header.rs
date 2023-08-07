use super::user_box::UserBox;
use stylist::yew::styled_component;
use yew::prelude::*;

#[styled_component(Header)]
pub fn header() -> Html {
    let class = classes!("w-full", "flex", "gap-2",);
    let banner_class = classes!(
        "flex",
        "grow",
        "items-center",
        "justify-center",
        "bg-gradient-to-r",
        "from-cyan-300",
        "via-blue-500",
        "to-cyan-300",
        "py-4",
        "my-1",
        "text-2xl",
        "md:text-5xl",
        "rounded-lg",
        "shadow-md"
    );
    html! {
        <div {class}>
            <div class={banner_class}>{"PetoMPP.NET"}</div>
            <UserBox/>
        </div>
    }
}

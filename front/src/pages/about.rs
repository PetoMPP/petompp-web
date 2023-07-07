use stylist::yew::styled_component;
use yew::prelude::*;

use crate::pages::page_base::PageBase;

#[styled_component(About)]
pub fn about() -> Html {
    html! {
        <PageBase>
            <h1>{"About"}</h1>
            <p>{"This is a page about me"}</p>
            <p>{"I'm a software engineer"}</p>
            <p>{"I'm also interested in learning more about the web"}</p>
            <p>{"If you want to know more about me, feel free to contact me"}</p>
        </PageBase>
    }
}

use yew::prelude::*;

use crate::pages::page_base::PageBase;

#[function_component(Projects)]
pub fn projects() -> Html {
    html! {
        <PageBase>
            <p class={"text-xl"}>{"Projects"}</p>
            <p class={"text-sm"}>{"This is where I will put my projects."}</p>
        </PageBase>
    }
}

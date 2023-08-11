use crate::pages::page_base::PageBase;
use yew::prelude::*;

#[function_component(NotFound)]
pub fn admin_panel() -> Html {
    html! {
        <PageBase>
            <div class="text-6xl font-bold font-mono flex justify-center">{"404"}</div>
        </PageBase>
    }
}

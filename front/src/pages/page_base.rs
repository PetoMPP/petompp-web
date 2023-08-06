use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct PageBaseProps {
    pub children: Children,
}

#[styled_component(PageBase)]
pub fn page_base(props: &PageBaseProps) -> Html {
    html! {
        <div class={"flex flex-col w-full my-1 p-3 rounded-xl border border-cyan-400 bg-white"}>
            {props.children.clone()}
        </div>
    }
}

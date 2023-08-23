use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct PageBaseProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(PageBase)]
pub fn page_base(props: &PageBaseProps) -> Html {
    html! {
        <div class={"animate-fade-up flex flex-col w-full my-1 p-3 rounded-xl border border-primary bg-base-100"}>
                {props.children.clone()}
        </div>
    }
}

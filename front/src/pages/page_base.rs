use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct PageBaseProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(PageBase)]
pub fn page_base(props: &PageBaseProps) -> Html {
    html! {
        <div class={"animate-fade-up flex flex-col w-full min-h-[20rem] lg:min-h-[40rem] mt-10 lg:mt-24 mb-6 p-3 rounded-xl border border-primary bg-base-100"}>
                {props.children.clone()}
        </div>
    }
}

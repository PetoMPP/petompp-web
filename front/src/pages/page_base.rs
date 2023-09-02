use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct PageBaseProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(PageBase)]
pub fn page_base(props: &PageBaseProps) -> Html {
    html! {
        <div class={"animate-fade-up flex flex-col mt-10 lg:mt-20 min-h-[40rem] w-full mb-6 p-8 rounded-t-xl bg-base-100"}>
                {props.children.clone()}
        </div>
    }
}

use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct BlogPostSummaryProps {
    pub summary: crate::models::blog::BlogPostSummary,
}

#[function_component(BlogPostSummary)]
pub fn blog_post_summary(props: &BlogPostSummaryProps) -> Html {
    let tags = props.summary
        .tags
        .iter()
        .map(|tag| {
            html! {
                <div class={"badge badge-outilne badge-primary"}>{tag}</div>
            }
        })
        .collect::<Html>();
    let style =
        "-webkit-mask-image: -webkit-linear-gradient(left, rgba(0,0,0,0),rgba(0,0,0,1));";
    html! {
    <div class={"card card-side bg-base-200"}>
        <div class={"card-body"}>
            <div class={"flex flex-col gap-1 justify-between"}>
                <h2 class={"card-title"}>{&props.summary.title}</h2>
                <div class={"flex flex-row gap-2"}>
                    {tags}
                </div>
            </div>
            <div class={"card-actions flex flex-col"}>
                <div class={"divider"}/>
                <p>{&props.summary.summary}</p>
            </div>
            </div>
        <figure class={"absolute right-0 h-full w-2/3 object-fill"} {style}>
            <img class={"rounded-xl min-h-full"} src={"/img/placeholder.svg"}/>
        </figure>
      </div>
    }
}

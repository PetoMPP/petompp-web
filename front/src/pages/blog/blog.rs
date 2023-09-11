use crate::{components::organisms::blog_post::BlogPostSummary, pages::page_base::PageBase};
use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct BlogProps {
    pub tag: Option<String>,
}

#[function_component(Blog)]
pub fn blog(props: &BlogProps) -> Html {
    gloo::console::log!(props.tag.as_ref().map(|s| s.as_str()));
    let posts = vec![
        crate::models::blog::BlogPostSummary::new(
            "Post 1".to_string(),
            "This is the first post in the blog. It is about tag1 and tag2.".to_string(),
            vec!["tag1".to_string(), "tag2".to_string()],
        ),
        crate::models::blog::BlogPostSummary::new(
            "Post 2".to_string(),
            "This is the second post in the blog. It is about tag2 and tag3.".to_string(),
            vec!["tag2".to_string(), "tag3".to_string()],
        ),
        crate::models::blog::BlogPostSummary::new(
            "Post 3".to_string(),
            "This is the third post in the blog. It is about tag3 and tag4.".to_string(),
            vec!["tag3".to_string(), "tag4".to_string()],
        ),
    ];
    let posts = match &props.tag {
        Some(tag) => posts
            .iter()
            .filter(|summary| summary.tags.contains(tag))
            .collect::<Vec<_>>(),
        None => posts.iter().collect::<Vec<_>>(),
    };
    let posts = posts
        .iter()
        .map(|summary| {
            html! {
                <BlogPostSummary summary={(**summary).clone()}/>
            }
        })
        .collect::<Html>();
    html! {
        <PageBase>
        <div class={"prose"}>
            <h1>{"Blog"}</h1>
            <p>{"This is the blog page."}</p>
        </div>
        <div class={"flex flex-col gap-2"}>
            {posts}
        </div>
        </PageBase>
    }
}

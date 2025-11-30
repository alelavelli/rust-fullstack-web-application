use yew::{Html, Properties, function_component, html};

use crate::model::BlogPost;

#[derive(Properties, PartialEq)]
pub struct PostDetailProps {
    pub post: BlogPost,
}

#[function_component(PostDetails)]
pub fn post_details(PostDetailProps { post }: &PostDetailProps) -> Html {
    html! {
        <div>
            <p>{post.title.clone()}</p>
            <p>{format!("written by {} on {}", post.creator_username, post.creation_date)}</p>
            <p>{post.content.clone()}</p>
        </div>
    }
}

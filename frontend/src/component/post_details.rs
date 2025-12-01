use yew::{Html, Properties, function_component, html};

use crate::model::BlogPost;

#[derive(Properties, PartialEq)]
pub struct PostDetailProps {
    pub post: BlogPost,
}

#[function_component(PostDetails)]
pub fn post_details(PostDetailProps { post }: &PostDetailProps) -> Html {
    html! {
        <div class="blog-post-details">
            <h4>{post.title.clone()}</h4>
            <i>{format!("written by {} on {}", post.creator_username, post.creation_date)}</i>
            <p class="blog-post-content">{post.content.clone()}</p>
        </div>
    }
}

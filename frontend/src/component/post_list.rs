use yew::{Callback, Html, Properties, function_component, html};

use crate::model::BlogPost;

#[derive(Properties, PartialEq)]
pub struct PostsListProp {
    pub posts: Vec<BlogPost>,
    pub on_click: Callback<BlogPost>,
}

#[function_component(PostsList)]
pub fn posts_list(PostsListProp { posts, on_click }: &PostsListProp) -> Html {
    let on_click = on_click.clone();

    let html_posts = posts.iter().map(|post| {
        let on_post_select = {
            let on_click = on_click.clone();
            let post = post.clone();
            Callback::from(move |_| {
                on_click.emit(post.clone())
            })
        };

        html!{
            <li key={post.id.clone()} onclick={on_post_select} class="post-list-entry">{format!("{} written by {}", post.title, post.creator_username)}</li>
        }
    }).collect::<Html>();

    html! {
        <ul class="post-list">
            {html_posts}
        </ul>
    }
}

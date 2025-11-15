use crate::model::BlogPost;
use gloo_net::http::Request;
use yew::{Callback, Html, Properties, function_component, html, use_effect_with, use_state};

#[derive(Properties, PartialEq)]
struct PostDetailProps {
    post: BlogPost,
}

#[function_component(PostDetail)]
fn post_details(PostDetailProps { post }: &PostDetailProps) -> Html {
    html! {
        <div>
            <p>{post.title.clone()}</p>
            <p>{format!("written by {} on {}", post.creator_username, post.creation_date)}</p>
            <p>{post.content.clone()}</p>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct PostsListsProp {
    posts: Vec<BlogPost>,
    on_click: Callback<BlogPost>,
}

#[function_component(PostsList)]
fn posts_list(PostsListsProp { posts, on_click }: &PostsListsProp) -> Html {
    let on_click = on_click.clone();

    posts.iter().map(|post| {
        let on_post_select = {
            let on_click = on_click.clone();
            let post = post.clone();
            Callback::from(move |_| {
                on_click.emit(post.clone())
            })
        };

        html! {
            <p key={post.id.clone()} onclick={on_post_select}>{format!("{} written by {}", post.title, post.creator_username)}</p>
        }
    }).collect::<Html>()
}

#[function_component(App)]
pub fn app() -> Html {
    let _blog_posts_mock = vec![
        BlogPost {
            id: "1".into(),
            title: "First blog".into(),
            content: "this is the content of the blog. I think that I could write something but for now I can simply write a long text just to see how it will be displayed on the screen. Who know how it will be printed on the screen.".into(),
            creation_date: "2025/11/14".into(),
            creator_id: "creator-id".into(),
            creator_username: "alex_sinks".into()
        },
        BlogPost {
            id: "2".into(),
            title: "Second blog".into(),
            content: "this is the content of the blog. I think that I could write something but for now I can simply write a long text just to see how it will be displayed on the screen. Who know how it will be printed on the screen.".into(),
            creation_date: "2025/11/14".into(),
            creator_id: "creator-id".into(),
            creator_username: "alex_sinks".into()
        },
        BlogPost {
            id: "3".into(),
            title: "Third blog".into(),
            content: "this is the content of the blog. I think that I could write something but for now I can simply write a long text just to see how it will be displayed on the screen. Who know how it will be printed on the screen.".into(),
            creation_date: "2025/11/14".into(),
            creator_id: "creator-id".into(),
            creator_username: "alex_sinks".into()
        }
    ];

    let blog_posts = use_state(|| vec![]);
    {
        let blog_posts = blog_posts.clone();
        use_effect_with((), move |_| {
            let blog_posts = blog_posts.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_posts: Vec<BlogPost> =
                    Request::get("http://localhost:3000/api/user/blog/post")
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();
                blog_posts.set(fetched_posts);
            });
            || ()
        });
    }

    let selected_post = use_state(|| None);

    let on_post_select = {
        let selected_post = selected_post.clone();
        Callback::from(move |post: BlogPost| selected_post.set(Some(post)))
    };

    let details = selected_post.as_ref().map(|post| {
        html! {
            <PostDetail post={post.clone()} />
        }
    });

    html! {
        <main>
            <h1>{ "Hello Blog!" }</h1>
            <h2>{ "Your personal blog written totally in Rust ;)" }</h2>
            <div>
                <h3>{"Here the list of published posts:"}</h3>
                <PostsList posts={(*blog_posts).clone()} on_click={on_post_select.clone()}/>
            </div>
            {for details}
        </main>
    }
}

use log::error;
use yew::{
    Callback, Html, UseStateHandle, function_component, html, use_context, use_effect_with,
    use_state,
};
use yew_router::prelude::Redirect;

use crate::{
    app::AppRoute,
    component::{post_details::PostDetails, post_list::PostsList},
    enums::HttpStatus,
    environment::EnvironmentService,
    model::{BlogPost, LoggedUserInfo},
    service::api::ApiService,
    types::{ApiResponse, AppContext},
};

#[function_component(Home)]
pub fn home_component() -> Html {
    let app_context = use_context::<UseStateHandle<AppContext>>().expect("No app_context found");
    let user_info: UseStateHandle<Option<LoggedUserInfo>> = use_state(|| None);
    let blog_posts: UseStateHandle<Vec<BlogPost>> = use_state(|| vec![]);
    let blog_post_error = use_state(|| None);
    let selected_post = use_state(|| None);

    {
        let blog_posts = blog_posts.clone();
        use_effect_with(user_info.clone(), move |user_info| {
            let blog_posts = blog_posts.clone();
            let user_info = user_info.clone();
            if let Some(user_info) = (*user_info).clone() {
                wasm_bindgen_futures::spawn_local(async move {
                    let environment_service = EnvironmentService::new();
                    let api_service = ApiService::new(
                        environment_service.api_url,
                        environment_service.mock,
                        Some(user_info.token),
                    );

                    let blog_post_response = api_service.get_posts().await;
                    if let Ok(ApiResponse { body, status }) = blog_post_response {
                        match status {
                            HttpStatus::Success(_) => {
                                blog_posts.set(body);
                                blog_post_error.set(None);
                            }
                            _ => {
                                blog_post_error.set(Some("Ops, something went wrong".into()));
                            }
                        }
                    } else {
                        error!(
                            "Encountered an error in get posts request. Error {err}",
                            err = blog_post_response.err().unwrap()
                        );
                        blog_post_error.set(Some("Got error from backend".to_string()));
                    }
                });
            }
        });
    }

    if let Some(context_user_info) = app_context.user_info.clone() {
        if user_info.is_none() {
            user_info.set(Some(context_user_info));
        }

        let blog_posts = blog_posts.clone();

        let on_post_select = {
            let selected_post = selected_post.clone();
            Callback::from(move |post: BlogPost| selected_post.set(Some(post)))
        };

        let details = selected_post.as_ref().map(|post| {
            html! {
                <PostDetails post={post.clone()}/>
            }
        });

        html! {
            <>
                <div>
                <h1>{ "Hello Blog!" }</h1>
                <h2>{ "Your personal blog written totally in Rust ;)" }</h2>
                </div>
                <div>
                    <h3>{"Here the list of published posts:"}</h3>
                    <PostsList posts={(*blog_posts).clone()} on_click={on_post_select}/>
                </div>
                {for details}
            </>
        }
    } else {
        html! {
            <Redirect<AppRoute> to={AppRoute::Login}/>
        }
    }
}

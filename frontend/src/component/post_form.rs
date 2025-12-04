use log::{error, info};
use web_sys::HtmlInputElement;
use yew::{
    Callback, Html, Properties, SubmitEvent, UseStateHandle, function_component, html,
    use_node_ref, use_state,
};

use crate::{
    enums::HttpStatus, environment::EnvironmentService, model::LoggedUserInfo,
    service::api::ApiService, types::ApiResponse,
};

#[derive(Properties, PartialEq)]
pub struct PostFormProp {
    pub user_info: LoggedUserInfo,
    pub on_close: Callback<()>,
}

#[function_component(PostForm)]
pub fn post_form(
    PostFormProp {
        user_info,
        on_close,
    }: &PostFormProp,
) -> Html {
    let on_close_internal = {
        let on_close = on_close.clone();
        Callback::from(move |_| on_close.emit(()))
    };

    let title_node_ref = use_node_ref();
    let content_node_ref = use_node_ref();
    let form_error: UseStateHandle<Option<String>> = use_state(|| None);

    let onsubmit = {
        let title_node_ref = title_node_ref.clone();
        let content_node_ref = content_node_ref.clone();
        let form_error = form_error.clone();
        let user_info = user_info.clone();
        let on_close = on_close.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let user_info = user_info.clone();
            let form_error = form_error.clone();
            let on_close = on_close.clone();

            let title: Option<String> = title_node_ref
                .cast::<HtmlInputElement>()
                .map(|title| title.value());
            let content: Option<String> = content_node_ref
                .cast::<HtmlInputElement>()
                .map(|content| content.value());

            if let (Some(title), Some(content)) = (title, content) {
                info!(
                    "title {title} and content {content}",
                    title = title,
                    content = content
                );
                wasm_bindgen_futures::spawn_local(async move {
                    let environment_service = EnvironmentService::new();
                    let api_service = ApiService::new(
                        environment_service.api_url,
                        environment_service.mock,
                        Some(user_info.token),
                    );
                    let api_response: Result<ApiResponse<()>, crate::error::ApiError> =
                        api_service.publish_post(title, content).await;

                    if let Ok(ApiResponse { body: _, status }) = api_response {
                        match status {
                            HttpStatus::Success(_) => {
                                on_close.emit(());
                                form_error.set(None);
                            }
                            _ => {
                                form_error.set(Some(format!("Got error from backend: {status}")));
                            }
                        }
                    } else {
                        error!(
                            "Encountered an error in publish post request. Error {err}",
                            err = api_response.err().unwrap()
                        );
                        form_error.set(Some("Got error from backend".to_string()));
                    }
                });
            }
        })
    };

    html! {
        <div class="blog-post-form">
            <h4>{"Write a new blog post"}</h4>
            <form class="form-container" {onsubmit}>
                <div class="blog-post-form-input-container">
                    <input placeholder="title" ref={title_node_ref}/>
                    <textarea type="text" placeholder="content" ref={content_node_ref}/>
                    if let Some(error_msg) = (*form_error).clone() {
                        <p style="color:red">{error_msg}</p>
                    }
                </div>
                <div class="blog-post-form-action-container">
                    <button class="form-button-primary" type="submit">{"Publish"}</button>
                    <button class="form-button-secondary" onclick={on_close_internal}>{"Cancel"}</button>
                </div>
            </form>
        </div>
    }
}

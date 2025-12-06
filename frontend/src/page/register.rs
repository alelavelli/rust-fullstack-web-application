use log::error;
use web_sys::HtmlInputElement;
use yew::{
    Callback, Html, SubmitEvent, UseStateHandle, function_component, html, use_context,
    use_node_ref, use_state,
};
use yew_router::prelude::Redirect;

use crate::{
    app::AppRoute,
    enums::HttpStatus,
    environment::EnvironmentService,
    service::{api::ApiService, auth::AuthService},
    types::{ApiResponse, AppContext},
};

#[function_component(Register)]
pub fn register_component() -> Html {
    /* According to documentation https://yew.rs/docs/concepts/html/events we use NodeRef
    We attach them to input elements and they will be used by the onsubmit function to
    retrieve the values.
    */
    let first_name_nr = use_node_ref();
    let last_name_nr = use_node_ref();
    let username_nr = use_node_ref();
    let password_nr = use_node_ref();
    let confirm_password_nr = use_node_ref();

    let app_context = use_context::<UseStateHandle<AppContext>>().expect("No app_context found");
    let app_context_clone = app_context.clone();

    // variable to display error message if something when wrong
    let request_error: UseStateHandle<Option<String>> = use_state(|| None);

    let onsubmit = {
        let first_name_nr = first_name_nr.clone();
        let last_name_nr = last_name_nr.clone();
        let username_nr = username_nr.clone();
        let password_nr = password_nr.clone();
        let confirm_password_nr = confirm_password_nr.clone();

        let request_error = request_error.clone();

        Callback::from(move |e: SubmitEvent| {
            // the callback gets the values from the input elements
            // if they are not None then a backend request is done
            e.prevent_default();

            let first_name = first_name_nr
                .cast::<HtmlInputElement>()
                .map(|first_name| first_name.value());
            let last_name = last_name_nr
                .cast::<HtmlInputElement>()
                .map(|last_name| last_name.value());
            let username = username_nr
                .cast::<HtmlInputElement>()
                .map(|username| username.value());
            let password = password_nr
                .cast::<HtmlInputElement>()
                .map(|password| password.value());
            let confirm_password = confirm_password_nr
                .cast::<HtmlInputElement>()
                .map(|confirm_password| confirm_password.value());

            let request_error = request_error.clone();
            let app_context = app_context_clone.clone();

            wasm_bindgen_futures::spawn_local(async move {
                if let (
                    Some(first_name),
                    Some(last_name),
                    Some(username),
                    Some(password),
                    Some(confirm_password),
                ) = (first_name, last_name, username, password, confirm_password)
                    && !first_name.trim().is_empty()
                    && !last_name.trim().is_empty()
                    && !username.trim().is_empty()
                    && !password.trim().is_empty()
                    && !confirm_password.trim().is_empty()
                {
                    if password != confirm_password {
                        request_error.set(Some("Passwords must be the same".to_string()));
                        return;
                    }
                    let environment_service = EnvironmentService::new();
                    let api_service = ApiService::new(
                        environment_service.api_url,
                        environment_service.mock,
                        None,
                    );

                    let register_response = api_service
                        .register(first_name, last_name, username, password)
                        .await;
                    if let Ok(ApiResponse { body, status }) = register_response {
                        match status {
                            HttpStatus::Success(_) => {
                                if let Some(body) = body {
                                    AuthService::new(
                                        environment_service.token_storage_location_name,
                                        app_context,
                                    )
                                    .set_logged_user_info(body)
                                    .expect("Failed to store token");

                                    request_error.set(None);
                                } else {
                                    // if the body is None then it is an internal error
                                    request_error
                                        .set(Some(String::from("Ops, something went wrong.")));
                                }
                            }
                            _ => {
                                request_error
                                    .set(Some(format!("Got error from backend: {status}")));
                            }
                        }
                    } else {
                        error!(
                            "Encountered an error in login request. Error {err}",
                            err = register_response.err().unwrap()
                        );
                        request_error.set(Some("Got error from backend".to_string()));
                    }
                }
            });
        })
    };

    if app_context.user_info.is_some() {
        html! {
            <Redirect<AppRoute> to={AppRoute::Home}/>
        }
    } else {
        html! {
            <div class="page-centered">
                <h1>{"Welcome to Hello Blog!"}</h1>
                <div>{"Please insert your credentials and log in."}</div>
                <form class="form-container" {onsubmit}>
                        <input type="text" placeholder="first name" ref={first_name_nr}/>
                        <input type="text" placeholder="last name" ref={last_name_nr}/>
                        <input type="text" placeholder="username" ref={username_nr}/>
                        <input type="password" placeholder="password" ref={password_nr}/>
                        <input type="password" placeholder="confirm password" ref={confirm_password_nr}/>
                    <button type="submit" class="form-button-primary">{"Register"}</button>
                    if let Some(error_msg) = (*request_error).clone() {
                        <p style="color:red">{error_msg}</p>
                    }
                </form>
            </div>
        }
    }
}

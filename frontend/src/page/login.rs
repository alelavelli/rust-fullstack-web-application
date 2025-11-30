use log::{error, info};
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

#[function_component(Login)]
pub fn login_component() -> Html {
    /* According to documentation https://yew.rs/docs/concepts/html/events we use NodeRef
    We attach them to input elements and they will be used by the onsubmit function to
    retrieve the values.
    */
    let username_node_ref = use_node_ref();
    let password_node_ref = use_node_ref();

    let app_context = use_context::<UseStateHandle<AppContext>>().expect("No app_context found");
    let app_context_clone = app_context.clone();

    // variable to display error message if something when wrong
    let login_error: UseStateHandle<Option<String>> = use_state(|| None);
    // this clone is needed to be passed to onsubmit callback
    let login_error_clone = login_error.clone();

    let onsubmit = {
        let username_node_ref = username_node_ref.clone();
        let password_node_ref = password_node_ref.clone();

        Callback::from(move |e: SubmitEvent| {
            // the callback gets the values from the input elements
            // if they are not None then a backend request is done
            e.prevent_default();

            let username = username_node_ref
                .cast::<HtmlInputElement>()
                .map(|username| username.value());
            let password = password_node_ref
                .cast::<HtmlInputElement>()
                .map(|password| password.value());

            let login_error = login_error_clone.clone();
            let app_context = app_context_clone.clone();

            wasm_bindgen_futures::spawn_local(async move {
                if let (Some(username), Some(password)) = (username, password) {
                    if !username.trim().is_empty() && !password.trim().is_empty() {
                        let environment_service = EnvironmentService::new();
                        let api_service = ApiService::new(
                            environment_service.api_url,
                            environment_service.mock,
                            None,
                        );
                        // make the request, if the type is Ok then set the token and update the app context
                        // if it is error then update the error boolean variable
                        let logged_user_info = api_service.login(username, password).await;

                        if let Ok(ApiResponse { body, status }) = logged_user_info {
                            match status {
                                HttpStatus::Success(_) => {
                                    if let Some(body) = body {
                                        AuthService::new(
                                            environment_service.token_storage_location_name,
                                            app_context,
                                        )
                                        .set_logged_user_info(body)
                                        .expect("Failed to store token");

                                        login_error.set(None);
                                    } else {
                                        // if the body is None then it is an internal error
                                        login_error
                                            .set(Some(String::from("Ops, something went wrong.")));
                                    }
                                }
                                _ => {
                                    login_error
                                        .set(Some(format!("Got error from backend: {status}")));
                                }
                            }
                        } else {
                            error!(
                                "Encountered an error in login request. Error {err}",
                                err = logged_user_info.err().unwrap()
                            );
                            login_error.set(Some("Got error from backend".to_string()));
                        }
                    }
                }
            });
        })
    };

    if app_context.user_info.is_some() {
        info!("login: redirect to home");
        html! {
            <Redirect<AppRoute> to={AppRoute::Home}/>
        }
    } else {
        html! {
            <div class="page-centered">
                <h1>{"Welcome to Hello Blog!"}</h1>
                <div>{"Please insert your credentials and log in."}</div>
                <form class="form-container" {onsubmit}>
                    <fieldset>
                        <input type="text" placeholder="username" ref={username_node_ref}/>
                    </fieldset>
                    <fieldset>
                        <input type="password" placeholder="password" ref={password_node_ref}/>
                    </fieldset>
                    <button type="submit">{"Login"}</button>
                    if let Some(error_msg) = (*login_error).clone() {
                        <p style="color:red">{error_msg}</p>
                    }
                </form>
            </div>
        }
    }
}

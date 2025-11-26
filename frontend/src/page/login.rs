use log::error;
use web_sys::HtmlInputElement;
use yew::{
    Callback, Html, SubmitEvent, UseStateHandle, function_component, html, use_context,
    use_node_ref, use_state,
};

use crate::{
    service::auth::AuthService,
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

    // we extract the application context because we need the api service to make login request
    let app_context = use_context::<UseStateHandle<AppContext>>().expect("No app_context found");

    // boolean variable to display error message if something when wrong
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
            let app_context = app_context.clone();

            wasm_bindgen_futures::spawn_local(async move {
                if let (Some(username), Some(password)) = (username, password) {
                    if !username.trim().is_empty() && !password.trim().is_empty() {
                        // make the request, if the type is Ok then set the token and update the app context
                        // if it is error then update the error boolean variable
                        let logged_user_info =
                            app_context.api_service.login(username, password).await;

                        if let Ok(ApiResponse { body, status }) = logged_user_info {
                            if status == 200 {
                                if let Some(body) = body {
                                    AuthService::new()
                                        .set_token(body.token.clone())
                                        .expect("Failed to store token");

                                    app_context.set(AppContext {
                                        environment_service: app_context
                                            .environment_service
                                            .clone(),
                                        api_service: app_context.api_service.clone(),
                                        user_info: Some(body),
                                    });
                                    login_error.set(None);
                                } else {
                                    // if the body is None then it is an internal error
                                    login_error
                                        .set(Some(String::from("Ops, something went wrong.")));
                                }
                            } else {
                                login_error.set(Some(format!("Got error from backend: {status}")));
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

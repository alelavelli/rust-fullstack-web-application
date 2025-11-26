use log::{error, info};
use web_sys::HtmlInputElement;
use yew::{
    Callback, Html, SubmitEvent, UseStateHandle, function_component, html, use_context,
    use_effect_with, use_node_ref, use_state,
};

use crate::{model::LoginInfo, service::auth::AuthService, types::AppContext};

#[function_component(Login)]
pub fn login_component() -> Html {
    /* According to documentation https://yew.rs/docs/concepts/html/events we use NodeRef
    We attach them to input elements and they will be used by the onsubmit function to
    retrieve the values.
    */
    let username_node_ref = use_node_ref();
    let password_node_ref = use_node_ref();

    let app_context = use_context::<UseStateHandle<AppContext>>().expect("No app_context found");

    let login_info = use_state(|| LoginInfo {
        username: None,
        password: None,
    });
    let login_error = use_state(|| false);
    let login_error_clone = login_error.clone();

    /* use_effect_with(login_info.clone(), move |login_info| {
        let login_info = login_info.clone();
        let login_error = login_error_clone.clone();
        let app_context = app_context.clone();
        wasm_bindgen_futures::spawn_local(async move {
            if login_info.username.is_some() && login_info.password.is_some() {
                let username = login_info.username.as_ref().unwrap().clone();
                let password = login_info.password.as_ref().unwrap().clone();

                let logged_user_info = app_context.api_service.login(username, password).await;

                if let Ok(logged_user_info) = logged_user_info {
                    AuthService::new()
                        .set_token(logged_user_info.token.clone())
                        .expect("Failed to store token");

                    app_context.set(AppContext {
                        environment_service: app_context.environment_service.clone(),
                        api_service: app_context.api_service.clone(),
                        user_info: Some(logged_user_info),
                    });
                    login_error.set(false);
                } else {
                    error!("Encountered an error in login request");
                    login_error.set(true);
                }
            }
        });
    }); */

    let onsubmit = {
        let username_node_ref = username_node_ref.clone();
        let password_node_ref = password_node_ref.clone();
        let login_info = login_info.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            let username = if let Some(username) = username_node_ref.cast::<HtmlInputElement>() {
                Some(username.value())
            } else {
                None
            };

            let password = if let Some(password) = password_node_ref.cast::<HtmlInputElement>() {
                Some(password.value())
            } else {
                None
            };
            info!("before set");
            login_info.set(LoginInfo { username, password });
            info!("after set");
            let login_info = login_info.clone();
            let login_error = login_error_clone.clone();
            let app_context = app_context.clone();
            info!("before spawn");
            wasm_bindgen_futures::spawn_local(async move {
                if login_info.username.is_some() && login_info.password.is_some() {
                    let username = login_info.username.as_ref().unwrap().clone();
                    let password = login_info.password.as_ref().unwrap().clone();

                    let logged_user_info = app_context.api_service.login(username, password).await;

                    if let Ok(logged_user_info) = logged_user_info {
                        AuthService::new()
                            .set_token(logged_user_info.token.clone())
                            .expect("Failed to store token");

                        app_context.set(AppContext {
                            environment_service: app_context.environment_service.clone(),
                            api_service: app_context.api_service.clone(),
                            user_info: Some(logged_user_info),
                        });
                        login_error.set(false);
                    } else {
                        error!("Encountered an error in login request");
                        login_error.set(true);
                    }
                }
            });
            info!("after spawn");
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
                if *login_error {
                    <p style="color:red">{"Ops, something went wrong"}</p>
                }
            </form>
        </div>
    }
}

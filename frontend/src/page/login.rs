use web_sys::HtmlInputElement;
use yew::{Callback, Html, SubmitEvent, function_component, html, use_context, use_node_ref};

use crate::types::AppContext;

#[function_component(Login)]
pub fn login_component() -> Html {
    /* According to documentation https://yew.rs/docs/concepts/html/events we use NodeRef
    We attach them to input elements and they will be used by the onsubmit function to
    retrieve the values.
    */
    let username_node_ref = use_node_ref();
    let password_node_ref = use_node_ref();

    let app_context = use_context::<AppContext>().expect("No app_context found");

    let onsubmit = {
        let username_node_ref = username_node_ref.clone();
        let password_node_ref = password_node_ref.clone();

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

            if username.is_some() && password.is_some() {
                app_context
                    .api_service
                    .login(username.unwrap(), password.unwrap());
            }
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
            </form>
        </div>
    }
}

use yew::{Html, UseStateHandle, function_component, html, use_context};

use crate::types::AppContext;

#[function_component(Header)]
pub fn header_component() -> Html {
    let app_context = use_context::<UseStateHandle<AppContext>>().expect("No app_context found");
    let logged_user = app_context.user_info.is_some();

    html! {
        <header>
            <h1>{ "Hello Blog!" }</h1>
            if logged_user {
                <p>{"logged user!"}</p>
            } else {
                <p>{"no user!"}</p>
            }
        </header>
    }
}

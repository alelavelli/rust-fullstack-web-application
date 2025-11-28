use yew::{Html, UseStateHandle, function_component, html, use_context};

use crate::types::AppContext;

#[function_component(Header)]
pub fn header_component() -> Html {
    let app_context = use_context::<UseStateHandle<AppContext>>().expect("No app_context found");

    html! {
        <header>
            <h1>{ "Hello Blog!" }</h1>
            if let Some(user_info) = &app_context.user_info {
                <p>{format!("Hi {}!", user_info.username)}</p>
            } else {
                <p>{"no user!"}</p>
            }
        </header>
    }
}

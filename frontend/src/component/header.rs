use yew::{Callback, Html, UseStateHandle, function_component, html, use_context};
use yew_router::prelude::Link;

use crate::{
    app::AppRoute, environment::EnvironmentService, service::auth::AuthService, types::AppContext,
};

#[function_component(Header)]
pub fn header_component() -> Html {
    let app_context = use_context::<UseStateHandle<AppContext>>().expect("No app_context found");

    let onclick = {
        let app_context = app_context.clone();
        Callback::from(move |_| {
            let environment_service = EnvironmentService::new();
            let auth_service = AuthService::new(
                environment_service.token_storage_location_name,
                app_context.clone(),
            );
            auth_service.remove_logged_user();
        })
    };

    html! {
        <header>
            <h1>
                <Link<AppRoute> to={AppRoute::Home} classes="header-link"> { "Hello Blog!" } </Link<AppRoute>>
            </h1>
            if let Some(user_info) = &app_context.user_info {
                <div>
                    <p>{format!("Hi {}!", user_info.username)}</p>
                    <button {onclick} class="header-link">{"Logout"}</button>
                </div>
            } else {
                <Link<AppRoute> to={AppRoute::Login} classes="header-link"> {"Login"} </Link<AppRoute>>
            }
        </header>
    }
}

use yew::{Html, UseStateHandle, function_component, html, use_context};
use yew_router::prelude::Redirect;

use crate::{app::AppRoute, types::AppContext};

#[function_component(Home)]
pub fn home_component() -> Html {
    let app_context = use_context::<UseStateHandle<AppContext>>().expect("No app_context found");

    if app_context.user_info.is_none() {
        html! {
            <Redirect<AppRoute> to={AppRoute::Login}/>
        }
    } else {
        html! {
            <div>
            <h1>{ "Hello Blog!" }</h1>
            <h2>{ "Your personal blog written totally in Rust ;)" }</h2>
            </div>
        }
    }
}

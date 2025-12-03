use yew::{Html, UseStateHandle, function_component, html, use_context};
use yew_router::prelude::Redirect;

use crate::{app::AppRoute, types::AppContext};

#[function_component(Admin)]
pub fn admin_component() -> Html {
    let app_context = use_context::<UseStateHandle<AppContext>>().expect("No app_context found");

    if let Some(context_user_info) = app_context.user_info.clone()
        && context_user_info.admin.is_some_and(|x| x)
    {
        html! {
            <h2>{"Admin panel"}</h2>
        }
    } else {
        html! {
            <Redirect<AppRoute> to={AppRoute::NotFound}/>
        }
    }
}

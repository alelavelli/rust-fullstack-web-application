use log::error;
use yew::{
    Html, UseStateHandle, function_component, html, use_context, use_effect_with, use_state,
};
use yew_router::prelude::Redirect;

use crate::{
    app::AppRoute,
    component::user_list::UsersList,
    enums::HttpStatus,
    environment::EnvironmentService,
    model::{LoggedUserInfo, UserInfo},
    service::api::ApiService,
    types::{ApiResponse, AppContext},
};

#[function_component(Admin)]
pub fn admin_component() -> Html {
    let app_context = use_context::<UseStateHandle<AppContext>>().expect("No app_context found");
    let user_info: UseStateHandle<Option<LoggedUserInfo>> = use_state(|| None);
    let users: UseStateHandle<Vec<UserInfo>> = use_state(Vec::new);
    let get_users_error = use_state(|| None);

    {
        let users = users.clone();
        use_effect_with(user_info.clone(), move |user_info| {
            let users = users.clone();
            let user_info = user_info.clone();
            if let Some(user_info) = (*user_info).clone() {
                wasm_bindgen_futures::spawn_local(async move {
                    let environment_service = EnvironmentService::new();
                    let api_service = ApiService::new(
                        environment_service.api_url,
                        environment_service.mock,
                        Some(user_info.token),
                    );
                    let response = api_service.get_admin_users_list().await;
                    if let Ok(ApiResponse { body, status }) = response {
                        match status {
                            HttpStatus::Success(_) => {
                                users.set(body);
                                get_users_error.set(None);
                            }
                            _ => {
                                get_users_error.set(Some("Ops, something went wrong".into()));
                            }
                        }
                    } else {
                        error!(
                            "Encountered an error in get users list. Error {e}",
                            e = response.err().unwrap()
                        );
                        get_users_error.set(Some("Got error from backend".to_string()));
                    }
                });
            }
        });
    }

    if let Some(context_user_info) = app_context.user_info.clone()
        && context_user_info.admin
    {
        if user_info.is_none() {
            user_info.set(Some(context_user_info.clone()));
        }

        html! {
            <div>
                <h1>{"Admin panel"}</h1>
                <div class="admin-user-list-container">
                    <h3>{"Users list:"}</h3>
                    <UsersList users={(*users).clone()}/>
                </div>
            </div>
        }
    } else {
        html! {
            <Redirect<AppRoute> to={AppRoute::NotFound}/>
        }
    }
}

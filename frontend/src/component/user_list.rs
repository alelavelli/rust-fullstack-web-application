use yew::{Html, Properties, function_component, html};

use crate::model::UserInfo;

#[derive(Properties, PartialEq)]
pub struct UsersListProp {
    pub users: Vec<UserInfo>,
}

#[function_component(UsersList)]
pub fn users_list(UsersListProp { users, }: &UsersListProp) -> Html {

    let html_users = users.iter().map(|user| {
    
        html!{
            <li key={user.user_id.clone()} class="users-list-entry">{format!("{}", user.username)}</li>
        }
    }).collect::<Html>();

    html! {
        <ul class="users-list">
            {html_users}
        </ul>
    }
}

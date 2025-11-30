use crate::{
    component::{footer::Footer, header::Header},
    environment::EnvironmentService,
    page::{home::Home, login::Login, not_found::NotFound},
    service::auth::AuthService,
    types::AppContext,
};
use log::info;
use yew::{ContextProvider, Html, UseStateHandle, function_component, html, use_state};
use yew_router::{BrowserRouter, Routable, Switch};

#[derive(Routable, Debug, Clone, PartialEq, Eq)]
pub enum AppRoute {
    #[at("/login")]
    Login,
    #[at("/")]
    Home,
    #[not_found]
    #[at("/not-found")]
    NotFound,
}

fn switch(route: AppRoute) -> Html {
    match route {
        AppRoute::Home => html! { <Home /> },
        AppRoute::Login => html! { <Login /> },
        AppRoute::NotFound => html! { <NotFound /> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    /* According to documentation https://yew.rs/docs/concepts/contexts
    we create a state handle to contain general context that will be used
    by the application */
    let app_context = use_state(|| AppContext::new(None));

    let environment_service = EnvironmentService::new();

    // Use the AuthService to load logged user info from local storage
    // it will update the provided context with user info if they are present and valid
    AuthService::new(
        environment_service.token_storage_location_name,
        app_context.clone(),
    )
    .set_logged_user_info_from_storage();

    html! {
        <BrowserRouter>
            <ContextProvider<UseStateHandle<AppContext>> context={app_context}>
                <Header/>
                <main>
                    <Switch<AppRoute> render={switch}/>
                </main>
                <Footer/>
            </ContextProvider<UseStateHandle<AppContext>>>
        </BrowserRouter>

    }
}

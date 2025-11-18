use yew::{Html, function_component, html};

#[function_component(NotFound)]
pub fn not_found_component() -> Html {
    html! {
        "Page not found"
    }
}

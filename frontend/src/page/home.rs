use yew::{Html, function_component, html};
#[function_component(Home)]
pub fn home_component() -> Html {
    html! {
        <div>
        <h1>{ "Hello Blog!" }</h1>
        <h2>{ "Your personal blog written totally in Rust ;)" }</h2>
        </div>
    }
}

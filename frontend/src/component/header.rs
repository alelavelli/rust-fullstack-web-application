use yew::{Html, function_component, html};

#[function_component(Header)]
pub fn header_component() -> Html {
    html! {
        <header>
            <h1>{ "Hello Blog!" }</h1>
            <p>{"another component"}</p>
        </header>
    }
}

use yew::{Html, function_component, html};

#[function_component(Footer)]
pub fn footer_component() -> Html {
    html! {
        <footer>
            <div class="footer-body">{"Footer"}</div>
        </footer>
    }
}

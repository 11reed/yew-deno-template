use yew::prelude::*;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    html! {
        <div>
            <ul>
                <li>
                    <a href="/">{ "Home" }</a>
                </li>
                <li>
                    <a href="/about">{ "About" }</a>
                </li>
            </ul>
        </div>
    }
}
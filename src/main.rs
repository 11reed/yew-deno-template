extern crate wasm_bindgen_futures;
use yew_router::prelude::*;
use yew::prelude::*;
use reqwasm::http::Request;
use gloo::console::log;

mod pages;
mod components;

use crate::pages::about::About;
use crate::pages::not_found::NotFound;
use crate::components::navbar::Navbar;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[not_found]
    #[at("/404")]
    NotFound
}

#[function_component(DenoServer)]
fn deno_server() -> Html {
    wasm_bindgen_futures::spawn_local(async move {
        let deno_endpoint = format!(
            "http://localhost:3000/api"
        );

        let data = Request::get(&deno_endpoint)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        let result = &data;
        log!(result);
    });

    html!(
        { "check the console" }
    )
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <h1>{ "Home" }</h1> },
        Route::About => html! { <About /> },
        Route::NotFound => html! { <NotFound /> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
            <DenoServer />
            <Navbar />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
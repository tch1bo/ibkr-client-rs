use yew::prelude::*;
use yew_router::prelude::*;

use super::routes::home::Home;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <Home/> },
        Route::NotFound => html! { <h1>{ "Page not found! :(" }</h1> },
    }
}
use yew::prelude::*;
use yew_router::prelude::*;

use super::routes::home::Home;
use super::routes::positions::Positions;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/positions")]
    Positions,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home/> },
        Route::Positions => html! { <Positions/> },
        Route::NotFound => html! { <h1>{ "Page not found! :(" }</h1> },
    }
}

mod components;
mod models;
mod requests;
mod router;
mod routes;

use components::error::ErrorHandler;
use components::global_state_provider::GlobalStateProvider;
use router::{switch, Route};
use wasm_logger;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let routes = vec![(Route::Home, "Home"), (Route::Positions, "Positions")];

    html! {
        <GlobalStateProvider>
        <BrowserRouter >
            <nav class="navbar navbar-expand-lg p-2 sticky-top navbar-dark bg-primary">

                <Link<Route> classes={classes!("navbar-brand")} to={Route::Home}>
                    {"IBKR Client"}
                </Link<Route>>

                <div>
                    <ul class="navbar-nav mr-auto">
                        {
                            routes.iter().enumerate().map(|(index, (route, text))| {
                                let class = if index == 0 {"nav-item active"} else {"nav-item"};
                                html!{
                                        <li class={class}>
                                                <Link<Route> classes={classes!("nav-link")} to={route.clone()}>
                                                    { text }
                                                </Link<Route>>
                                        </li>
                                }
                            }).collect::<Html>()
                        }
                    </ul>
                </div>
            </nav>
            <main>
                <Switch<Route> render={switch} />
            </main>
        </BrowserRouter>
        <ErrorHandler />
        </GlobalStateProvider>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}

mod models;
mod requests;
mod router;
mod routes;

use router::{switch, Route};
use yew::prelude::*;
use yew_router::prelude::*;

struct App {}

enum Msg {}

#[function_component(Main)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

impl App {
    fn view_nav(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <nav class="navbar navbar-expand-lg p-2 sticky-top navbar-dark bg-primary">

                <Link<Route> classes={classes!("navbar-brand")} to={Route::Home}>
                    {"IBKR client"}
                </Link<Route>>

                <div>
                    <ul class="navbar-nav mr-auto">
                        <li class="nav-item active">
                                <Link<Route> classes={classes!("nav-link")} to={Route::Home}>
                                    { "Home" }
                                </Link<Route>>
                        </li>

                    </ul>

                </div>
            </nav>
        }
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        App {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter >
                {self.view_nav(&ctx)}
                <main>
                    <Switch<Route> render={Switch::render(switch)} />
                </main>
            </BrowserRouter>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}

use yew::prelude::*;

use super::super::models::{GlobalStateContext, Ledger};
use super::super::requests::{make_get_ledger_url, make_state_updater_fn_for_get_request};

#[function_component(Home)]
pub fn home() -> Html {
    let global_state = &use_context::<GlobalStateContext>().expect("no GlobalStateContext found!");

    let ledger: UseStateHandle<Ledger> = use_state_eq(Ledger::default);
    let updater_fn = make_state_updater_fn_for_get_request(
        global_state.clone(),
        ledger.clone(),
        make_get_ledger_url,
    );
    use_effect_with_deps(|_| updater_fn(), global_state.account_id.clone());

    html! {
        <div>
            <h1> {"Hello "} {global_state.account_id.clone()}{"!"}</h1>
            <div id="ledger">
                <h2> {"Your ledger"} </h2>
                <ol>
                    {
                        ledger.items.clone().into_iter().map(|(key, item)| {
                            html!{
                            <li key={key}>
                                <p> {format!("{:?}", item)} </p>
                            </li>
                            }
                        }).collect::<Html>()
                    }
                </ol>
            </div>
        </div>
    }
}

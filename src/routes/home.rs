use std::rc::Rc;

use yew::prelude::*;

use super::super::components::error::set_global_error_message;
use super::super::components::position_list::PositionList;
use super::super::models::{GlobalStateContext, Ledger};
use super::super::requests::get_ledger;

#[function_component(Home)]
pub fn home() -> Html {
    let global_state = &use_context::<GlobalStateContext>().expect("no GlobalStateContext found!");

    let ledger: UseStateHandle<Ledger> = use_state_eq(Ledger::default);
    {
        let account_id = global_state.account_id.clone();
        let account_id2 = account_id.clone();
        let ledger = ledger.clone();
        let global_state = global_state.clone();
        use_effect_with_deps(
            move |_| {
                if account_id.is_none() {
                    return;
                }
                wasm_bindgen_futures::spawn_local(async move {
                    match get_ledger(account_id.unwrap().as_str()).await {
                        Ok(fetched_ledger) => {
                            ledger.set(fetched_ledger);
                        }
                        Err(e) => set_global_error_message(global_state, e),
                    }
                });
            },
            account_id2,
        );
    }

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
            <PositionList/>
        </div>
    }
}

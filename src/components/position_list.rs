use yew::prelude::*;

use super::super::components::error::set_global_error_message;
use super::super::models::{GlobalStateContext, Position};
use super::super::requests::get_all_positions;
use log;

#[function_component(PositionList)]
pub fn position_list() -> Html {
    let global_state = use_context::<GlobalStateContext>().expect("no GlobalStateContext found!");

    // Update the positions.
    let positions: UseStateHandle<Vec<Position>> = use_state_eq(|| vec![]);
    {
        let account_id = global_state.account_id.clone();
        let account_id2 = global_state.account_id.clone();
        let positions = positions.clone();
        use_effect_with_deps(
            move |_| {
                if account_id.is_none() {
                    return;
                }
                wasm_bindgen_futures::spawn_local(async move {
                    match get_all_positions(account_id.unwrap().as_str()).await {
                        Ok(fetched_positions) => {
                            positions.set(fetched_positions);
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
            <h2> {"Your positions"} </h2>
            {positions.len()}

        </div>
    }
}

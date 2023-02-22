use yew::prelude::*;

// use super::super::components::error::set_global_error_message;
use super::super::models::{GlobalStateContext, Position};
use super::super::requests::{make_get_positions_url, make_state_updater_fn_for_get_request};

#[function_component(PositionList)]
pub fn position_list() -> Html {
    let global_state = use_context::<GlobalStateContext>().expect("no GlobalStateContext found!");

    // Update the positions.
    let positions: UseStateHandle<Vec<Position>> = use_state_eq(|| vec![]);
    let updater_fn = make_state_updater_fn_for_get_request(
        global_state.clone(),
        positions.clone(),
        make_get_positions_url,
    );
    use_effect_with_deps(|_| updater_fn(), global_state.account_id.clone());
    html! {
        <div>
            <h2> {"Your positions"} </h2>
            {positions.len()}

        </div>
    }
}

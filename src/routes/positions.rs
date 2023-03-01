use yew::prelude::*;

use super::super::components::position_list::PositionList;

#[function_component(Positions)]
pub fn home() -> Html {
    html! {
        <div>
            <PositionList/>
        </div>
    }
}

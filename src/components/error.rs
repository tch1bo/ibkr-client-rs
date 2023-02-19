use super::super::models::{GlobalStateContext, GlobalStateAction};
use log;
use yew::prelude::*;

pub fn set_global_error_message(global_state: GlobalStateContext, error: anyhow::Error) {
    global_state.dispatch(GlobalStateAction::SetErrorMessage(format!("{:#}", error)));
}

#[function_component(ErrorHandler)]
pub fn error_handler() -> Html {
    let global_state = use_context::<GlobalStateContext>().expect("no GlobalStateContext found!");
    if global_state.error_message.is_none() {
        return html! {};
    }
    let error_message = global_state.error_message.clone().unwrap();
    log::error!("{}", error_message);
    html! {
        <div class="alert alert-danger alert-dismissable" role="alert">
            {error_message}
        </div>
    }
}

use super::super::components::error::set_global_error_message;
use super::super::models::{GlobalState, GlobalStateAction, GlobalStateContext};
use super::super::requests::get_account;
use yew::prelude::*;

#[derive(Properties, Debug, PartialEq)]
pub struct GlobalStateProviderProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(GlobalStateProvider)]
pub fn global_state_provider(props: &GlobalStateProviderProps) -> Html {
    let global_state = use_reducer(GlobalState::default);

    {
        // Update the `GlobalState::account_id`.
        let global_state = global_state.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    match get_account().await {
                        Ok(account) => {
                            global_state.dispatch(GlobalStateAction::SetAccountId(account.id))
                        }
                        Err(e) => set_global_error_message(global_state, e),
                    };
                });
            },
            (),
        );
    }

    html! {
        <ContextProvider<GlobalStateContext> context={global_state}>
            {props.children.clone()}
        </ContextProvider<GlobalStateContext>>
    }
}

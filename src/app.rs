use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

mod components;

use components::scrambler::Scrambler;

#[function_component]
pub fn App() -> Html {
    let input_value_handle = use_state(|| "ABCDE".to_string());

    let oninput = {
        let state = input_value_handle.clone();
        Callback::from(move |e: InputEvent| {
            let target: EventTarget = e
                .target()
                .expect("Event should have a target when dispatched");
            state.set(target.unchecked_into::<HtmlInputElement>().value().to_uppercase());
        })
    };

    html! {
        <div>
            <input type="text" {oninput} value={(*input_value_handle).clone()} />
            <p><Scrambler input={(*input_value_handle).clone()}/></p>
        </div>
    }
}

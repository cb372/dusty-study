use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

mod components;
mod services;

use components::content::Content;
use services::solving;

#[function_component]
pub fn App() -> Html {
    // on first render, warmup the backend lambda
    use_effect_with_deps(move |_| {
        wasm_bindgen_futures::spawn_local(async move {
            let _ = solving::warmup().await;
        });
    }, ());

    let input_value_handle = use_state(|| "".to_string());

    let input_ref = use_node_ref();

    /* A callback that gives access to the current raw value of the text box,
     * so we can check it when the results come back from the backend.
     */
    let get_input = {
        let input_ref = input_ref.clone();
        Callback::from(move |_| input_ref.cast::<HtmlInputElement>().unwrap().value().to_uppercase())
    };

    let oninput = {
        let state = input_value_handle.clone();
        Callback::from(move |e: InputEvent| {
            let target: EventTarget = e
                .target()
                .expect("Event should have a target when dispatched");
            state.set(target.unchecked_into::<HtmlInputElement>().value().to_uppercase());
        })
    };

    let clear_input = {
        let state = input_value_handle.clone();
        Callback::from(move |_| {
            state.set("".to_string());
        })
    };

    html! {
        <section class="section">
            <div class="container">
                <div class="field has-addons">
                    <div class="control has-icons-left has-icons-right is-expanded">
                        <input ref={input_ref} type="text" class="input is-info is-large" placeholder="Search" {oninput} value={(*input_value_handle).clone()} />
                        <span class="icon is-left">
                            <i class="fas fa-magnifying-glass"></i>
                        </span>
                    </div>
                    <div class="control">
                        <a class="button is-info is-large"><button class="delete is-large" onclick={clear_input}></button></a>
                    </div>
                </div>
                {
                    if (*input_value_handle).clone().chars().count() > 3 {
                        html!{<Content input={(*input_value_handle).clone()} get_input={get_input} />}
                    } else {
                        html!{<></>}
                    }
                }
            </div>
        </section>
    }
}

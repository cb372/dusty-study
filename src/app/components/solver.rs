//use gloo_console::log;
use gloo_net::Error;
use yew::prelude::*;

use crate::app::services::solving;

#[derive(Properties, PartialEq)]
pub struct SolverProp {
    pub input: String,
    pub get_input: Callback<(), String>,
    pub visible: bool
}

#[function_component(Solver)]
pub fn solver(prop: &SolverProp) -> Html {
    let results = use_state(|| None); // Option<Result<Vec<String>, Error>>

    let input = prop.input.clone();
    let get_input = prop.get_input.clone();
    {
        let input_clone = input.clone();
        let results = results.clone();
        use_effect_with_deps(move |_| {
            let input_cloned_again = input_clone.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_results: Result<Vec<String>, Error> =
                    solving::solve(input_cloned_again).await;

                let current_input = get_input.emit(());
                if input_clone == current_input.clone() {
                    results.set(Some(fetched_results));
                } else {
                    /*
                     * If the value in the input textbox has changed since we fired the request to the
                     * backend, discard the results because they are stale. We don't want to overwrite
                     * a later result.
                     */
                    //log!(format!("Discarding stale results from solver. Request was triggered with input {input_clone} but text box is currently displaying '{current_input}'"));
                }
            });
            || ()
        }, input);
    }

    if prop.visible {
        let results = &(*results);
        match results {
            None => {
                html!{<div>{"Loading..."}</div>}
            },
            Some(Result::Err(e)) => {
                html!{<div>{format!("Error! {}", e)}</div>}
            },
            Some(Result::Ok(anagrams)) => {
                let list_items = anagrams.iter().map(|result| html! {<li>{result}</li>}).collect::<Html>();
                let count = anagrams.len();
                let input = prop.input.clone();
                html!{
                    <>
                        <p>{format!("Found {count} anagrams of '{input}'")}</p>
                        <div>
                            <ul>{list_items}</ul>
                        </div>
                    </>
                }
            }
        }
    } else {
        html!{<></>}
    }
}

use gloo_net::Error;
use yew::prelude::*;

use crate::app::services::solving;

#[derive(Properties, PartialEq)]
pub struct SolverProp {
    pub input: String,
    pub visible: bool
}

#[function_component(Solver)]
pub fn solver(prop: &SolverProp) -> Html {
    let results = use_state(|| None); // Option<Result<Vec<String>, Error>>

    let input = prop.input.clone();
    {
        let input_clone = input.clone();
        let results = results.clone();
        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_results: Result<Vec<String>, Error> =
                    solving::solve(input_clone).await;
                results.set(Some(fetched_results));
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
                html!{
                    <div>
                        <ul>{list_items}</ul>
                    </div>
                }
            }
        }
    } else {
        html!{<></>}
    }
}

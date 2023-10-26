use gloo_net::http::Request;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SolverProp {
    pub input: String,
}

#[function_component(Solver)]
pub fn solver(prop: &SolverProp) -> Html {
    let results = use_state(|| vec![]);
    let url = format!("/.netlify/functions/hello?input={}", prop.input.clone());

    {
        let input = prop.input.clone();
        let results = results.clone();
        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_results: Vec<String> = Request::get(&url)
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                results.set(fetched_results);
            });
            || ()
        }, input);
    }

    let list_items = results.clone().iter().map(|result| html! {<li>{result}</li>}).collect::<Html>();

    html! {
        <div>
            <p>{"Solve!"}</p>
            <ul>{list_items}</ul>
        </div>
    }
}

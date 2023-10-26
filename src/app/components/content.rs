use yew::prelude::*;

use crate::app::components::scrambler::Scrambler;
use crate::app::components::solver::Solver;

#[derive(Properties, PartialEq)]
pub struct ContentProp {
    pub input: String,
}

#[derive(PartialEq)]
enum ActiveTab {
    Scramble,
    Solve
}

#[function_component(Content)]
pub fn content(prop: &ContentProp) -> Html {
    let active_tab_handle = use_state(|| ActiveTab::Scramble);

    let set_active_tab_scramble = {
        let active_tab_handle = active_tab_handle.clone();
        Callback::from(move |_| active_tab_handle.set(ActiveTab::Scramble))
    };

    let set_active_tab_solve = {
        let active_tab_handle = active_tab_handle.clone();
        Callback::from(move |_| active_tab_handle.set(ActiveTab::Solve))
    };

    html! {
        <>
            <div class="tabs is-centered is-boxed is-medium">
                <ul>
                    <li class={if *active_tab_handle == ActiveTab::Scramble {"is-active"} else {""}}>
                      <a onclick={set_active_tab_scramble}>
                        <span>{"Scramble"}</span>
                      </a>
                    </li>
                    <li class={if *active_tab_handle == ActiveTab::Solve {"is-active"} else {""}}>
                      <a onclick={set_active_tab_solve}>
                        <span>{"Solve"}</span>
                      </a>
                    </li>
                </ul>
            </div>
            {
                match *active_tab_handle {
                    ActiveTab::Scramble =>
                        html!{<p><Scrambler input={prop.input.clone()}/></p>},
                    ActiveTab::Solve =>
                        html!{<p><Solver input={prop.input.clone()}/></p>}
                }
            }
        </>
    }
}

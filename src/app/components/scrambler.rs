use gloo_console::log;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashSet;
use wasm_bindgen::JsValue;
use yew::prelude::*;

fn str_shuffled(s: String) -> Vec<char> {
    let mut rng = thread_rng();
    let mut chars = s.chars().collect::<Vec<char>>();
    let slice = chars.as_mut_slice();
    slice.shuffle(&mut rng);
    slice.to_vec()
}

#[derive(Clone, PartialEq, Debug)]
pub struct Scramble {
    circle: Vec<char>,
    centre: Option<char>
}

impl Scramble {

    pub fn new(input: String) -> Scramble {
        log!("Scramble::new", input.clone());
        let vowels = HashSet::from(['A', 'E', 'I', 'O', 'U']); // TODO make this static
        let mut shuffled = str_shuffled(input);
        let first_vowel_index = shuffled.iter().position(|x| vowels.contains(x));
        match first_vowel_index {
            Some(i) => {
                let vowel = shuffled.splice(i..i, []).collect::<Vec<_>>().first().copied();
                Scramble {
                    circle: shuffled,
                    centre: vowel
                }
            }
            None => {
                Scramble {
                    circle: shuffled,
                    centre: None
                }
            }
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct ScramblerProp {
    pub input: String,
}

#[function_component(Scrambler)]
pub fn scrambler(prop: &ScramblerProp) -> Html {
    let scramble = Scramble::new(prop.input.clone().to_string());
    let trigger = use_force_update();

    let onclick = Callback::from(move |_| trigger.force_update());

    html! {
        <div>
            <p>{"Scramble!"}</p>
            <p>{prop.input.clone()}</p>
            <Circle scramble={scramble.clone()} />
            <p><button onclick={onclick}>{"Re-scramble"}</button></p>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct CircleProp {
    pub scramble: Scramble,
}

#[function_component(Circle)]
pub fn circle(CircleProp { scramble }: &CircleProp) -> Html {
    log!("circle", JsValue::from(scramble.circle.clone().into_iter().collect::<String>()));
    html! {
        <div>
            <p>{scramble.circle.clone().into_iter().collect::<String>()}</p>
            <p>{scramble.centre}</p>
        </div>
    }
}

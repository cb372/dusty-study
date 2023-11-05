use rand::seq::SliceRandom;
use rand::thread_rng;
use std::cmp::max;
use std::collections::HashSet;
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
        let vowels = HashSet::from(['A', 'E', 'I', 'O', 'U']);
        let mut shuffled = str_shuffled(input.replace(" ", ""));
        let first_vowel_index = shuffled.iter().position(|x| vowels.contains(x));
        match first_vowel_index {
            Some(i) => {
                let vowel = shuffled.splice(i..i+1, []).collect::<Vec<_>>().first().copied();
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
        <div class="level">
            <Circle scramble={scramble.clone()} />
            <p class="level-item"><button class="button" onclick={onclick}>{"Re-scramble"}</button></p>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct CircleProp {
    pub scramble: Scramble,
}

fn to_svg_text(x: char, i: usize, degrees_per_char: f32) -> Html {
    let rotation = (i as f32 + 0.5) * degrees_per_char;
    let transform = format!("rotate({} 0 0) translate(0 -75) rotate(-{} 0 0)", rotation, rotation);
    html! {
        <text
            x="0"
            y="0"
            text-anchor="middle"
            dominant-baseline="central"
            font-size="30px"
            fill="black"
            transform={transform}
        >{x}</text>
    }
}

#[function_component(Circle)]
pub fn circle(CircleProp { scramble }: &CircleProp) -> Html {
    let count = max(1, scramble.circle.len()) as f32;
    let degrees_per_char = 360.0 / count;
    let circle_chars = scramble.circle
        .iter()
        .enumerate()
        .map(|(i, x)| to_svg_text(*x, i, degrees_per_char))
        .collect::<Vec<Html>>();

    html! {
        <>
            <p class="level-item">
                <svg width="200" height="200" viewBox="-100 -100 200 200" fill="none" xmlns="http://www.w3.org/2000/svg">
                    <text
                        x="0"
                        y="0"
                        text-anchor="middle"
                        dominant-baseline="central"
                        font-size="30px"
                        fill="black"
                    >{scramble.centre}</text>
                    <circle cx="0" cy="0" r="45%" stroke="#2F4F4F" stroke-width="1" fill="none" />
                    {circle_chars}
                </svg>
            </p>
        </>
    }
}

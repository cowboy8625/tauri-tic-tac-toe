mod button;
use button::Button;

use wasm_bindgen::prelude::*;
// use wasm_bindgen_futures::spawn_local;
use web_sys::window;
use yew::prelude::*;
use gloo::console;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Player {
    X,
    Y,
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Player::X => write!(f, "X"),
            Player::Y => write!(f, "Y"),
        }
    }
}

fn main() {
    yew::start_app::<App>();
}

#[wasm_bindgen(module = "/public/glue.js")]
extern "C" {
    #[wasm_bindgen(js_name = invokeHello, catch)]
    pub async fn hello(name: String) -> Result<JsValue, JsValue>;
}

#[function_component(App)]
pub fn app() -> Html {
    let state = use_state(|| ([None; 9], Player::X));
    let onclick = {
        let cloned_state = state.clone();
        Callback::from(move |(id, inner): (usize, Option<Player>)| {
            if inner.is_some() {
                return;
            }
            let (mut spots, old) = (*cloned_state).clone();
            spots[id] = Some(old);
            let current = match old {
                Player::X => Player::Y,
                Player::Y => Player::X,
            };
            cloned_state.set((spots, current));
        })
    };
    use_effect_with_deps(
        move |state| {
            let (spots, player) = state;
            console::log!(&format!("Spots: {:?}, Player: {:?}", spots, player));
            || ()
        },
        (*state).clone(),
    );

    let (spots, label) = (*state).clone();
    if is_win(&spots) {
        return html! {
            <span>{" WINNER WINNER!!!!! "}</span>
        }
    }
    html! {
        <div id="board" class="container">
            <div class="row">
                <Button id=0 value={label} onclick={onclick.clone()}/>
                <Button id=1 value={label} onclick={onclick.clone()}/>
                <Button id=2 value={label} onclick={onclick.clone()}/>
            </div>
            <div class="row">
                <Button id=3 value={label} onclick={onclick.clone()}/>
                <Button id=4 value={label} onclick={onclick.clone()}/>
                <Button id=5 value={label} onclick={onclick.clone()}/>
            </div>
            <div class="row">
                <Button id=6 value={label} onclick={onclick.clone()}/>
                <Button id=7 value={label} onclick={onclick.clone()}/>
                <Button id=8 value={label} onclick={onclick.clone()}/>
            </div>
        </div>
    }
}

fn is_win(b: &[Option<Player>]) -> bool {
    (b[0] == b[1]) && (b[1] == b[2]) && (b[0] != None)
    || (b[3] == b[4]) && (b[4] == b[5]) && (b[3] != None)
    || (b[6] == b[7]) && (b[7] == b[8]) && (b[6] != None)
    || (b[0] == b[3]) && (b[3] == b[6]) && (b[3] != None)
    || (b[1] == b[4]) && (b[4] == b[7]) && (b[1] != None)
    || (b[2] == b[5]) && (b[5] == b[8]) && (b[2] != None)
    || (b[0] == b[4]) && (b[4] == b[8]) && (b[0] != None)
    || (b[2] == b[4]) && (b[4] == b[6]) && (b[2] != None)
}

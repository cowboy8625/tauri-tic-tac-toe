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
    let player = use_state(|| Player::X);
    let onclick = {
        let player_state = player.clone();
        Callback::from(move |inner: Option<Player>| {
            if inner.is_some() {
                return;
            }
            let old = (*player_state).clone();
            let current = match old {
                Player::X => Player::Y,
                Player::Y => Player::X,
            };
            player_state.set(current.clone());
        })
    };
    use_effect_with_deps(
        move |_| {
            window().map(|win| {
                win.document().map(|doc| {
                    doc.get_element_by_id("board").map(|el| {
                        console::log!(&format!("el: {:?}", el));
                    })
                })
            });
            || ()
        },
        player.clone(),
    );

    let label = (*player).clone();
    html! {
        <div id="board" class="container">
            <div class="row">
                <Button value={label} onclick={onclick.clone()}/>
                <Button value={label} onclick={onclick.clone()}/>
                <Button value={label} onclick={onclick.clone()}/>
            </div>
            <div class="row">
                <Button value={label} onclick={onclick.clone()}/>
                <Button value={label} onclick={onclick.clone()}/>
                <Button value={label} onclick={onclick.clone()}/>
            </div>
            <div class="row">
                <Button value={label} onclick={onclick.clone()}/>
                <Button value={label} onclick={onclick.clone()}/>
                <Button value={label} onclick={onclick.clone()}/>
            </div>
        </div>
    }
}

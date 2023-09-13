mod board;
use board::Board;
use yew::{html, Component, Context, Html};

#[derive(Debug, Clone, Copy)]
pub enum Msg {
    Click(usize),
    Reset,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Player {
    X,
    O,
}

impl Player {
    fn swap(&self) -> Self {
        match self {
            Self::X => Self::O,
            Self::O => Self::X,
        }
    }
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::X => write!(f, "X"),
            Self::O => write!(f, "O"),
        }
    }
}

pub struct App {
    pub slots: [Option<Player>; 9],
    pub current_player: Player,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            slots: [None; 9],
            current_player: Player::X,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Click(id) if self.slots[id].is_none() => {
                self.slots[id] = Some(self.current_player);
                self.current_player = self.current_player.swap();
                true
            }
            Msg::Reset => {
                self.slots = [None; 9];
                self.current_player = Player::X;
                true
            }
            _ => false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick =
            ctx.link().callback(move |id: usize| Msg::Click(id));

        if !is_win(&self.slots) && self.slots.iter().all(|x| x.is_some()) {
            return html! {
                <div class="container">
                    <span class="message">{"No One Won This Time!"}</span>
                    <button class="reset-btn" onclick={ctx.link().callback(|_| Msg::Reset)}>{"Reset"}</button>
                </div>
            };
        } else if is_win(&self.slots) {
            let message = format!("{} win!", self.current_player.swap());
            return html! {
                <div class="container">
                    <span class="message">{message}</span>
                    <button class="reset-btn" onclick={ctx.link().callback(|_| Msg::Reset)}>{"Reset"}</button>
                </div>
            };
        }
        html! {<Board slots={self.slots.clone()} onclick={onclick} />}
    }
}

fn main() {
    yew::start_app::<App>();
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

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

impl App {
    fn get_slot(&self, id: usize) -> String {
        self.slots[id].map(|i| i.to_string()).unwrap_or("".into())
    }
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
                false
            }
            _ => false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = |id: usize| {
            ctx.link().callback(move |_| Msg::Click(id))
        };
        html! {<Board slots={self.slots.clone()} onclick={onclick} />}
    }
}

fn main() {
    yew::start_app::<App>();
}

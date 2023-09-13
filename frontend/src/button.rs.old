use gloo::console;
use yew::{html, Component, Context, Html, Properties};
use yew::prelude::*;
use super::Player;


// <div value="ldaskdj" item=123>{children}</div>
#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub value: Player,
    pub onclick: Callback<(usize, Option<Player>)>,
    pub id: usize,
}

// Define the possible messages which can be sent to the component
pub enum Msg {
    UpdateButtonUi(Player),
}

pub struct Button(Option<Player>);

impl Component for Button {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self(None)
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        use Player::*;
        match msg {
            Msg::UpdateButtonUi(player @ (X|Y)) if self.0.is_none() => {
                self.0 = Some(player);
                true
            }
            _ => false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let value = ctx.props().value.clone();
        let id = ctx.props().id.clone();
        let onclick = ctx.props().onclick.clone();
        let label = self.0.map(|s| s.to_string()).unwrap_or_default();
        let inner = self.0.clone();
        html! {
            <button class="button-comp" onclick={
                ctx.link().callback(move |_| {
                    onclick.emit((id, inner));
                    Msg::UpdateButtonUi(value)

                })
            }>
                {label}
            </button>
        }
    }
}

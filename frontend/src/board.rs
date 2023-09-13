use yew::{html, Properties, Component, Context, Html};
use yew::prelude::*;
use super::Player;

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub slots: [Option<Player>; 9],
    pub onclick: Callback<usize>,
}

pub struct Board;

impl Component for Board {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self { }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let slots = ctx.props().slots.clone();
        let onclick = ctx.props().onclick.clone();
        let getter = |i: usize| {
            slots[i].map(|i| i.to_string()).unwrap_or_default()
        };
        html! {
<div class="container">
    <div class="row">
        <button class="bp" onclick={|_|onclick.emit(0)}>{getter(0)}</button>
        <button class="bp" onclick={|_|onclick.emit(1)}>{getter(1)}</button>
        <button class="bp" onclick={|_|onclick.emit(2)}>{getter(2)}</button>
    </div>
    <div class="row">
        <button class="bp" onclick={|_|onclick.emit(3)}>{getter(3)}</button>
        <button class="bp" onclick={|_|onclick.emit(4)}>{getter(4)}</button>
        <button class="bp" onclick={|_|onclick.emit(5)}>{getter(5)}</button>
    </div>
    <div class="row">
        <button class="bp" onclick={|_|onclick.emit(6)}>{getter(6)}</button>
        <button class="bp" onclick={|_|onclick.emit(7)}>{getter(7)}</button>
        <button class="bp" onclick={|_|onclick.emit(8)}>{getter(8)}</button>
    </div>
</div>
        }
    }
}

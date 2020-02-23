#![recursion_limit="512"]

use yew::{html, Component, ComponentLink, Html, ShouldRender};

mod photos;
mod recommendations;

use photos::Photos;

struct App {
}

enum Msg {
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        App {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <Photos />
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
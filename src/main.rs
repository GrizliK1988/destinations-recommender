#![recursion_limit="512"]

use yew::{html, Component, ComponentLink, Html, ShouldRender};

mod layout;

use layout::Layout;

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
            <Layout />
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
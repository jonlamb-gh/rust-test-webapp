use yew::prelude::*;

use web::*;

pub struct RootModel {}

pub enum RootMsg {}

impl Component<Context> for RootModel {
    type Message = RootMsg;
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<Context, Self>) -> Self {
        RootModel {}
    }

    fn update(&mut self, _msg: Self::Message, _context: &mut Env<Context, Self>) -> ShouldRender {
        true
    }
}

impl Renderable<Context, RootModel> for RootModel {
    fn view(&self) -> Html<Context, Self> {
        html! {
            <div>
                <h1>{"Test WebApp"}</h1>
                <NewInvoiceModel:/>
            <div/>
        }
    }
}

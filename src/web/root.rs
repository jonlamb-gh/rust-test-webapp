use yew::prelude::*;

use web::*;

pub struct RootModel {
    people_version: usize,
}

pub enum RootMsg {
    PeopleUpdated(usize),
}

impl Component<Context> for RootModel {
    // Some details omitted. Explore the examples to get more.

    type Message = RootMsg;
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<Context, Self>) -> Self {
        RootModel { people_version: 0 }
    }

    fn update(&mut self, msg: Self::Message, context: &mut Env<Context, Self>) -> ShouldRender {
        true
    }
}

impl Renderable<Context, RootModel> for RootModel {
    fn view(&self) -> Html<Context, Self> {
        html! {
            <div>
                <h1>{"Test WebApp"}</h1>
                <PiecesModel:/>
            <div/>
        }
    }
}

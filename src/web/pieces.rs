// TODO - page level org?

use stdweb::web::html_element::SelectElement;
use yew::callback::Callback;
use yew::format::Json;
use yew::prelude::*;

use data;
use data::*;
use web::*;

pub enum PiecesMsg {
    Todo,
}

#[derive(Clone)]
pub struct PiecesModel {
    pieces: Vec<Piece>,
}

#[derive(Clone, Default, PartialEq)]
pub struct PiecesProps {}

impl Component<Context> for PiecesModel {
    type Message = PiecesMsg;
    type Properties = PiecesProps;

    fn create(props: Self::Properties, context: &mut Env<Context, Self>) -> Self {
        context.console.debug("creating PiecesModel");

        let piece_a = Piece::new();
        let piece_b = Piece::new();

        let mut pieces = Vec::<Piece>::new();
        pieces.push(piece_a);
        pieces.push(piece_b);

        PiecesModel {
            //pieces: Vec::<Piece>::new(),
            pieces,
        }
    }

    fn update(&mut self, msg: Self::Message, context: &mut Env<Context, Self>) -> ShouldRender {
        true
    }

    fn change(
        &mut self,
        _props: Self::Properties,
        _context: &mut Env<Context, Self>,
    ) -> ShouldRender {
        false
    }
}

impl Renderable<Context, PiecesModel> for PiecesModel {
    fn view(&self) -> Html<Context, Self> {
        let piece_row = |piece: &Piece| {
            html!{
                <tr>
                    <td>{ piece.lumber_type().to_str() }</td>
                    <td>{ piece.description() }</td>
                </tr>
            }
        };

        html! {
            <>
                <h2>{"a header here"}</h2>
                <table>
                    <thead>
                        <tr><th>{"Lumber Type"}</th><th>{"Description"}</th><th>{" "}</th></tr>
                    </thead>
                    <tbody>
                        { for self.pieces.iter().map(|p| piece_row(p)) }
                    </tbody>
                </table>
            </>
        }
    }
}

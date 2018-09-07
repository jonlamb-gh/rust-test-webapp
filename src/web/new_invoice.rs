use stdweb::web::html_element::SelectElement;
use yew::format::Json;
use yew::prelude::*;

use data::*;
use web::*;

pub enum NewInvoiceMsg {
    AddPiece,
}

pub struct NewInvoiceModel {
    invoice: Invoice,
}

#[derive(Clone, Default, PartialEq)]
pub struct NewInvoiceProps {}

impl Component<Context> for NewInvoiceModel {
    type Message = NewInvoiceMsg;
    type Properties = NewInvoiceProps;

    fn create(_props: Self::Properties, context: &mut Env<Context, Self>) -> Self {
        context.console.debug("creating NewInvoiceModel");

        NewInvoiceModel {
            invoice: Invoice::new(),
        }
    }

    fn update(&mut self, msg: Self::Message, context: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            NewInvoiceMsg::AddPiece => {
                context.console.debug("adding a Piece");
                let piece = Piece::new();
                self.invoice.add_piece(piece);
                true
            }
        }
    }

    fn change(
        &mut self,
        _props: Self::Properties,
        _context: &mut Env<Context, Self>,
    ) -> ShouldRender {
        false
    }
}

impl Renderable<Context, NewInvoiceModel> for NewInvoiceModel {
    fn view(&self) -> Html<Context, Self> {
        let piece_row = |piece: &Piece| {
            html!{
                <tr>
                    <td>{ piece.lumber_type().to_str() }</td>
                    <td>{ piece.description() }</td>
                    <td>{ format!("{}", piece.board_dimensions()) }</td>
                    <td>{ "1" }</td>
                    <td>{ piece.board_dimensions().board_feet() }</td>
                    <td>{ format!("{}", piece.lumber_type().fob_price()) }</td>
                    <td>{ format!("{}", piece.cost()) }</td>
                </tr>
            }
        };

        html! {
            <>
                <h2>{"New Invoice"}</h2>
                <table>
                    <thead>
                        <tr>
                            <th>{"Lumber Type"}</th>
                            <th>{"Description"}</th>
                            <th>{"Dimensions (T x W x L)"}</th>
                            <th>{"Quantity"}</th>
                            <th>{"BF"}</th>
                            <th>{"fob <LOCATION>"}</th>
                            <th>{"Cost"}</th>
                            <th>{" "}</th>
                        </tr>
                    </thead>
                    <tbody>
                        { for self.invoice.pieces().iter().map(|p| piece_row(p)) }
                    </tbody>
                    <tfoot>
                        <tr><td>
                            <button onclick=|_| NewInvoiceMsg::AddPiece, >
                                <i class=("fa", "fa-plus-square"), aria-hidden="true",></i>
                            </button>
                        </td></tr>
                    </tfoot>
                </table>
            </>
        }
    }
}

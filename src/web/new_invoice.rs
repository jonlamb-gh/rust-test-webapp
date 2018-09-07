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

        Self {
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
        let header = |name: &str| {
            html!{
                <th>{ format!("{}", name) }</th>
            }
        };

        let piece_col = |val: &str| {
            html!{
                <td>{ format!("{}", val) }</td>
            }
        };

        let piece_row = |piece: &Piece| {
            let values = piece.enumerate();
            html!{
                <tr>
                    { for values.iter().map(|v| piece_col(v)) }
                </tr>
            }
        };

        html! {
            <>
                <h2>{"New Invoice"}</h2>
                <table>
                    <thead>
                        <tr>
                            { for Piece::enumerate_headers().iter().map(|h| header(h)) }
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
                <InvoiceSummaryModel: summary={self.invoice.summary()},/>
            </>
        }
    }
}

#[derive(Clone, PartialEq, Default)]
struct InvoiceSummaryModel {
    summary: Summary,
}

enum InvoiceSummaryMsg {}

impl Component<Context> for InvoiceSummaryModel {
    type Message = InvoiceSummaryMsg;
    type Properties = Self;

    fn create(props: Self::Properties, context: &mut Env<Context, Self>) -> Self {
        context.console.debug("creating InvoiceSummaryModel");

        Self {
            summary: props.summary,
        }
    }

    fn update(&mut self, _msg: Self::Message, _context: &mut Env<Context, Self>) -> ShouldRender {
        false
    }

    fn change(
        &mut self,
        _props: Self::Properties,
        _context: &mut Env<Context, Self>,
    ) -> ShouldRender {
        false
    }
}

impl Renderable<Context, InvoiceSummaryModel> for InvoiceSummaryModel {
    fn view(&self) -> Html<Context, Self> {
        let header = |name: &str| {
            html!{
                <th>{ format!("{}", name) }</th>
            }
        };

        let summary_val = |val: &str| {
            html!{
                <td>{ format!("{}", val) }</td>
            }
        };

        let summary_values = self.summary.enumerate();

        html! {
            <>
                <h2>{"Summary"}</h2>
                <table>
                    <thead>
                        <tr>
                            { for Invoice::enumerate_headers().iter().map(|h| header(h)) }
                        </tr>
                    </thead>
                    <tbody>
                        { for summary_values.iter().map(|v| summary_val(v)) }
                    </tbody>
                </table>
            </>
        }
    }
}

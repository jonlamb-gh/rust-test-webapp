use yew::prelude::*;

use data::*;
use web::*;

pub enum NewInvoiceMsg {
    AddItem,
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
            NewInvoiceMsg::AddItem => {
                context.console.debug("adding new item");
                let item = BillableItem::new();
                self.invoice.add_billable_item(item);
                true
            }
        }
    }

    fn change(
        &mut self,
        _props: Self::Properties,
        context: &mut Env<Context, Self>,
    ) -> ShouldRender {
        context.console.debug("update NewInvoiceModel props");
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

        let item_col_val = |val: &str| {
            html!{
                <td>{ format!("{}", val) }</td>
            }
        };

        let item_row = |item: &BillableItem| {
            let values = item.enumerate();
            html!{
                <tr>
                    { for values.iter().map(|v| item_col_val(v)) }
                </tr>
            }
        };

        html! {
            <>
                <h2>{"New Invoice"}</h2>
                <table>
                    <thead>
                        <tr>
                            { for BillableItem::enumerate_headers().iter().map(|h| header(h)) }
                        </tr>
                    </thead>
                    <tbody>
                        { for self.invoice.items().iter().map(|i| item_row(i)) }
                    </tbody>
                    <tfoot>
                        <tr><td>
                            <button onclick=|_| NewInvoiceMsg::AddItem, >
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
        true
    }

    fn change(
        &mut self,
        props: Self::Properties,
        context: &mut Env<Context, Self>,
    ) -> ShouldRender {
        context.console.debug("update InvoiceSummaryModel props");
        self.summary = props.summary;
        true
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

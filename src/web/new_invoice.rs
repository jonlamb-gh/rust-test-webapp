use stdweb::web::html_element::SelectElement;
use yew::callback::Callback;
use yew::prelude::*;

use data::*;
use web::*;

type IsEditting = bool;
type Id = usize;

pub enum NewInvoiceMsg {
    AddItem,
    EditItem(Id),
    DeleteItem(Id),
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
            NewInvoiceMsg::EditItem(id) => {
                context.console.debug(&format!("edit item: {}", id));
                true
            }
            NewInvoiceMsg::DeleteItem(id) => {
                context.console.debug(&format!("delete item: {}", id));
                self.invoice.remove_billable_item(id);
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

        let edit_delete = |id: Id, is_editting: IsEditting| {
            html!{
                <EditDelete: id={id}, is_editting={is_editting}, on_edit=|id: Id| NewInvoiceMsg::EditItem(id), on_delete=|id: Id| NewInvoiceMsg::DeleteItem(id), />
            }
        };

        let item_row = |id: Id, item: &BillableItem| {
            let values = item.enumerate();
            html!{
                <tr>
                    { for values.iter().map(|v| item_col_val(v)) }
                    <td class="edit_delete",>{ edit_delete(id, false) }</td>
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
                        { for self.invoice.items().iter().enumerate().map(|(i, b)| item_row(i, b)) }
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
                <OrderInfoModel: order_info={self.invoice.order_info().clone()},/>
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

#[derive(Clone, PartialEq, Default)]
struct OrderInfoModel {
    order_info: OrderInfo,
}

enum OrderInfoMsg {}

impl Component<Context> for OrderInfoModel {
    type Message = OrderInfoMsg;
    type Properties = Self;

    fn create(props: Self::Properties, context: &mut Env<Context, Self>) -> Self {
        context.console.debug("creating OrderInfoModel");

        Self {
            order_info: props.order_info,
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
        context.console.debug("update OrderInfoModel props");
        self.order_info = props.order_info;
        true
    }
}

impl Renderable<Context, OrderInfoModel> for OrderInfoModel {
    fn view(&self) -> Html<Context, Self> {
        let header = |name: &str| {
            html!{
                <th>{ format!("{}", name) }</th>
            }
        };

        let info_val = |val: &str| {
            html!{
                <td>{ format!("{}", val) }</td>
            }
        };

        let values = self.order_info.enumerate();

        html! {
            <>
                <h2>{"Order Information"}</h2>
                <table>
                    <thead>
                        <tr>
                            { for OrderInfo::enumerate_headers().iter().map(|h| header(h)) }
                        </tr>
                    </thead>
                    <tbody>
                        { for values.iter().map(|v| info_val(v)) }
                    </tbody>
                </table>
            </>
        }
    }
}

#[derive(Clone, PartialEq, Default)]
struct EditDelete {
    pub id: Id,
    pub is_editting: IsEditting,
    pub on_edit: Option<Callback<Id>>,
    pub on_delete: Option<Callback<Id>>,
}

enum EditDeleteMsg {
    Edit,
    Delete,
}

impl Component<Context> for EditDelete {
    type Message = EditDeleteMsg;
    type Properties = Self;

    fn create(props: Self::Properties, _context: &mut Env<Context, Self>) -> Self {
        Self {
            id: props.id,
            is_editting: props.is_editting,
            on_edit: props.on_edit,
            on_delete: props.on_delete,
        }
    }

    fn update(&mut self, msg: Self::Message, context: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            EditDeleteMsg::Edit => {
                context.console.debug(&format!("editting: {}", self.id));
                if !self.is_editting {
                    self.on_edit.as_ref().map(|c| c.emit(self.id));
                }
            }
            EditDeleteMsg::Delete => {
                context.console.debug(&format!("deleting: {}", self.id));
                self.on_delete.as_ref().map(|c| c.emit(self.id));
            }
        }

        false
    }

    fn change(
        &mut self,
        props: Self::Properties,
        _context: &mut Env<Context, Self>,
    ) -> ShouldRender {
        if self.is_editting != props.is_editting {
            self.is_editting = props.is_editting;
            return true;
        }
        false
    }
}

impl Renderable<Context, EditDelete> for EditDelete {
    fn view(&self) -> Html<Context, Self> {
        let disabled = if self.is_editting { "disabled" } else { "" };

        html! {
            <div class="edit_delete", >
                <i class=("fa", "fa-pencil-square-o", "fa-fw", disabled), aria-hidden="true", onclick=|_| EditDeleteMsg::Edit, />
                <i class=("fa", "fa-trash", "fa-fw"), aria-hidden="true", onclick=|_| EditDeleteMsg::Delete, />
            </div>
        }
    }
}

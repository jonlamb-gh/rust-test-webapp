use yew::services::console::ConsoleService;
use yew::services::storage::StorageService;

pub struct Context {
    pub console: ConsoleService,
    pub local_store: StorageService,
}

mod new_invoice;
mod root;

pub use self::new_invoice::NewInvoiceModel;
pub use self::root::RootModel;

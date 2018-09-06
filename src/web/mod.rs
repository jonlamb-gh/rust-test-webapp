use yew::services::console::ConsoleService;
use yew::services::storage::StorageService;

pub struct Context {
    pub console: ConsoleService,
    pub local_store: StorageService,
}

mod root;

pub use self::root::RootModel;

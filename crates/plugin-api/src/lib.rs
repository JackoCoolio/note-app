#![no_std]

wit_bindgen::generate!({
    path: "./plugin.wit",
    world: "tasks",
    export_macro_name: "register_plugin",
    pub_export_macro: true,
});

pub use tasks::plugin::types::{Capabilities, Metadata};

pub trait Plugin {
    fn metadata() -> Metadata;
    fn get_capabilities() -> Capabilities;
}

impl<T> exports::plugin::Guest for T
where
    T: Plugin,
{
    fn get_metadata() -> Metadata {
        T::metadata()
    }

    fn get_capabilities() -> Capabilities {
        <T as Plugin>::get_capabilities()
    }
}

// Registers a plugin by exposing relevant functions
// to the WASM component;
// #[macro_export]
// macro_rules! register_plugin {
//     ($plugin:ident) => {
//         $crate::bindings::export!($plugin);
//     };
// }

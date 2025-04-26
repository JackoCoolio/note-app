#![no_std]

use plugin_api::{Capabilities, Metadata, Plugin, register_plugin};

struct MyPlugin;

impl Plugin for MyPlugin {
    fn metadata() -> plugin_api::Metadata {
        Metadata {
            name: "File Note Provider".into(),
            version: "0.0.0".into(),
        }
    }

    fn get_capabilities() -> plugin_api::Capabilities {
        Capabilities::NOTE_PROVIDER
    }
}

register_plugin!(MyPlugin with_types_in plugin_api);

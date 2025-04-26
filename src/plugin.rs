use std::path::Path;

use tasks::plugin::types::Metadata;
use wasmtime::{
    Engine, Store,
    component::{Component, Linker, bindgen},
};
use wasmtime_wasi::{IoView, ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};

bindgen!({
    path: "./crates/plugin-api",
    world: "tasks",
});

struct H {
    ctx: WasiCtx,
    table: ResourceTable,
}

impl WasiView for H {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
}

impl IoView for H {
    fn table(&mut self) -> &mut wasmtime_wasi::ResourceTable {
        &mut self.table
    }
}

impl host::Host for H {
    fn get_number(&mut self) -> i32 {
        42
    }
}

impl tasks::plugin::types::Host for H {}

pub struct Plugin {
    store: wasmtime::Store<H>,
    bindings: Tasks,
}

impl Plugin {
    pub fn load_from_file(engine: &Engine, path: impl AsRef<Path>) -> Result<Self, Error> {
        let component = Component::from_file(engine, path).map_err(Error::Load)?;

        let mut linker = Linker::new(engine);

        let host = H {
            ctx: WasiCtxBuilder::new().build(),
            table: ResourceTable::new(),
        };

        Tasks::add_to_linker(&mut linker, |host: &mut H| host).map_err(Error::LinkComponent)?;

        // TODO: handle separate WASI capabilities separately - ask permission from
        // user: "plugin XYZ is requesting to access the filesystem... yes/no?"
        wasmtime_wasi::add_to_linker_sync(&mut linker).map_err(Error::LinkWasi)?;

        let mut store = Store::new(engine, host);

        let bindings =
            Tasks::instantiate(&mut store, &component, &linker).map_err(Error::Instantiate)?;

        Ok(Self { store, bindings })
    }

    pub fn metadata(&mut self) -> Metadata {
        self.bindings
            .plugin()
            .call_get_metadata(&mut self.store)
            .unwrap()
    }
}

#[derive(Debug)]
pub enum Error {
    Load(wasmtime::Error),
    LinkComponent(wasmtime::Error),
    LinkWasi(wasmtime::Error),
    Instantiate(wasmtime::Error),
}

use plugin::Plugin;
use wasmtime::Engine;

mod plugin;

fn main() {
    let engine = Engine::default();
    let mut plug = Plugin::load_from_file(
        &engine,
        "./plugins/file-note-provider/target/wasm32-wasip2/debug/file_note_provider.wasm",
    )
    .unwrap();

    println!(
        "loaded plugin {}@{}",
        plug.metadata().name,
        plug.metadata().version
    );
}

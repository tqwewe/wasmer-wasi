use anyhow::Result;
use wasmtime::*;
use wasmtime_wasi::sync::WasiCtxBuilder;
use wit_bindgen_rust::import;

import!("./domain.wit");

pub use domain::add;

fn main() -> Result<()> {
    // Define the WASI functions globally on the `Config`.
    let engine = Engine::default();
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |s| s)?;

    // Create a WASI context and put it in a Store; all instances in the store
    // share this context. `WasiCtxBuilder` provides a number of ways to
    // configure what the target program will have access to.
    let wasi = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_args()?
        .build();
    let mut store = Store::new(&engine, wasi);

    // Instantiate our module with the imports we've created, and run it.
    let module = Module::from_file(&engine, "target/wasm32-wasi/release/domain.wasm")?;
    linker.module(&mut store, "", &module)?;
    // linker
    //     .get(&mut store, "", Some("add"))
    //     .ok_or(anyhow!("NOO"))?
    //     .into_func()
    //     .ok_or(anyhow!("NOO"))?.typed(store)

    let s = add("hey");
    println!("{}", s);

    Ok(())
}

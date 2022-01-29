use anyhow::Result;
use wasmtime::*;
use wasmtime_wasi::sync::WasiCtxBuilder;
use wit_bindgen_wasmtime::import;

import!("./domain.wit");

pub use domain::{Domain, DomainData};

fn main() -> Result<()> {
    // Define the WASI functions globally on the `Config`.
    let engine = Engine::default();
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |s| s)?;

    // Create a WASI context and put it in a Store; all instances in the store
    // share this context. `WasiCtxBuilder` provides a number of ways to
    // configure what the target program will have access to.
    let mut wasi = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_args()?
        .build();
    let domain_index = wasi.table().push(Box::new(DomainData::default()))?;
    let mut store = Store::new(&engine, wasi);

    // Instantiate our module with the imports we've created, and run it.
    let module = Module::from_file(&engine, "target/wasm32-wasi/release/domain.wasm")?;
    // linker.module(&mut store, "", &module)?;

    let (domain, instance) = Domain::instantiate(&mut store, &module, &mut linker, move |s| {
        s.table().get_mut::<DomainData>(domain_index).unwrap()
    })?;

    let s = domain.add(&mut store, "hey")?;
    println!("{}", s);

    Ok(())
}

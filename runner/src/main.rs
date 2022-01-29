use anyhow::Result;
use wasmtime::*;
use wasmtime_wasi::sync::WasiCtxBuilder;
use wit_bindgen_wasmtime::import;

import!("./domain.wit");

pub use domain::{Domain, DomainData};

struct Data {
    //wasi structure here
    wasi: wasmtime_wasi::WasiCtx,
    // and any extra data for wasm here, including domain data:
    domain: DomainData,
}

fn main() -> Result<()> {
    // Define the WASI functions globally on the `Config`.
    let engine = Engine::default();
    let mut linker = Linker::<Data>::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |ctx| &mut ctx.wasi)?;

    // Create a WASI context and put it in a Store; all instances in the store
    // share this context. `WasiCtxBuilder` provides a number of ways to
    // configure what the target program will have access to.
    let wasi = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_args()?
        .build();
    let mut store = Store::new(
        &engine,
        Data {
            wasi,
            domain: DomainData::default(),
        },
    );

    // Instantiate our module with the imports we've created, and run it.
    let module = Module::from_file(&engine, "target/wasm32-wasi/release/domain.wasm")?;
    // linker.module(&mut store, "", &module)?;

    let (domain, _instance) =
        Domain::instantiate(&mut store, &module, &mut linker, move |ctx| &mut ctx.domain)?;

    let s = domain.add(&mut store, "hey")?;
    println!("{}", s);

    Ok(())
}

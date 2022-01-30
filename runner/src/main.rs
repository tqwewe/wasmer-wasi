use std::time::Instant;

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
    linker.module(&mut store, "", &module)?;

    let (domain, _instance) =
        Domain::instantiate(&mut store, &module, &mut linker, move |ctx| &mut ctx.domain)?;

    let mut state = domain.new_instance(&mut store, "ari")??;
    // println!("{:?}", state);
    let events = domain.handle_command(
        &mut store,
        &state,
        br#"{"command":"open_account","params":{"initial_balance":10.0}}"#,
    )??;

    state = domain.apply_events(
        &mut store,
        &state,
        &events
            .iter()
            .map(|event| event.as_ref())
            .collect::<Vec<_>>(),
    )??;

    let events = events
        .into_iter()
        .map(|event| String::from_utf8_lossy(&event).to_string())
        .collect::<Vec<_>>();
    println!("{}", events.join("\n"));

    let start = Instant::now();

    for _ in 0..100000 {
        let events = domain.handle_command(
            &mut store,
            &state,
            br#"{"command":"deposit_funds","params":{"amount":5.0}}"#,
        )??;

        state = domain.apply_events(
            &mut store,
            &state,
            &events
                .iter()
                .map(|event| event.as_ref())
                .collect::<Vec<_>>(),
        )??;
    }

    let end = Instant::now();
    let duration = end - start;
    println!("time: {}", duration.as_millis());

    let events = domain.handle_command(
        &mut store,
        &state,
        br#"{"command":"open_account","params":{"initial_balance":10.0}}"#,
    )??;

    let events = events
        .into_iter()
        .map(|event| String::from_utf8_lossy(&event).to_string())
        .collect::<Vec<_>>();
    println!("{:?}", events);

    Ok(())
}

impl std::fmt::Display for domain::Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            domain::Error::Command(bytes) => write!(f, "{}", String::from_utf8_lossy(bytes)),
            _ => write!(f, "{:?}", self),
        }
    }
}

impl std::error::Error for domain::Error {}

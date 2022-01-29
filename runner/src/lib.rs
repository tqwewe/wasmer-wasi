use anyhow::Result;
use wasmtime::*;
use wasmtime_wasi::sync::WasiCtxBuilder;
use wit_bindgen_wasmtime::import;

import!("./domain.wit");

pub use domain::*;

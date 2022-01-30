use std::time::Instant;

use domain::{Domain, WasmDomain};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut state = Domain::new_instance("ari".to_string()).unwrap();

    let events = Domain::handle_command(
        state.clone(),
        br#"{"command":"open_account","params":{"initial_balance":10.0}}"#.to_vec(),
    )
    .unwrap();

    state = Domain::apply_events(state, events).unwrap();

    let start = Instant::now();

    for _ in 0..100000 {
        let events = Domain::handle_command(
            state.clone(),
            br#"{"command":"deposit_funds","params":{"amount":5.0}}"#.to_vec(),
        )
        .unwrap();

        state = Domain::apply_events(state, events).unwrap();
    }

    let end = Instant::now();
    let duration = end - start;
    println!("time: {}", duration.as_millis());

    Ok(())
}

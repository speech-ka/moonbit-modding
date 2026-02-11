use anyhow::Result;
use wasmtime::component::*;
use wasmtime::{Config, Engine, Store};

bindgen!({
    world: "calculator",
    path: "./wit",
});

struct HostState;

fn main() -> Result<()> {
    let mut config = Config::new();
    config.wasm_component_model(true);
    
    let engine = Engine::new(&config)?;
    let mut linker = Linker::new(&engine);
    
    let component = Component::from_file(
        &engine,
        "./moonbit-guest/component.wasm"
    )?;
    
    let mut store = Store::new(&engine, HostState);
    let bindings = Calculator::instantiate(&mut store, &component, &linker)?;
    
    let point_a = Point { x: 10, y: 20 };
    let point_b = Point { x: 5, y: 15 };
    
    println!("Point A: ({}, {})", point_a.x, point_a.y);
    println!("Point B: ({}, {})", point_b.x, point_b.y);
    
    let sum = bindings.call_add_points(&mut store, point_a, point_b)?;
    println!("Sum: ({}, {})", sum.x, sum.y);
    
    let result = bindings.call_calculate(&mut store, point_a, point_b)?;
    println!("Calculation Result:");
    println!("  Sum: ({}, {})", result.sum.x, result.sum.y);
    println!("  Distance: {:.2}", result.distance);
    
    let origin = bindings.call_get_origin(&mut store)?;
    println!("Origin: ({}, {})", origin.x, origin.y);
    
    Ok(())
}
use anyhow::Result;
use wasmtime::component::{
	bindgen, Component, HasSelf, Linker, Resource as WasmtimeResource, ResourceTable,
};
use wasmtime::{Config, Engine, Store};

use crate::fallow::midden::krenel::{Host as KrenelHost, HostActor, Point};
use crate::wasi::logging::logging::{Host as WasiLoggingHost, Level as WasiLogLevel};

bindgen!({
    path: "moonbit-guest/wit",
    world: "fortalice",
    with: {
        "fallow:midden/krenel.actor": Actor,
    },

	//imports: { default: trappable },
});

#[derive(Clone)]
pub struct Actor {
	pos: fallow::midden::krenel::Point,
	kind: String,
}

#[derive(Default)]
struct HostState {
	table: ResourceTable,
}

impl HostActor for HostState {
	fn get_position(&mut self, actor: WasmtimeResource<Actor>) -> Point {
		let actor = self.table.get(&actor).unwrap();
		actor.pos
	}

	fn set_position(&mut self, actor: WasmtimeResource<Actor>, pos: Point) -> () {
		let actor = self.table.get_mut(&actor).unwrap();
		actor.pos = pos;
	}

	fn drop(&mut self, actor: WasmtimeResource<Actor>) -> Result<()> {
		let _actor: Actor = self.table.delete(actor)?;
		Ok(())
	}
}

impl KrenelHost for HostState {
	fn spawn(&mut self, kind: String, pos: Point) -> WasmtimeResource<Actor> {
		let actor = Actor { pos, kind };
		self.table.push(actor).unwrap()
	}
}

impl WasiLoggingHost for HostState {
	fn log(&mut self, level: WasiLogLevel, context: String, message: String) {
		match level {
			WasiLogLevel::Trace => log::trace!("[{context}] {message}"),
			WasiLogLevel::Debug => log::debug!("[{context}] {message}"),
			WasiLogLevel::Info => log::info!("[{context}] {message}"),
			WasiLogLevel::Warn => log::warn!("[{context}] {message}"),
			WasiLogLevel::Error => log::error!("[{context}] {message}"),
			WasiLogLevel::Critical => log::error!("[CRITICAL][{context}] {message}"),
		}
	}
}

fn main() -> Result<()> {
	env_logger::init();
	let mut config = Config::new();
	config.wasm_component_model(true);
	let engine = Engine::new(&config)?;
	let component = Component::from_file(&engine, "moonbit-guest/component.wasm")?;
	let mut linker = Linker::<HostState>::new(&engine);
	Fortalice::add_to_linker::<_, HasSelf<_>>(&mut linker, |state: &mut HostState| state)?;
	let mut store = Store::new(&engine, HostState::default());
	let bindings = Fortalice::instantiate(&mut store, &component, &linker)?;
	bindings.call_on_update(&mut store, 1.0 / 60.0)?;

	Ok(())
}
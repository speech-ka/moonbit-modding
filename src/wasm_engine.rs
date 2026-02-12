use anyhow::Result;
use bevy::prelude::*;
use wasmtime::{
	Config,
	Engine,
	Store,
	component::{
		HasSelf,
		Linker,
		Resource as WasmtimeResource,
		ResourceTable,
		bindgen,
	},
};

bindgen!({
	path: "moonbit-guest/wit",
	world: "fortalice",
	with: {
		"fallow:midden/krenel.actor": Actor,
	},

	imports: {
		"fallow:midden/krenel": trappable,
	},

/*    imports: {
		"fallow:midden/krenel.spawn": trappable,
	},*/
});
use crate::wasm_engine::{
	fallow::midden::krenel::{
		Host as KrenelHost,
		HostActor,
		Point,
	},
	wasi::logging::logging::{
		Host as WasiLoggingHost,
		Level as WasiLogLevel,
	},
};

#[derive(Clone)]
pub struct Actor {
	pos: Point,
	kind: String,
}

#[derive(Default)]
pub struct HostState {
	table: ResourceTable,
}

impl HostActor for HostState {
	fn get_position(&mut self, actor: WasmtimeResource<Actor>) -> Result<Point> {
		let actor = self.table.get(&actor)?;
		Ok(actor.pos)
	}

	fn set_position(&mut self, actor: WasmtimeResource<Actor>, pos: Point) -> Result<()> {
		let actor = self.table.get_mut(&actor)?;
		actor.pos = pos;
		Ok(())
	}

	fn drop(&mut self, actor: WasmtimeResource<Actor>) -> Result<()> {
		self.table.delete(actor)?;
		Ok(())
	}
}

impl KrenelHost for HostState {
	fn spawn(&mut self, kind: String, pos: Point) -> Result<WasmtimeResource<Actor>> {
		let actor = Actor {
			pos,
			kind,
		};
		Ok(self.table.push(actor)?)
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

pub struct WasmEngine {
	pub engine: Engine,
	pub linker: Linker<HostState>,
	pub store: Store<HostState>,
}

pub fn make_wasm_engine() -> Result<WasmEngine> {
	let mut config = Config::new();
	config.wasm_component_model(true);
	let engine = Engine::new(&config)?;
	let mut linker = Linker::<HostState>::new(&engine);
	Fortalice::add_to_linker::<_, HasSelf<_>>(
		&mut linker,
		|state: &mut HostState| state,
	)?;
	let store = Store::new(
		&engine,
		HostState::default(),
	);
	Ok(
		WasmEngine {
			engine,
			linker,
			store,
		},
	)
}

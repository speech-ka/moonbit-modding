use std::fs;

use bevy::prelude::*;
use wasmtime::component::Component;

use crate::{
	asset_mod_metadata::ModMetaData,
	asset_tracking::LoadResource,
	asset_wasm_binary::WasmBinary,
	wasm_engine::{
		Fortalice,
		WasmEngine,
		make_wasm_engine,
	},
};

pub fn plugin(app: &mut App) {
	app.load_resource::<ProvisionalMods>()
		.add_systems(
			PreStartup,
			create_wasm_engine,
		)
		.add_systems(
			Update,
			load_mods.run_if(resource_exists::<ProvisionalMods>),
		);
}

#[derive(Asset, Resource, Reflect, Clone)]
struct ProvisionalMods(pub Vec<ProvisionalMod>);

#[derive(Reflect, Clone)]
struct ProvisionalMod {
	wasm_binary: Handle<WasmBinary>,
	mod_metadata: Handle<ModMetaData>,
}

impl FromWorld for ProvisionalMods {
	fn from_world(world: &mut World) -> Self {
		let asset_server = world.resource::<AssetServer>();
		let mod_names = get_mod_names().unwrap_or_default();
		Self(
			mod_names
				.iter()
				.map(
					|mod_name| {
						let wasm_binary = asset_server.load(format!("mods/{mod_name}/component.wasm"));
						let mod_metadata = asset_server.load(format!("mods/{mod_name}/@mod.dhall"));
						ProvisionalMod {
							wasm_binary,
							mod_metadata,
						}
					},
				)
				.collect(),
		)
	}
}

#[derive(Resource)]
pub struct Mods(pub Vec<Mod>);

pub struct Mod {
	pub bindings: Fortalice,
	pub mod_metadata: ModMetaData,
}

fn create_wasm_engine(world: &mut World) {
	world.insert_non_send_resource(make_wasm_engine().unwrap())
}

fn load_mods(
	mut commands: Commands,
	provisional_mods: Res<ProvisionalMods>,
	mut wasm_binaries: ResMut<Assets<WasmBinary>>,
	mut mod_metadata_assets: ResMut<Assets<ModMetaData>>,
	mut wasm_engine: NonSendMut<WasmEngine>,
) {
	let WasmEngine {
		ref engine,
		ref mut linker,
		ref mut store,
	} = *wasm_engine;
	let mods: Vec<Mod> = provisional_mods
		.0
		.iter()
		.filter_map(
			|provisional_mod| {
				let wasm_binary = wasm_binaries.remove(&provisional_mod.wasm_binary)?;
				let mod_metadata = mod_metadata_assets.remove(&provisional_mod.mod_metadata)?;
				let component = Component::from_binary(
					engine,
					&(wasm_binary.0),
				)
				.ok()?;
				let bindings = Fortalice::instantiate(
					&mut *store, &component, &*linker,
				)
				.ok()?;
				Some(
					Mod {
						bindings,
						mod_metadata,
					},
				)
			},
		)
		.collect();
	commands.remove_resource::<ProvisionalMods>();
	commands.insert_resource(Mods(mods));
}

pub fn get_mod_names() -> Result<Vec<String>> {
	Ok(
		fs::read_dir("./assets/mods")?
			.filter_map(
				|entry| {
					let entry = entry.ok()?;
					let metadata = entry.metadata().ok()?;

					if metadata.is_dir() {
						entry
							.file_name()
							.into_string()
							.ok()
					} else {
						None
					}
				},
			)
			.collect(),
	)
}

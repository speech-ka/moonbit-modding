use std::fs;

use bevy::prelude::*;
use wasmtime::component::Component;

use crate::{
	asset_mod_metadata::ModMetaData,
	asset_wasm_binary::WasmBinary,
	wasm_engine::{
		Fortalice,
		WasmEngine,
		make_wasm_engine,
	},
};

pub fn plugin(app: &mut App) {
	app.add_systems(
		PreStartup,
		(
			create_wasm_engine,
			load_provisional_mods,
			load_mods,
		)
			.chain(),
	);
}

#[derive(Resource)]
struct ProvisionalMods(pub Vec<ProvisionalMod>);

struct ProvisionalMod {
	wasm_binary: Handle<WasmBinary>,
	mod_metadata: Handle<ModMetaData>,
}

#[derive(Resource)]
pub struct Mods(pub Vec<Mod>);

pub struct Mod {
	pub(crate) bindings: Fortalice,
	pub(crate) mod_metadata: ModMetaData,
}

fn create_wasm_engine(world: &mut World) {
	world.insert_non_send_resource(make_wasm_engine().unwrap())
}

fn load_provisional_mods(mut commands: Commands, asset_server: Res<AssetServer>) {
	let mod_names = get_mod_names().unwrap_or(Vec::new());
	commands.insert_resource(
		ProvisionalMods(
			mod_names
				.iter()
				.map(
					|mod_name| {
						let wasm_binary = asset_server.load(format!("mods/{mod_name}/component.wasm"));
						let mod_metadata = asset_server.load(format!("mods/{mod_name}/mod.dhall"));
						ProvisionalMod {
							wasm_binary,
							mod_metadata,
						}
					},
				)
				.collect(),
		),
	)
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
}

pub fn get_mod_names() -> Result<Vec<String>> {
	Ok(
		fs::read_dir("./mods")?
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

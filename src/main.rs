mod asset_mod_metadata;
mod asset_tracking;
mod asset_wasm_binary;
mod load_mods;
mod wasm_engine;

use bevy::prelude::*;

use crate::{
	load_mods::Mods,
	wasm_engine::WasmEngine,
};

fn main() -> AppExit {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_plugins(asset_tracking::plugin)
		.add_plugins((
			asset_mod_metadata::plugin,
			asset_wasm_binary::plugin,
		))
		.add_plugins(load_mods::plugin)
		.add_systems(
			Update,
			update_mods.run_if(resource_exists::<Mods>),
		)
		.run()
}

fn update_mods(mods: Res<Mods>, mut wasm_engine: NonSendMut<WasmEngine>) {
	let WasmEngine {
		ref mut store,
		..
	} = *wasm_engine;
	let mods = mods.0.iter();
	for r#mod in mods {
		r#mod
			.bindings
			.call_on_update(
				&mut *store,
				1.0 / 60.0,
			)
			.unwrap_or_else(
				|err| {
					error!(
						"Error calling on_update for mod {}: {}",
						r#mod.mod_metadata.name, err
					);
				},
			);
	}
}

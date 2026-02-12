use bevy::{
	asset::{
		AssetLoader,
		LoadContext,
		io::Reader,
	},
	prelude::*,
};
use thiserror::Error;

pub fn plugin(app: &mut App) {
	app.init_asset_loader::<WasmBinaryLoader>()
		.init_asset::<WasmBinary>();
}

#[derive(Asset, TypePath, Debug)]
pub struct WasmBinary(pub(crate) Vec<u8>);

#[derive(Default, TypePath)]
struct WasmBinaryLoader;

#[non_exhaustive]
#[derive(Debug, Error)]
enum WasmBinaryLoaderError {
	#[error("Could not load asset: {0}")]
	Io(#[from] std::io::Error),
}

impl AssetLoader for WasmBinaryLoader {
	type Asset = WasmBinary;
	type Error = WasmBinaryLoaderError;
	type Settings = ();

	async fn load(
		&self,
		reader: &mut dyn Reader,
		_settings: &(),
		_load_context: &mut LoadContext<'_>,
	) -> Result<Self::Asset, Self::Error> {
		let mut wasm_binary = Vec::new();
		reader
			.read_to_end(&mut wasm_binary)
			.await?;
		Ok(WasmBinary(wasm_binary))
	}

	fn extensions(&self) -> &[&str] {
		&["wasm"]
	}
}

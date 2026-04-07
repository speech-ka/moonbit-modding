use bevy::{
	asset::{
		AssetLoader,
		AsyncReadExt,
		LoadContext,
		io::Reader,
	},
	prelude::*,
};
use serde::Deserialize;
use thiserror::Error;

pub fn plugin(app: &mut App) {
	app.init_asset_loader::<ModMetaDataLoader>()
		.init_asset::<ModMetaData>();
}

#[derive(Asset, TypePath, Deserialize)]
pub struct ModMetaData {
	pub(crate) name: String,
	pub(crate) description: String,
}

#[derive(Default, TypePath)]
struct ModMetaDataLoader;

#[non_exhaustive]
#[derive(Debug, Error)]
enum ModMetaDataLoaderError {
	#[error("Could not load asset: {0}")]
	Io(#[from] std::io::Error),
	#[error("Invalid schema: {0}")]
	SerdeDhall(#[from] serde_dhall::Error),
}

impl AssetLoader for ModMetaDataLoader {
	type Asset = ModMetaData;
	type Error = ModMetaDataLoaderError;
	type Settings = ();

	async fn load(
		&self,
		reader: &mut dyn Reader,
		_settings: &(),
		_load_context: &mut LoadContext<'_>,
	) -> Result<Self::Asset, Self::Error> {
		let mut data = String::new();
		reader
			.read_to_string(&mut data)
			.await?;
		Ok(serde_dhall::from_str(&data).parse()?)
	}

	fn extensions(&self) -> &[&str] {
		&["dhall"]
	}
}

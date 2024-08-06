use bevy::{
    asset::{io::Reader, AssetLoader, AsyncReadExt, LoadContext},
    prelude::*,
};
use thiserror::Error;

use crate::PaperdollAsset;

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum PaperdollLoaderError {
    #[error("Could not load paperdoll source: {0}")]
    Io(#[from] std::io::Error),
    #[error("Could not load paperdoll from source: {0}")]
    Load(#[from] anyhow::Error),
}

/// Bevy asset loader for loading paperdoll asset (.ppd).
#[derive(Default)]
pub struct PaperdollLoader;

impl AssetLoader for PaperdollLoader {
    type Asset = PaperdollAsset;

    type Settings = ();

    type Error = PaperdollLoaderError;

    async fn load<'a>(
        &'a self,
        reader: &'a mut Reader<'_>,
        _settings: &'a Self::Settings,
        _load_context: &'a mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;

        let factory = paperdoll_tar::read(bytes.as_slice())?;

        let paperdoll_asset = PaperdollAsset::new(factory);

        Ok(paperdoll_asset)
    }

    fn extensions(&self) -> &[&str] {
        &[paperdoll_tar::EXTENSION_NAME]
    }
}

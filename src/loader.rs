use bevy::{
    asset::{AssetLoader, AsyncReadExt, LoadContext},
    prelude::*,
    utils::{thiserror, BoxedFuture},
};

use crate::PaperdollAsset;

#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
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

    fn load<'a>(
        &'a self,
        reader: &'a mut bevy::asset::io::Reader,
        _settings: &'a Self::Settings,
        _load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;

            let factory = paperdoll_tar::read(bytes.as_slice())?;

            let paperdoll_asset = PaperdollAsset::new(factory);

            Ok(paperdoll_asset)
        })
    }

    fn extensions(&self) -> &[&str] {
        &[paperdoll_tar::EXTENSION_NAME]
    }
}

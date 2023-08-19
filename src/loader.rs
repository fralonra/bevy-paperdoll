use bevy::{
    asset::{AssetLoader, Error, LoadContext, LoadedAsset},
    utils::BoxedFuture,
};

use crate::PaperdollAsset;

/// Bevy asset loader for loading paperdoll asset (.ppd).
#[derive(Default)]
pub struct PaperdollLoader;

impl AssetLoader for PaperdollLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), Error>> {
        Box::pin(async move {
            let factory = paperdoll_tar::read(bytes)?;

            let paperdoll_asset = PaperdollAsset::new(factory);

            load_context.set_default_asset(LoadedAsset::new(paperdoll_asset));

            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &[paperdoll_tar::EXTENSION_NAME]
    }
}

//! Compile-time embedded assets. Paths passed to GPUI never depend on the
//! process working directory, so packaged applications behave like dev builds.

use anyhow::Result;
use gpui::{AssetSource, SharedString};
use rust_embed::RustEmbed;
use std::borrow::Cow;

#[derive(RustEmbed)]
#[folder = "assets/"]
pub(crate) struct EmbeddedAssets;

impl AssetSource for EmbeddedAssets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        Ok(Self::get(path).map(|asset| asset.data))
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        Ok(Self::iter()
            .filter(|asset| path.is_empty() || asset.starts_with(path))
            .map(|asset| SharedString::from(asset.into_owned()))
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::EmbeddedAssets;
    use gpui::AssetSource;

    #[test]
    fn embeds_the_application_mark() {
        let source = EmbeddedAssets;

        assert!(source.load("mark.svg").unwrap().is_some());
        assert!(
            source
                .list("")
                .unwrap()
                .iter()
                .any(|path| path.as_ref() == "mark.svg")
        );
    }
}

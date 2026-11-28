#![expect(
    clippy::unwrap_used,
    reason = "test support reports a failure by panicking"
)]

use std::sync::LazyLock;

use mcrs_minecraft_client_jar::Directory;
use mcrs_minecraft_client_widget::GuiExtent;
use mcrs_minecraft_engine_registries::test_item_table;
use mcrs_minecraft_font::{Font, FontOptions, InflatedFile};
use mcrs_minecraft_gui_atlas::StitchRequests;
use mcrs_minecraft_gui_atlas::{GuiAtlasParts, build_gui_atlas};
use mcrs_minecraft_item_table::ItemTable;
use mcrs_minecraft_model::atlas::SpriteRegistry;
use mcrs_minecraft_model::item_model_bake::{ItemModels, bake_all};
use mcrs_minecraft_model::resource_set::ResourceSet;

pub static ATLAS: LazyLock<GuiAtlasParts> = LazyLock::new(|| gui_atlas(&StitchRequests::default()));

pub static TABLE: LazyLock<ItemTable> = LazyLock::new(test_item_table);

pub static MODELS: LazyLock<ItemModels> =
    LazyLock::new(|| bake_all(ResourceSet::corpus(), &mut SpriteRegistry::default()).unwrap());

pub const GUI: GuiExtent = GuiExtent::new(427, 240);

#[must_use]
pub fn gui_atlas(requests: &StitchRequests) -> GuiAtlasParts {
    build_gui_atlas(ResourceSet::corpus().files(), corpus_font(), requests).unwrap()
}

#[must_use]
pub fn corpus_font() -> &'static Font {
    static FONT: LazyLock<Font> = LazyLock::new(|| {
        Font::load(
            ResourceSet::corpus().files(),
            FontOptions::default(),
            inflate_zip,
        )
        .unwrap()
        .font
    });
    &FONT
}

fn inflate_zip(archive: &[u8]) -> Result<Vec<InflatedFile>, String> {
    let directory = Directory::of(archive)?;
    directory
        .inflated(archive, |name| name.ends_with(".hex"))
        .map(|file| {
            let (name, bytes) = file?;
            Ok(InflatedFile {
                name: name.to_owned(),
                bytes,
            })
        })
        .collect()
}

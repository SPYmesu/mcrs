use mcrs_minecraft_item_component::Builtin;
use mcrs_minecraft_item_component::BuiltinKey;
use mcrs_minecraft_nbt::tag::NbtTag;
use mcrs_minecraft_resource::ResourceLocation;

use crate::NameLookup;
use crate::SavedError;
use crate::saved::read_value;

pub fn read_saved_component<L: NameLookup>(
    component: ResourceLocation<'_>,
    value: &NbtTag,
    names: &L,
) -> Result<Option<Builtin>, SavedError> {
    match BuiltinKey::from_location(component) {
        Some(key) => read_value(key, value, names).map(Some),
        None => Ok(None),
    }
}

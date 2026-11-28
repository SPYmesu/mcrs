use mcrs_minecraft_item_component::ItemStack;

#[derive(Clone, Debug, PartialEq, Hash)]
pub struct SlotStack {
    pub slot: u8,
    pub stack: ItemStack,
}

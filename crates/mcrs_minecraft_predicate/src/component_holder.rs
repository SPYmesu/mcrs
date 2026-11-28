use mcrs_minecraft_item_component::AttackAnimation;
use mcrs_minecraft_item_component::AttributeModifiers;
use mcrs_minecraft_item_component::BannerPatterns;
use mcrs_minecraft_item_component::Bees;
use mcrs_minecraft_item_component::BlockState;
use mcrs_minecraft_item_component::Builtin;
use mcrs_minecraft_item_component::BundleContents;
use mcrs_minecraft_item_component::ChargedProjectiles;
use mcrs_minecraft_item_component::Container;
use mcrs_minecraft_item_component::ContainerLoot;
use mcrs_minecraft_item_component::CustomData;
use mcrs_minecraft_item_component::CustomModelData;
use mcrs_minecraft_item_component::CustomName;
use mcrs_minecraft_item_component::Damage;
use mcrs_minecraft_item_component::DyedColor;
use mcrs_minecraft_item_component::EnchantmentGlintOverride;
use mcrs_minecraft_item_component::Enchantments;
use mcrs_minecraft_item_component::FireworkExplosion;
use mcrs_minecraft_item_component::Fireworks;
use mcrs_minecraft_item_component::Food;
use mcrs_minecraft_item_component::Instrument;
use mcrs_minecraft_item_component::InteractAnimation;
use mcrs_minecraft_item_component::ItemComponent;
use mcrs_minecraft_item_component::ItemModel;
use mcrs_minecraft_item_component::ItemName;
use mcrs_minecraft_item_component::ItemStack;
use mcrs_minecraft_item_component::LodestoneTracker;
use mcrs_minecraft_item_component::Lore;
use mcrs_minecraft_item_component::MapDecorations;
use mcrs_minecraft_item_component::MapId;
use mcrs_minecraft_item_component::MaxDamage;
use mcrs_minecraft_item_component::MaxStackSize;
use mcrs_minecraft_item_component::OminousBottleAmplifier;
use mcrs_minecraft_item_component::PaintingVariant;
use mcrs_minecraft_item_component::PiercingWeapon;
use mcrs_minecraft_item_component::PotionContents;
use mcrs_minecraft_item_component::Profile;
use mcrs_minecraft_item_component::Rarity;
use mcrs_minecraft_item_component::RepairCost;
use mcrs_minecraft_item_component::StoredEnchantments;
use mcrs_minecraft_item_component::SuspiciousStewEffects;
use mcrs_minecraft_item_component::Tool;
use mcrs_minecraft_item_component::TooltipDisplay;
use mcrs_minecraft_item_component::Trim;
use mcrs_minecraft_item_component::Unbreakable;
use mcrs_minecraft_item_table::ItemTable;

use mcrs_minecraft_item::data_component_type_keys::DataComponentType;

#[derive(Clone, Copy, Debug)]
pub(crate) enum ComponentHolder<'a> {
    Empty,
    Stack {
        stack: &'a ItemStack,
        table: &'a ItemTable,
    },
}

impl<'a> ComponentHolder<'a> {
    pub(crate) fn effective<T: ItemComponent + 'static>(self) -> Option<&'a T> {
        match self {
            Self::Empty => None,
            Self::Stack { stack, table } => table.effective(stack),
        }
    }

    pub(crate) fn has(self, kind: DataComponentType) -> bool {
        match kind {
            DataComponentType::MaxStackSize => self.effective::<MaxStackSize>().is_some(),
            DataComponentType::MaxDamage => self.effective::<MaxDamage>().is_some(),
            DataComponentType::Damage => self.effective::<Damage>().is_some(),
            DataComponentType::Unbreakable => self.effective::<Unbreakable>().is_some(),
            DataComponentType::CustomName => self.effective::<CustomName>().is_some(),
            DataComponentType::ItemName => self.effective::<ItemName>().is_some(),
            DataComponentType::ItemModel => self.effective::<ItemModel>().is_some(),
            DataComponentType::Lore => self.effective::<Lore>().is_some(),
            DataComponentType::Rarity => self.effective::<Rarity>().is_some(),
            DataComponentType::Enchantments => self.effective::<Enchantments>().is_some(),
            DataComponentType::AttributeModifiers => {
                self.effective::<AttributeModifiers>().is_some()
            }
            DataComponentType::TooltipDisplay => self.effective::<TooltipDisplay>().is_some(),
            DataComponentType::RepairCost => self.effective::<RepairCost>().is_some(),
            DataComponentType::Food => self.effective::<Food>().is_some(),
            DataComponentType::Tool => self.effective::<Tool>().is_some(),
            DataComponentType::MapId => self.effective::<MapId>().is_some(),
            DataComponentType::MapDecorations => self.effective::<MapDecorations>().is_some(),
            DataComponentType::Profile => self.effective::<Profile>().is_some(),
            DataComponentType::DyedColor => self.effective::<DyedColor>().is_some(),
            DataComponentType::CustomModelData => self.effective::<CustomModelData>().is_some(),
            DataComponentType::EnchantmentGlintOverride => {
                self.effective::<EnchantmentGlintOverride>().is_some()
            }
            DataComponentType::BlockState => self.effective::<BlockState>().is_some(),
            DataComponentType::LodestoneTracker => self.effective::<LodestoneTracker>().is_some(),
            DataComponentType::Trim => self.effective::<Trim>().is_some(),
            DataComponentType::PotionContents => self.effective::<PotionContents>().is_some(),
            DataComponentType::CustomData => self.effective::<CustomData>().is_some(),
            DataComponentType::SuspiciousStewEffects => {
                self.effective::<SuspiciousStewEffects>().is_some()
            }
            DataComponentType::OminousBottleAmplifier => {
                self.effective::<OminousBottleAmplifier>().is_some()
            }
            DataComponentType::Instrument => self.effective::<Instrument>().is_some(),
            DataComponentType::BannerPatterns => self.effective::<BannerPatterns>().is_some(),
            DataComponentType::Bees => self.effective::<Bees>().is_some(),
            DataComponentType::BundleContents => self.effective::<BundleContents>().is_some(),
            DataComponentType::ChargedProjectiles => {
                self.effective::<ChargedProjectiles>().is_some()
            }
            DataComponentType::Container => self.effective::<Container>().is_some(),
            DataComponentType::ContainerLoot => self.effective::<ContainerLoot>().is_some(),
            DataComponentType::FireworkExplosion => self.effective::<FireworkExplosion>().is_some(),
            DataComponentType::Fireworks => self.effective::<Fireworks>().is_some(),
            DataComponentType::StoredEnchantments => {
                self.effective::<StoredEnchantments>().is_some()
            }
            DataComponentType::PaintingVariant => self.effective::<PaintingVariant>().is_some(),
            DataComponentType::PiercingWeapon => self.effective::<PiercingWeapon>().is_some(),
            DataComponentType::AttackAnimation => self.effective::<AttackAnimation>().is_some(),
            DataComponentType::InteractAnimation => self.effective::<InteractAnimation>().is_some(),
            _ => false,
        }
    }

    pub(crate) fn has_equal(self, expected: &Builtin) -> bool {
        match expected {
            Builtin::MaxStackSize(value) => self.effective() == Some(value),
            Builtin::MaxDamage(value) => self.effective() == Some(value),
            Builtin::Damage(value) => self.effective() == Some(value),
            Builtin::Unbreakable(value) => self.effective() == Some(value),
            Builtin::CustomName(value) => self.effective() == Some(value),
            Builtin::ItemName(value) => self.effective() == Some(value),
            Builtin::ItemModel(value) => self.effective() == Some(value),
            Builtin::Lore(value) => self.effective() == Some(value),
            Builtin::Rarity(value) => self.effective() == Some(value),
            Builtin::Enchantments(value) => self.effective() == Some(value),
            Builtin::AttributeModifiers(value) => self.effective() == Some(value),
            Builtin::TooltipDisplay(value) => self.effective() == Some(value),
            Builtin::RepairCost(value) => self.effective() == Some(value),
            Builtin::Food(value) => self.effective() == Some(value),
            Builtin::Tool(value) => self.effective() == Some(value),
            Builtin::MapId(value) => self.effective() == Some(value),
            Builtin::MapDecorations(value) => self.effective() == Some(value),
            Builtin::Profile(value) => self.effective() == Some(value),
            Builtin::DyedColor(value) => self.effective() == Some(value),
            Builtin::CustomModelData(value) => self.effective() == Some(value),
            Builtin::EnchantmentGlintOverride(value) => self.effective() == Some(value),
            Builtin::BlockState(value) => self.effective() == Some(value),
            Builtin::LodestoneTracker(value) => self.effective() == Some(value),
            Builtin::Trim(value) => self.effective() == Some(value),
            Builtin::PotionContents(value) => self.effective() == Some(value),
            Builtin::CustomData(value) => self.effective() == Some(value),
            Builtin::SuspiciousStewEffects(value) => self.effective() == Some(value),
            Builtin::OminousBottleAmplifier(value) => self.effective() == Some(value),
            Builtin::Instrument(value) => self.effective() == Some(value),
            Builtin::BannerPatterns(value) => self.effective() == Some(value),
            Builtin::Bees(value) => self.effective() == Some(value),
            Builtin::BundleContents(value) => self.effective() == Some(value),
            Builtin::ChargedProjectiles(value) => self.effective() == Some(value),
            Builtin::Container(value) => self.effective() == Some(value),
            Builtin::ContainerLoot(value) => self.effective() == Some(value),
            Builtin::FireworkExplosion(value) => self.effective() == Some(value),
            Builtin::Fireworks(value) => self.effective() == Some(value),
            Builtin::StoredEnchantments(value) => self.effective() == Some(value),
            Builtin::PaintingVariant(value) => self.effective() == Some(value),
            Builtin::PiercingWeapon(value) => self.effective() == Some(value),
            Builtin::AttackAnimation(value) => self.effective() == Some(value),
            Builtin::InteractAnimation(value) => self.effective() == Some(value),
            _ => false,
        }
    }
}

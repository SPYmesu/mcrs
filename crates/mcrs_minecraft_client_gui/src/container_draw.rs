use crate::creative_screen_shown::shows_creative_screen;
use crate::menu_label_text::MenuLabelText;
use crate::menu_screen::{MenuContents, menu_screen};
use crate::menu_stacks::MenuStacks;
use crate::shown_drag_preview::ShownDragPreview;
use bevy::prelude::*;
use mcrs_minecraft_client_world::screen::Screen;
use mcrs_minecraft_container_gesture::DragPreview;
use mcrs_minecraft_engine_inventory_menu::MenuParts;
use mcrs_minecraft_engine_inventory_menu::MenuTitle;
use mcrs_minecraft_engine_inventory_menu::Menus;
use mcrs_minecraft_engine_item::SlotRef;
use mcrs_minecraft_engine_item::Slots;
use mcrs_minecraft_font::{ClientLanguage, Font};
use mcrs_minecraft_game_mode::Abilities;
use mcrs_minecraft_gui_atlas::GuiAtlasLayout;
use mcrs_minecraft_gui_primitives::SlotIcon;
use mcrs_minecraft_gui_primitives::Tooltip;
use mcrs_minecraft_gui_primitives::point;
use mcrs_minecraft_gui_scene::GuiBand;
use mcrs_minecraft_gui_scene::GuiLayer;
use mcrs_minecraft_gui_scene::GuiPointer;
use mcrs_minecraft_gui_scene::GuiSize;
use mcrs_minecraft_inventory_menu::MenuLayout;
use mcrs_minecraft_menu_geometry::MenuGeometry;
use mcrs_minecraft_player_holders::PlayerHolders;

#[derive(Component, Default)]
#[require(GuiLayer = GuiLayer::new(GuiBand::Screen, 3), Tooltip)]
pub struct ContainerLayer;

#[derive(Default)]
pub(crate) struct ContainerStacks {
    open: MenuStacks,
    labels: MenuLabelText,
}

#[cfg_attr(mcrs_lint, mcrs::alloc_sites(14))]
pub(crate) fn container_draw(
    screen: Res<Screen>,
    player: Query<(&PlayerHolders, &Menus, Option<&Abilities>)>,
    menus: Query<(&MenuLayout, &MenuParts, Option<Ref<MenuTitle>>)>,
    holders: Query<&Slots>,
    atlas: Option<Res<GuiAtlasLayout>>,
    font: Option<Res<Font>>,
    language: Option<Res<ClientLanguage>>,
    drag: Option<Res<ShownDragPreview>>,
    icons: Query<&SlotIcon>,
    mut stacks: Local<ContainerStacks>,
    size: Res<GuiSize>,
    pointer: Res<GuiPointer>,
    mut layer: Single<(&mut GuiLayer, &mut Tooltip), With<ContainerLayer>>,
) {
    let (layer, tooltip) = &mut *layer;
    layer.redraw();
    tooltip.clear();
    let stacks = &mut *stacks;
    let Ok((player, player_menus, abilities)) = player.single() else {
        return;
    };
    let occupant = |slot: SlotRef| holders.get(slot.holder).ok()?.get(usize::from(slot.index));
    let icon_of = |stack| icons.get(stack).ok().copied();
    let Some(open) = player_menus.open() else {
        return;
    };
    let Ok((layout, parts, title)) = menus.get(open) else {
        return;
    };
    if let Some(language) = language {
        stacks.labels.follow(open, title, language.into());
    }
    if *screen != Screen::Menu || shows_creative_screen(*screen, player_menus, abilities) {
        return;
    }
    let (Some(atlas), Some(font), Some(geometry)) = (atlas, font, MenuGeometry::of_layout(layout))
    else {
        return;
    };
    stacks.open.collect(layout, parts, player.cursor, occupant);
    let no_drag = DragPreview::default();
    menu_screen(
        layer,
        geometry,
        MenuContents {
            stacks: &stacks.open,
            drag: drag.as_deref().map_or(&no_drag, ShownDragPreview::preview),
            size: *size,
            cursor: point(pointer.point()),
        },
        &stacks.labels,
        &atlas,
        &font,
        icon_of,
    );
}

use crate::batch_glint::{BatchGlint, batch_has_glint};
use crate::gui_batch::GuiBatch;
use crate::gui_batch_build::build_gui_batch;
use crate::gui_glint::{GuiGlint, write_gui_glint};
use crate::panorama_spin::{panorama_shows, spin_panorama};
use bevy::prelude::*;
use bevy::render::RenderApp;
use bevy::render::extract_resource::ExtractResource;
use bevy::render::extract_resource::ExtractResourcePlugin;
use mcrs_minecraft_font::GlyphCache;
use mcrs_minecraft_gui_atlas::GuiImages;
use mcrs_minecraft_gui_primitives::Tooltip;
use mcrs_minecraft_gui_primitives::build_icon_meshes;
use mcrs_minecraft_gui_primitives::{TooltipLayer, draw_tooltip};
use mcrs_minecraft_gui_scene::GuiConfig;
use mcrs_minecraft_gui_scene::GuiFade;
use mcrs_minecraft_gui_scene::GuiLayer;
use mcrs_minecraft_gui_scene::GuiPhase;
use mcrs_minecraft_gui_scene::spawn_layer;
use mcrs_minecraft_gui_scene::{gui_frame_changed, layer_stale};
use mcrs_minecraft_item_icon::IconMeshes;
use mcrs_minecraft_model::item_model_bake::ItemModels;
use mcrs_minecraft_render::PanoramaView;

pub struct GuiRenderPlugin;

impl Plugin for GuiRenderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GuiBatch>()
            .init_resource::<BatchGlint>()
            .init_resource::<GuiGlint>()
            .init_resource::<GlyphCache>()
            .add_plugins(ExtractResourcePlugin::<GuiImages, GuiRenderPlugin>::default())
            .add_plugins(ExtractResourcePlugin::<GuiGlint>::default())
            .add_systems(Startup, spawn_layer::<TooltipLayer>)
            .add_systems(
                Update,
                build_icon_meshes.run_if(resource_exists_and_changed::<ItemModels>),
            )
            .add_systems(
                PostUpdate,
                (
                    draw_tooltip
                        .in_set(GuiPhase::Batch)
                        .before(build_gui_batch)
                        .run_if(
                            layer_stale::<TooltipLayer>()
                                .or_else(any_match_filter::<Changed<Tooltip>>)
                                .or_else(any_component_removed::<Tooltip>),
                        ),
                    build_gui_batch.in_set(GuiPhase::Batch).run_if(
                        any_match_filter::<Changed<GuiLayer>>
                            .or_else(any_component_removed::<GuiLayer>)
                            .or_else(resource_changed::<GuiFade>)
                            .or_else(resource_changed::<GuiConfig>)
                            .or_else(gui_frame_changed())
                            .or_else(resource_exists_and_changed::<ItemModels>)
                            .or_else(resource_exists_and_changed::<IconMeshes>),
                    ),
                    write_gui_glint
                        .in_set(GuiPhase::Batch)
                        .after(build_gui_batch)
                        .run_if(batch_has_glint),
                    spin_panorama
                        .in_set(GuiPhase::Batch)
                        .run_if(resource_exists::<PanoramaView>.and_then(panorama_shows)),
                ),
            );
    }
}

impl ExtractResource<RenderApp, GuiRenderPlugin> for GuiImages {
    type Source = GuiImages;

    fn extract_resource(source: &Self::Source) -> Self {
        *source
    }
}

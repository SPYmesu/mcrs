use bevy::prelude::*;
use mcrs_minecraft_client_world::player::Player;
use mcrs_minecraft_client_world::screen::Screen;
use mcrs_minecraft_render::{PanoramaDraw, PanoramaView};

pub(crate) fn project_panorama_view(
    screen: Res<Screen>,
    players: Query<(), With<Player>>,
    view: ResMut<PanoramaView>,
) {
    let draw = match *screen {
        Screen::Message(_) | Screen::LevelLoading => PanoramaDraw::Blurred,
        _ if !players.is_empty() => PanoramaDraw::Hidden,
        Screen::Title => PanoramaDraw::Plain,
        Screen::None
        | Screen::Menu
        | Screen::GameModeSwitcher(_)
        | Screen::Chat
        | Screen::Death
        | Screen::WorldListError => PanoramaDraw::Hidden,
        Screen::Pause
        | Screen::Options
        | Screen::VideoSettings
        | Screen::AccessibilitySettings(_)
        | Screen::Controls(_)
        | Screen::MouseSettings(_)
        | Screen::SelectWorld
        | Screen::DeleteWorldConfirmation
        | Screen::CreateWorld(_)
        | Screen::Disconnected => PanoramaDraw::Blurred,
    };
    view.map_unchanged(|view| &mut view.draw).set_if_neq(draw);
}

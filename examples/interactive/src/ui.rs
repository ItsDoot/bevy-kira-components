use bevy::app::{App, Plugin};
use bevy::prelude::*;

use bevy_kira_components::sources::audio_file::source::AudioFile;
use bevy_kira_components::sources::AudioHandle;

use crate::InteractiveSound;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, ui_init)
            .add_systems(Update, ui_update);
    }
}

fn ui_init(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                margin: UiRect::all(Val::Px(8.)),
                ..default()
            },
            ..default()
        })
        .with_children(|children| {
            let style = TextStyle {
                font_size: 14.0,
                ..default()
            };
            children.spawn(TextBundle {
                text: Text::from_section(
                    "Hold Space to play a looping sound, press A (on QWERTY keyboards) to send a one-shot sound",
                    style.clone(),
                ),
                ..default()
            });
            children.spawn((
                PlaybackPos,
                TextBundle {
                    text: Text::from_sections([
                        TextSection::new("Playback position: ", style.clone()),
                        TextSection::new("0.0 s", style),
                    ]),
                    ..default()
                },
            ));
        });
}

#[derive(Component)]
struct PlaybackPos;

fn ui_update(
    mut q_ui: Query<&mut Text, With<PlaybackPos>>,
    q_audio: Query<&AudioHandle, With<InteractiveSound>>,
) {
    let mut text = q_ui.single_mut();
    let audio_handle_result = q_audio.get_single();
    if let Ok(handle) = audio_handle_result {
        let pos = handle.position();
        text.sections[1].value = format!("{pos:2.1} s");
    }
}

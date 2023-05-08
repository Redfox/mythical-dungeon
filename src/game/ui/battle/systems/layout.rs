use bevy::core::Name;
use bevy::prelude::*;
use crate::game::player::resources::Player;
use crate::game::ui::battle::styles::{BACKGROUND_COLOR, BATTLE_ACTION_STYLE, IMAGE_STYLE, UI_STYLE};
use crate::game::ui::battle::components::UI;

#[derive(Component)]
pub struct BattleAction {

}

pub fn spawn_ui(
    mut commands: Commands,
    player: Res<Player>,
    asset_server: Res<AssetServer>
) {
    commands
        .spawn((
            NodeBundle {
                style: UI_STYLE,
                ..default()
            },
            UI { },
            Name::new("UI")
        ))
        .with_children(|parent| {
            battle_action(parent, player, asset_server);
        });
}

fn battle_action(parent: &mut ChildBuilder, player: Res<Player>, asset_server: Res<AssetServer>) {
   parent
        .spawn((
            NodeBundle {
                style: BATTLE_ACTION_STYLE,
                background_color: BackgroundColor::from(Color::BLACK),
                ..default()
            },
            BattleAction { }
        ))
        .with_children(|parent| {
            for (i, char) in player.characters.iter().enumerate() {
                let mut style = IMAGE_STYLE;
                style.position = UiRect::new(Val::Percent(5.0 * i as f32), Val::Px(0.0), Val::Px(-50.0 * (i as f32 + 1.)), Val::Px(0.0));

                parent
                .spawn(ImageBundle {
                    style,
                    image: asset_server.load("2x/EXTRAS/Portraits/Owlbear.png").into(),
                    ..default()
                });
            }
        });
}

pub fn despawn_battle_action(
    mut commands: Commands,
    battle_ui: Query<Entity, With<UI>>,
) {
    for entity in battle_ui.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
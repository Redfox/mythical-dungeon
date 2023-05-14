use bevy::prelude::*;

pub const BACKGROUND_COLOR: Color = Color::rgba(0.25, 0.25, 0.25, 0.5);

pub const UI_STYLE: Style = Style {
    display: Display::Flex,
    flex_direction: FlexDirection::Column,
    justify_content: JustifyContent::End,
    align_items: AlignItems::Center,
    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
    ..Style::DEFAULT
};

pub const BATTLE_ACTION_STYLE: Style = Style {
    display: Display::Flex,
    flex_direction: FlexDirection::Column,
    justify_content: JustifyContent::Start,
    align_items: AlignItems::Start,
    size: Size::new(Val::Percent(80.0), Val::Px(50.0)),
    margin: UiRect::new(Val::Px(32.0), Val::Px(0.0), Val::Px(0.0), Val::Px(40.0)),
    ..Style::DEFAULT
};

pub const IMAGE_STYLE: Style = Style {
    size: Size::new(Val::Px(48.0), Val::Px(48.0)),
    margin: UiRect::new(Val::Px(8.0), Val::Px(8.0), Val::Px(8.0), Val::Px(8.0)),
    ..Style::DEFAULT
};

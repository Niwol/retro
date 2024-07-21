use bevy::{app::AppExit, color::palettes, prelude::*};

use crate::application::{CurrentGame, MENU_AREA};

const ITEM_BACKGROUND_NORMAL: Color = Color::BLACK;
const ITEM_BACKGROUND_SELECTED: Color = Color::srgb(0.3, 0.3, 0.3);

const MENU_ITEMS: [&'static str; 4] = ["Breakout", "Tetris", "Settings", "Exit"];

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MenuSelection>()
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (
                    (handle_input_change_selection, handle_input_select),
                    update_menu_apparence,
                )
                    .chain()
                    .run_if(in_state(CurrentGame::InMainMenu)),
            );
    }
}

#[derive(Resource, Default)]
struct MenuSelection {
    selection_index: usize,
    changed: bool,
}

#[derive(Component)]
struct MenuItemComponent;

fn setup(mut commands: Commands, images: Res<AssetServer>) {
    commands
        .spawn(ImageBundle {
            style: Style {
                width: Val::Px(MENU_AREA.width()),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            image: UiImage {
                texture: images.load("menu/menu_background.png"),
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|builder| {
            for menu_item in MENU_ITEMS {
                builder
                    .spawn(
                        TextBundle::from_section(
                            menu_item,
                            TextStyle {
                                font_size: 30.0,
                                color: palettes::basic::GREEN.into(),
                                ..Default::default()
                            },
                        )
                        .with_background_color(ITEM_BACKGROUND_NORMAL),
                    )
                    .insert(MenuItemComponent);
            }
        });

    commands.insert_resource(MenuSelection {
        selection_index: 0,
        changed: true,
    });
}

fn update_menu_apparence(
    menu_selection: ResMut<MenuSelection>,
    mut menu_items: Query<(&mut BackgroundColor, &Text), With<MenuItemComponent>>,
) {
    if !menu_selection.changed {
        return;
    }

    for (mut background_color, menu_text) in &mut menu_items {
        if menu_text.sections[0].value == MENU_ITEMS[menu_selection.selection_index] {
            background_color.0 = ITEM_BACKGROUND_SELECTED;
        } else {
            background_color.0 = ITEM_BACKGROUND_NORMAL;
        }
    }
}

fn handle_input_change_selection(
    input: Res<ButtonInput<KeyCode>>,
    mut menu_selection: ResMut<MenuSelection>,
) {
    if input.just_pressed(KeyCode::ArrowUp) {
        let current_index = menu_selection.selection_index;
        if current_index == 0 {
            menu_selection.selection_index = MENU_ITEMS.len() - 1;
        } else {
            menu_selection.selection_index = current_index - 1;
        }

        menu_selection.changed = true;
    }

    if input.just_pressed(KeyCode::ArrowDown) {
        let current_index = menu_selection.selection_index;
        if current_index == MENU_ITEMS.len() - 1 {
            menu_selection.selection_index = 0;
        } else {
            menu_selection.selection_index = current_index + 1;
        }

        menu_selection.changed = true;
    }
}

fn handle_input_select(
    input: Res<ButtonInput<KeyCode>>,
    mut exit_event: EventWriter<AppExit>,
    menu_selection: ResMut<MenuSelection>,
    mut next_game: ResMut<NextState<CurrentGame>>,
) {
    if !input.just_pressed(KeyCode::Space) {
        return;
    }

    match MENU_ITEMS[menu_selection.selection_index] {
        "Breakout" => {
            next_game.set(CurrentGame::Breakout);
        }
        "Tetris" => {
            next_game.set(CurrentGame::Tetris);
        }
        "Settings" => (),
        "Exit" => {
            exit_event.send(AppExit::Success);
        }
        _ => panic!("Selected impossible menu item"),
    };
}

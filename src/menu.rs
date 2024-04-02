use bevy::{app::AppExit, prelude::*};

use crate::application::{CurrentGame, MENU_AREA};

const ITEM_BACKGROUND_NORMAL: Color = Color::BLACK;
const ITEM_BACKGROUND_SELECTED: Color = Color::DARK_GRAY;

const MENU_ITEMS: [&'static str; 3] = ["Breakout", "Settings", "Exit"];

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MenuSelection>()
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (handle_input_change_selection, handle_input_select)
                    .run_if(in_state(CurrentGame::InMainMenu)),
            );
    }
}

#[derive(Resource)]
struct MenuSelection {
    selected_item: Entity,
}

impl FromWorld for MenuSelection {
    fn from_world(_world: &mut World) -> Self {
        MenuSelection {
            selected_item: Entity::PLACEHOLDER,
        }
    }
}

fn setup(mut commands: Commands, images: Res<AssetServer>) {
    let mut selected_item = Entity::PLACEHOLDER;
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
            selected_item = builder
                .spawn(
                    TextBundle::from_section(
                        "Breakout",
                        TextStyle {
                            font_size: 30.0,
                            color: Color::GREEN,
                            ..Default::default()
                        },
                    )
                    .with_background_color(ITEM_BACKGROUND_SELECTED),
                )
                .id();

            builder.spawn(
                TextBundle::from_section(
                    "Settings",
                    TextStyle {
                        font_size: 30.0,
                        color: Color::GREEN,
                        ..Default::default()
                    },
                )
                .with_background_color(ITEM_BACKGROUND_NORMAL),
            );

            builder.spawn(
                TextBundle::from_section(
                    "Exit",
                    TextStyle {
                        font_size: 30.0,
                        color: Color::GREEN,
                        ..Default::default()
                    },
                )
                .with_background_color(ITEM_BACKGROUND_NORMAL),
            );
        });

    commands.insert_resource(MenuSelection { selected_item });
}

fn handle_input_change_selection(
    input: Res<ButtonInput<KeyCode>>,
    mut menu_selection: ResMut<MenuSelection>,
    mut menu_items: Query<(Entity, &mut BackgroundColor, &Text)>,
) {
    if input.just_pressed(KeyCode::ArrowUp) {
        let (_, mut background, text) = menu_items.get_mut(menu_selection.selected_item).unwrap();
        background.0 = ITEM_BACKGROUND_NORMAL;

        let index = MENU_ITEMS
            .iter()
            .position(|x| *x == &text.sections[0].value)
            .unwrap();

        let next_index = if index == 0 {
            MENU_ITEMS.len() - 1
        } else {
            index - 1
        };

        for (entity, mut background, text) in &mut menu_items {
            if MENU_ITEMS[next_index] == &text.sections[0].value {
                menu_selection.selected_item = entity;
                background.0 = ITEM_BACKGROUND_SELECTED;
            }
        }
    }

    if input.just_pressed(KeyCode::ArrowDown) {
        let (_, mut background, text) = menu_items.get_mut(menu_selection.selected_item).unwrap();
        background.0 = ITEM_BACKGROUND_NORMAL;

        let index = MENU_ITEMS
            .iter()
            .position(|x| *x == &text.sections[0].value)
            .unwrap();

        let next_index = (index + 1) % MENU_ITEMS.len();

        for (entity, mut background, text) in &mut menu_items {
            if MENU_ITEMS[next_index] == &text.sections[0].value {
                menu_selection.selected_item = entity;
                background.0 = ITEM_BACKGROUND_SELECTED;
            }
        }
    }
}

fn handle_input_select(
    input: Res<ButtonInput<KeyCode>>,
    mut exit_event: EventWriter<AppExit>,
    menu_selection: ResMut<MenuSelection>,
    menu_items: Query<&Text>,
    mut next_game: ResMut<NextState<CurrentGame>>,
) {
    if !input.just_pressed(KeyCode::Space) {
        return;
    }

    let text = menu_items.get(menu_selection.selected_item).unwrap();

    let index = MENU_ITEMS
        .iter()
        .position(|x| *x == &text.sections[0].value)
        .unwrap();

    match MENU_ITEMS[index] {
        "Breakout" => {
            println!("Starting breakout");
            next_game.set(CurrentGame::Breakout);
        }
        "Settings" => println!("Settings"),
        "Exit" => {
            println!("Exiting app");
            exit_event.send(AppExit);
        }
        _ => panic!("Selected impossible menu item"),
    };
}

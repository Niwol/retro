use bevy::color::palettes;
use bevy::prelude::*;

use crate::application::{CurrentGame, GAME_AREA, WINDOW_RESOLUTION};

use super::{
    game::{CleanupGameEvent, LoadGameEvent},
    TetrisState,
};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MenuUpdateEvent>()
            .add_systems(OnEnter(CurrentGame::Tetris), load_menu)
            .add_systems(OnExit(CurrentGame::Tetris), cleanup_menu)
            .add_systems(OnEnter(TetrisState::InMenu), show_menu)
            .add_systems(OnExit(TetrisState::InMenu), hide_menu)
            .add_systems(
                Update,
                (handle_input, update_menu)
                    .chain()
                    .run_if(in_state(TetrisState::InMenu)),
            );
    }
}

const ITEM_BACKGROUND_NORMAL: Color = Color::srgba(0.0, 0.0, 0.0, 0.0);
const ITEM_BACKGROUND_SELECTED: Color = Color::srgb(0.3, 0.3, 0.3);

const MAIN_MENU_ITEMS: [&'static str; 3] = ["Play", "View High Scores", "Exit"];
const PAUSE_MENU_ITEMS: [&'static str; 2] = ["Resume", "Main Menu"];

#[derive(Component)]
struct UiRootComponent;

[derive(Component, PartialEq, Eq, Clone, Copy)]
pub enum MenuNodeComponent {
    MainMenu,
    PauseMenu,
}

impl MenuNodeComponent {
    fn get_items(&self) -> &'static [&'static str] {
        match self {
            MenuNodeComponent::MainMenu => &MAIN_MENU_ITEMS,
            MenuNodeComponent::PauseMenu => &PAUSE_MENU_ITEMS,
        }
    }
}

#[derive(Component)]
struct MenuItemComponent;

#[derive(Resource)]
struct MenuState {
    selection_index: usize,
    current_menu_node: MenuNodeComponent,
}

#[derive(Event)]
pub struct MenuUpdateEvent {
    pub new_menu_node: Option<MenuNodeComponent>,
    pub new_selection_index: Option<usize>,
}

fn load_menu(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut update_menu_event: EventWriter<MenuUpdateEvent>,
) {
    let background_handle = assets.load("tetris/menu_background.png");

    let ui_root = commands
        .spawn(ImageBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,

                left: Val::Px(GAME_AREA.min.x + WINDOW_RESOLUTION[0] / 2.0),
                bottom: Val::Px(GAME_AREA.max.y + WINDOW_RESOLUTION[1] / 2.0),
                width: Val::Px(GAME_AREA.width()),
                height: Val::Px(GAME_AREA.height()),

                ..Default::default()
            },
            image: UiImage {
                texture: background_handle,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(UiRootComponent)
        .id();

    let main_menu_node = load_menu_node(&mut commands, MenuNodeComponent::MainMenu);
    load_menu_items(&mut commands, main_menu_node, &MAIN_MENU_ITEMS);

    let pause_menu_node = load_menu_node(&mut commands, MenuNodeComponent::PauseMenu);
    load_menu_items(&mut commands, pause_menu_node, &PAUSE_MENU_ITEMS);

    commands.entity(ui_root).add_child(main_menu_node);
    commands.entity(ui_root).add_child(pause_menu_node);

    commands.insert_resource(MenuState {
        selection_index: 0,
        current_menu_node: MenuNodeComponent::MainMenu,
    });

    update_menu_event.send(MenuUpdateEvent {
        new_selection_index: Some(0),
        new_menu_node: Some(MenuNodeComponent::MainMenu),
    });
}

fn load_menu_node(commands: &mut Commands, menu_node_component: MenuNodeComponent) -> Entity {
    commands
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(menu_node_component)
        .id()
}

fn load_menu_items(commands: &mut Commands, parent_entity: Entity, items: &[&str]) {
    commands.entity(parent_entity).with_children(|builder| {
        for menu_item in items {
            builder
                .spawn(
                    TextBundle::from_section(
                        *menu_item,
                        TextStyle {
                            font_size: 30.0,
                            color: palettes::css::ORANGE.into(),
                            ..Default::default()
                        },
                    )
                    .with_background_color(ITEM_BACKGROUND_NORMAL),
                )
                .insert(MenuItemComponent);
        }
    });
}

fn cleanup_menu(mut commands: Commands, ui_root: Query<Entity, With<UiRootComponent>>) {
    let ui_root = ui_root.single();
    commands.entity(ui_root).despawn_recursive();
}

fn handle_input(
    input: Res<ButtonInput<KeyCode>>,
    menu_state: Res<MenuState>,

    mut update_event: EventWriter<MenuUpdateEvent>,
    mut load_game_event: EventWriter<LoadGameEvent>,
    mut cleanup_game_event: EventWriter<CleanupGameEvent>,

    mut next_current_game: ResMut<NextState<CurrentGame>>,
    mut next_tetris_state: ResMut<NextState<TetrisState>>,
) {
    let current_menu_node = menu_state.current_menu_node;
    let current_items = current_menu_node.get_items();

    for key_code in input.get_just_pressed() {
        match key_code {
            KeyCode::ArrowUp | KeyCode::ArrowDown => {
                let new_selection_index = menu_state.selection_index as i32
                    + if *key_code == KeyCode::ArrowUp { -1 } else { 1 };
                let new_selection_index = ((new_selection_index + current_items.len() as i32)
                    % current_items.len() as i32)
                    as usize;

                update_event.send(MenuUpdateEvent {
                    new_selection_index: Some(new_selection_index),
                    new_menu_node: None,
                });
            }

            KeyCode::Space => match current_menu_node {
                MenuNodeComponent::MainMenu => match current_items[menu_state.selection_index] {
                    "Play" => {
                        next_tetris_state.set(TetrisState::InGame);
                        load_game_event.send_default();
                    }
                    "View High Scores" => {
                        update_event.send(MenuUpdateEvent {
                            new_selection_index: Some(0),
                            new_menu_node: Some(MenuNodeComponent::PauseMenu),
                        });
                    }
                    "Exit" => {
                        next_current_game.set(CurrentGame::InMainMenu);
                        next_tetris_state.set(TetrisState::Exited);
                    }
                    _ => (),
                },

                MenuNodeComponent::PauseMenu => match current_items[menu_state.selection_index] {
                    "Resume" => {
                        next_tetris_state.set(TetrisState::InGame);
                    }
                    "Main Menu" => {
                        update_event.send(MenuUpdateEvent {
                            new_selection_index: Some(0),
                            new_menu_node: Some(MenuNodeComponent::MainMenu),
                        });
                        cleanup_game_event.send_default();
                    }
                    _ => (),
                },
            },
            _ => (),
        }
    }
}

fn update_menu(
    mut menu_state: ResMut<MenuState>,
    mut menu_update_event: EventReader<MenuUpdateEvent>,
    mut menu_nodes: Query<(&mut Style, &MenuNodeComponent)>,
    mut menu_items: Query<(&mut BackgroundColor, &Text), With<MenuItemComponent>>,
) {
    let mut update_events = menu_update_event.read();

    if let Some(update_event) = update_events.next() {
        let mut next_node = menu_state.current_menu_node;
        let mut next_index = menu_state.selection_index;

        if let Some(new_menu_node) = update_event.new_menu_node {
            for (mut menu_node_style, menu_node) in &mut menu_nodes {
                if *menu_node == new_menu_node {
                    menu_node_style.display = Display::default();
                } else {
                    menu_node_style.display = Display::None;
                }
            }

            next_node = new_menu_node;
        }

        if let Some(new_selection_index) = update_event.new_selection_index {
            let current_items = menu_state.current_menu_node.get_items();
            let next_times = next_node.get_items();

            for (mut background_color, text) in &mut menu_items {
                if text.sections[0].value == current_items[menu_state.selection_index] {
                    background_color.0 = ITEM_BACKGROUND_NORMAL;
                }

                if text.sections[0].value == next_times[new_selection_index] {
                    background_color.0 = ITEM_BACKGROUND_SELECTED;
                }
            }

            next_index = new_selection_index;
        }

        menu_state.selection_index = next_index;
        menu_state.current_menu_node = next_node;

        menu_update_event.clear();
    }
}

fn show_menu(mut ui_root: Query<&mut Visibility, With<UiRootComponent>>) {
    if let Ok(mut ui_root_visibility) = ui_root.get_single_mut() {
        *ui_root_visibility = Visibility::Inherited;
    }
}

fn hide_menu(mut ui_root: Query<&mut Visibility, With<UiRootComponent>>) {
    if let Ok(mut ui_root_visibility) = ui_root.get_single_mut() {
        *ui_root_visibility = Visibility::Hidden;
    }
}

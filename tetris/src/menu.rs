use bevy::{color::palettes, prelude::*};

use crate::{
    tetris::{ClearGameEvent, LoadGameEvent},
    TetrisState,
};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<TetrisState>()
            .add_systems(Startup, setup_menu)
            .add_systems(
                Update,
                (handle_input, handle_menu_navigation_input, update_menu)
                    .chain()
                    .run_if(in_state(TetrisState::InMenu)),
            )
            .add_systems(OnEnter(TetrisState::InMenu), show_menu)
            .add_systems(OnExit(TetrisState::InMenu), hide_menu);
    }
}

const ITEM_BACKGROUND_NORMAL: Color = Color::srgb(0.0, 0.0, 0.0);
const ITEM_BACKGROUND_SELECTED: Color = Color::srgb(0.3, 0.3, 0.3);

const MAIN_MENU_ITEMS: [&'static str; 3] = ["Play", "View High Score", "Exit"];
const PAUSE_MENU_ITEMS: [&'static str; 2] = ["Resume", "Main Menu"];
const GAME_OVER_ITEMS: [&'static str; 1] = ["Main Menu"];

#[derive(Component, Clone, Copy, PartialEq, Eq)]
enum MenuNode {
    MainMenu,
    PauseMenu,
    GameOverMenu,
}

#[derive(Component)]
struct UiRootComponent;

#[derive(Component)]
struct MenuItemComponent;

impl MenuNode {
    fn get_items(&self) -> &'static [&'static str] {
        match self {
            MenuNode::MainMenu => &MAIN_MENU_ITEMS,
            MenuNode::PauseMenu => &PAUSE_MENU_ITEMS,
            MenuNode::GameOverMenu => &GAME_OVER_ITEMS,
        }
    }
}

#[derive(Resource)]
struct MenuState {
    menu_node: MenuNode,
    selection_index: usize,
    changed: bool,
}

fn setup_menu(mut commands: Commands) {
    let ui_root = commands
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,

                width: Val::Percent(100.0),
                height: Val::Percent(100.0),

                ..Default::default()
            },
            background_color: BackgroundColor(palettes::basic::GRAY.into()),
            ..Default::default()
        })
        .insert(UiRootComponent)
        .id();

    spawn_menu_node(&mut commands, ui_root, MenuNode::MainMenu);
    spawn_menu_node(&mut commands, ui_root, MenuNode::PauseMenu);
    spawn_menu_node(&mut commands, ui_root, MenuNode::GameOverMenu);

    let menu_state = MenuState {
        menu_node: MenuNode::MainMenu,
        selection_index: 0,
        changed: true,
    };

    commands.insert_resource(menu_state);
}

fn spawn_menu_node(commands: &mut Commands, parent_entity: Entity, menu_node: MenuNode) {
    let node = commands
        .spawn(NodeBundle {
            style: Style {
                display: Display::None,

                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,

                ..Default::default()
            },
            ..Default::default()
        })
        .insert(menu_node)
        .id();

    spawn_menu_items(commands, node, menu_node.get_items());

    commands.entity(parent_entity).add_child(node);
}

fn spawn_menu_items(
    commands: &mut Commands,
    parent_entity: Entity,
    menu_items: &'static [&'static str],
) {
    for item in menu_items {
        let child = commands
            .spawn(
                TextBundle::from_section(
                    *item,
                    TextStyle {
                        font_size: 30.0,
                        color: palettes::basic::PURPLE.into(),
                        ..Default::default()
                    },
                )
                .with_background_color(ITEM_BACKGROUND_NORMAL),
            )
            .insert(MenuItemComponent)
            .id();

        commands.entity(parent_entity).add_child(child);
    }
}

fn show_menu(mut ui_root: Query<&mut Visibility, With<UiRootComponent>>) {
    if let Ok(mut ui_visibility) = ui_root.get_single_mut() {
        *ui_visibility = Visibility::Inherited;
    }
}

fn hide_menu(mut ui_root: Query<&mut Visibility, With<UiRootComponent>>) {
    if let Ok(mut ui_visibility) = ui_root.get_single_mut() {
        *ui_visibility = Visibility::Hidden;
    }
}

fn handle_input(
    input: Res<ButtonInput<KeyCode>>,
    mut menu_state: ResMut<MenuState>,
    mut next_tetris_state: ResMut<NextState<TetrisState>>,

    mut load_game_event: EventWriter<LoadGameEvent>,
    mut clear_game_event: EventWriter<ClearGameEvent>,
) {
    let menu_items = menu_state.menu_node.get_items();
    let selected_value = menu_items[menu_state.selection_index];

    match &menu_state.menu_node {
        MenuNode::MainMenu => {
            if !input.just_pressed(KeyCode::Space) {
                return;
            }

            match selected_value {
                "Play" => {
                    next_tetris_state.set(TetrisState::InGame);
                    load_game_event.send_default();
                }
                "View High Score" => (),
                "Exit" => (),

                _ => unreachable!("Unknown menu item: {}", selected_value),
            }
        }

        MenuNode::PauseMenu => {
            if input.just_pressed(KeyCode::Escape) {
                next_tetris_state.set(TetrisState::InGame);
            }

            if !input.just_pressed(KeyCode::Space) {
                return;
            }

            match menu_items[menu_state.selection_index] {
                "Resume" => {
                    next_tetris_state.set(TetrisState::InGame);
                }
                "Main Menu" => {
                    menu_state.menu_node = MenuNode::MainMenu;
                    menu_state.selection_index = 0;
                    menu_state.changed = true;

                    clear_game_event.send_default();
                }

                _ => unreachable!("Unknown menu item: {}", selected_value),
            }
        }

        MenuNode::GameOverMenu => {}
    }
}

fn handle_menu_navigation_input(
    input: Res<ButtonInput<KeyCode>>,
    mut menu_state: ResMut<MenuState>,
) {
    let direction = if input.just_pressed(KeyCode::ArrowUp) {
        -1
    } else if input.just_pressed(KeyCode::ArrowDown) {
        1
    } else {
        return;
    };

    let nb_menu_items = menu_state.menu_node.get_items().len() as i32;
    let next_index =
        (menu_state.selection_index as i32 + direction + nb_menu_items) % nb_menu_items;

    menu_state.selection_index = next_index as usize;
    menu_state.changed = true;
}

fn update_menu(
    mut menu_state: ResMut<MenuState>,
    mut menu_nodes: Query<(&mut Style, &MenuNode)>,
    mut menu_itmes: Query<(&mut BackgroundColor, &Text), With<MenuItemComponent>>,
) {
    if menu_state.changed {
        for (mut style, menu_node) in &mut menu_nodes {
            if *menu_node == menu_state.menu_node {
                style.display = Display::default();
            } else {
                style.display = Display::None;
            }
        }

        let current_menu_items = menu_state.menu_node.get_items();
        for (mut background, text) in &mut menu_itmes {
            if text.sections[0].value == current_menu_items[menu_state.selection_index] {
                background.0 = ITEM_BACKGROUND_SELECTED;
            } else {
                background.0 = ITEM_BACKGROUND_NORMAL;
            }
        }

        menu_state.changed = false;
    }
}

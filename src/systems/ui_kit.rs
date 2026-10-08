//! Shared UI kit: Zellij-inspired keyboard-first navigation helpers.
//!
//! Two UI primitives every screen is built from:
//!
//! * **Key bar** ([`spawn_key_bar`]) - a full-width bottom strip showing
//!   `[key] action` pairs for the current context (Layer 1). Always visible in
//!   top-level contexts; a modal covers it and shows its own footer hints
//!   while open (Layer 2 = the full-screen menu itself).
//! * **Modal scaffolding** ([`spawn_modal`]) - one builder for every modal
//!   screen: responsive panel (px sizing with viewport-unit caps), optionally
//!   scrollable content area, and a pinned footer hint row using the same
//!   `[key] action` format.
//!
//! Visual standards (colors, typography) follow the ui-standards knowledge
//! doc and intentionally stay unchanged.

use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;

use crate::resources::game_state::GameState;
use crate::resources::turn_state::TurnPhase;
use crate::systems::base_hub_ui::BaseHubMode;

// --- Shared palette (ui-standards) ---
pub const COL_KEY: Color = Color::srgb(0.9, 0.9, 0.3); // key chips / highlight (yellow)
pub const COL_ACTION: Color = Color::srgb(0.65, 0.65, 0.65); // bar action labels (dim gray)
pub const COL_TEXT: Color = Color::srgb(0.8, 0.8, 0.8); // neutral body text
pub const COL_SUCCESS: Color = Color::srgb(0.3, 0.9, 0.3);
pub const COL_FAIL: Color = Color::srgb(0.9, 0.3, 0.3);
pub const COL_PANEL: Color = Color::srgb(0.15, 0.15, 0.15);
pub const COL_PANEL_INNER: Color = Color::srgb(0.1, 0.1, 0.1);
pub const COL_BORDER: Color = Color::srgb(0.5, 0.5, 0.5);
pub const COL_BORDER_DIM: Color = Color::srgb(0.35, 0.35, 0.35);
pub const COL_SEPARATOR: Color = Color::srgb(0.4, 0.4, 0.4);
pub const OVERLAY_ALPHA: f32 = 0.8;

// --- Typography (ui-standards) ---
pub const FONT_TITLE: f32 = 24.0;
pub const FONT_SUB: f32 = 16.0;
pub const FONT_BODY: f32 = 18.0;
pub const FONT_BAR: f32 = 14.0;

/// Height of the bottom key bar strip; HUD and other bottom-anchored UI must
/// sit above this.
pub const KEY_BAR_HEIGHT: f32 = 28.0;

/// Pixels one wheel notch scrolls a [`ModalScrollArea`].
const WHEEL_STEP_PX: f32 = 24.0;

// --- Markers ---

/// Root node of the bottom key bar strip.
#[derive(Component)]
pub struct KeyBarRoot;

/// Full-screen dark overlay root of a modal.
#[derive(Component)]
pub struct ModalRoot;

/// The centered panel inside a modal overlay.
#[derive(Component)]
pub struct ModalPanel;

/// Footer hint row of a modal (always visible, pinned to panel bottom).
#[derive(Component)]
pub struct ModalFooter;

/// Scrollable content area; target of [`modal_wheel_scroll_system`].
#[derive(Component)]
pub struct ModalScrollArea;

// --- Key hints ---

/// One `[key] action` pair shown in the key bar or a modal footer.
#[derive(Clone, Copy)]
pub struct KeyHint {
    /// Key name as shown to the player, e.g. "WASD", "E", "Tab", "Esc".
    pub keys: &'static str,
    /// Short action label, e.g. "Move", "Inspect".
    pub action: &'static str,
}

pub const fn hint(keys: &'static str, action: &'static str) -> KeyHint {
    KeyHint { keys, action }
}

/// Spawns one `[key] action` pair as a horizontal row.
fn spawn_key_hint(parent: &mut ChildSpawnerCommands, hint: &KeyHint, key_font: f32, action_font: f32) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(6.0),
            align_items: AlignItems::Center,
            ..default()
        })
        .with_children(|row| {
            row.spawn((
                Text::new(format!("[{}]", hint.keys)),
                TextFont {
                    font_size: key_font,
                    ..default()
                },
                TextColor(COL_KEY),
            ));
            row.spawn((
                Text::new(hint.action),
                TextFont {
                    font_size: action_font,
                    ..default()
                },
                TextColor(COL_ACTION),
            ));
        });
}

/// Spawns the full-width bottom key bar with the given hints (Layer 1).
/// `context` is one of the `BAR_CTX_*` constants; [`maintain_key_bar_system`]
/// uses it to detect when the bar must be re-spawned for a new context.
pub fn spawn_key_bar(commands: &mut Commands, hints: &[KeyHint], context: u32) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(0.0),
                left: Val::Px(0.0),
                right: Val::Px(0.0),
                height: Val::Px(KEY_BAR_HEIGHT),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                column_gap: Val::Px(26.0),
                padding: UiRect::horizontal(Val::Px(12.0)),
                overflow: Overflow::clip_y(),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.85)),
            KeyBarRoot,
            KeyBarContext(context),
            ZIndex(90),
        ))
        .with_children(|bar| {
            for hint in hints {
                spawn_key_hint(bar, hint, FONT_BAR, FONT_BAR);
            }
        });
}

/// Despawns every key bar currently alive.
pub fn despawn_key_bar(commands: &mut Commands, bars: &Query<Entity, With<KeyBarRoot>>) {
    for entity in bars.iter() {
        commands.entity(entity).despawn();
    }
}

/// Spawns a centered row of `[key] action` hints (modal footers, panel
/// footers). Same visual language as the key bar, one size up.
pub fn spawn_hint_row(parent: &mut ChildSpawnerCommands, hints: &[KeyHint], font_size: f32) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            column_gap: Val::Px(22.0),
            width: Val::Percent(100.0),
            ..default()
        })
        .with_children(|row| {
            for hint in hints {
                spawn_key_hint(row, hint, font_size, font_size);
            }
        });
}

// --- Modal scaffolding ---

/// Spawns a modal: full-screen overlay + responsive centered panel with a
/// title, a content area built by `build_content`, and a pinned footer hint
/// row.
///
/// Panel sizing: fixed `min_width` px wide (never narrower), capped at
/// `max_width_vw` viewport percent and 88% viewport height. If
/// `content_scrollable` is true the content area overflows with scroll (and
/// responds to the mouse wheel via [`modal_wheel_scroll_system`]).
///
/// Returns the overlay root entity.
pub fn spawn_modal(
    commands: &mut Commands,
    marker: impl Bundle,
    title: &str,
    ideal_width: f32,
    min_width: f32,
    max_width_vw: f32,
    content_scrollable: bool,
    build_content: impl FnOnce(&mut ChildSpawnerCommands),
    hints: &[KeyHint],
) -> Entity {
    spawn_modal_ex(
        commands,
        marker,
        title,
        ideal_width,
        min_width,
        max_width_vw,
        content_scrollable,
        build_content,
        hints,
        COL_BORDER,
        OVERLAY_ALPHA,
        COL_PANEL,
    )
}

/// [`spawn_modal`] with explicit accent styling (e.g. the death screen's red
/// border and darker overlay). All other behavior identical.
#[allow(clippy::too_many_arguments)]
pub fn spawn_modal_ex(
    commands: &mut Commands,
    marker: impl Bundle,
    title: &str,
    ideal_width: f32,
    min_width: f32,
    max_width_vw: f32,
    content_scrollable: bool,
    build_content: impl FnOnce(&mut ChildSpawnerCommands),
    hints: &[KeyHint],
    border: Color,
    overlay_alpha: f32,
    panel: Color,
) -> Entity {
    let overlay = commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, overlay_alpha)),
            marker,
            ModalRoot,
            ZIndex(100),
        ))
        .id();

    // Build the panel as a child of the overlay.
    commands.entity(overlay).with_children(|parent| {
        parent
            .spawn((
                Node {
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(32.0)),
                    row_gap: Val::Px(14.0),
                    // Definite ideal width (percent children resolve against
                    // it), shrunk on small windows down to min_width and
                    // capped at max_width_vw of the viewport.
                    width: Val::Px(ideal_width),
                    min_width: Val::Px(min_width),
                    max_width: Val::Vw(max_width_vw),
                    max_height: Val::Vh(88.0),
                    ..default()
                },
                BackgroundColor(panel),
                BorderColor(border),
                ModalPanel,
            ))
            .with_children(|panel| {
                // Title
                panel.spawn((
                    Text::new(title),
                    TextFont {
                        font_size: FONT_TITLE,
                        ..default()
                    },
                    TextColor(COL_KEY),
                ));

                // Separator under the title
                panel.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(2.0),
                        ..default()
                    },
                    BackgroundColor(COL_SEPARATOR),
                ));

                // Content area (optionally scrollable)
                if content_scrollable {
                    panel
                        .spawn((
                            Node {
                                flex_direction: FlexDirection::Column,
                                row_gap: Val::Px(10.0),
                                width: Val::Percent(100.0),
                                flex_grow: 1.0,
                                overflow: Overflow::scroll_y(),
                                ..default()
                            },
                            // bevy 0.16: ScrollPosition must be inserted
                            // explicitly on scrollable nodes.
                            ScrollPosition::DEFAULT,
                            ModalScrollArea,
                        ))
                        .with_children(build_content);
                } else {
                    panel
                        .spawn(Node {
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(10.0),
                            width: Val::Percent(100.0),
                            flex_grow: 1.0,
                            ..default()
                        })
                        .with_children(build_content);
                }

                // Pinned footer: separator + hint row
                panel
                    .spawn((
                        Node {
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(8.0),
                            width: Val::Percent(100.0),
                            flex_shrink: 0.0,
                            ..default()
                        },
                        ModalFooter,
                    ))
                    .with_children(|footer| {
                        footer.spawn((
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Px(2.0),
                                ..default()
                            },
                            BackgroundColor(COL_BORDER_DIM),
                        ));
                        footer
                            .spawn(Node {
                                flex_direction: FlexDirection::Row,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                column_gap: Val::Px(22.0),
                                width: Val::Percent(100.0),
                                ..default()
                            })
                            .with_children(|hint_row| {
                                for hint in hints {
                                    spawn_key_hint(hint_row, hint, FONT_SUB, FONT_SUB);
                                }
                            });
                    });
            });
    });

    overlay
}

/// Standard body text row for modal content.
pub fn modal_text(
    parent: &mut ChildSpawnerCommands,
    text: impl Into<String>,
    font_size: f32,
    color: Color,
) {
    parent.spawn((
        Text::new(text),
        TextFont {
            font_size,
            ..default()
        },
        TextColor(color),
    ));
}

/// Adjusts a scroll container so the selected row is visible.
///
/// `selected` is the row index, `row_height` the uniform row height in
/// logical px, `view_height` the visible height of the scroll area in logical
/// px. No-op when the row is already fully visible.
pub fn scroll_selection_into_view(
    scroll: &mut ScrollPosition,
    content_height: f32,
    view_height: f32,
    selected: usize,
    row_height: f32,
) {
    if row_height <= 0.0 || view_height <= 0.0 {
        return;
    }
    let target_top = selected as f32 * row_height;
    let target_bottom = target_top + row_height;
    let offset = scroll.offset_y;

    let new_offset = if target_top < offset {
        target_top
    } else if target_bottom > offset + view_height {
        target_bottom - view_height
    } else {
        offset // already visible
    };

    let max_offset = (content_height - view_height).max(0.0);
    scroll.offset_y = new_offset.clamp(0.0, max_offset);
}

// --- Context-aware key bar maintenance (Layer 1) ---

pub const BAR_CTX_NONE: u32 = 0;
pub const BAR_CTX_EDITOR: u32 = 1;
pub const BAR_CTX_ZONE: u32 = 2;
pub const BAR_CTX_HUB_STASH: u32 = 3;
pub const BAR_CTX_HUB_CONTRACTS: u32 = 4;

/// Marks which context a key bar was spawned for, so
/// [`maintain_key_bar_system`] can re-spawn it when the context changes.
#[derive(Component)]
pub struct KeyBarContext(pub u32);

pub const EDITOR_HINTS: &[KeyHint] = &[
    hint("F2", "Play"),
    hint("Tab", "Mode"),
    hint("1-6", "Tool"),
    hint("LMB", "Place"),
    hint("Wheel", "Zoom"),
    hint("F3", "Save"),
    hint("F4", "Load"),
    hint("Esc", "Quit"),
];

pub const ZONE_HINTS: &[KeyHint] = &[
    hint("WASD", "Move"),
    hint("E", "Inspect"),
    hint("Tab", "Bag"),
    hint("Q", "Bolt"),
    hint("F2", "Editor"),
    hint("Esc", "Quit"),
];

pub const HUB_STASH_HINTS: &[KeyHint] = &[
    hint("W/S", "Select"),
    hint("A/D", "Panel"),
    hint("E", "Move"),
    hint("Tab", "Contracts"),
    hint("Space", "Enter Zone"),
    hint("Esc", "Quit"),
];

pub const HUB_CONTRACTS_HINTS: &[KeyHint] = &[
    hint("Tab", "Stash"),
    hint("Esc", "Quit"),
];

fn bar_context_and_hints(
    game_state: &GameState,
    hub_mode: &BaseHubMode,
    turn_phase: &TurnPhase,
) -> (u32, &'static [KeyHint]) {
    match (game_state, hub_mode, turn_phase) {
        (GameState::Editing, _, _) => (BAR_CTX_EDITOR, EDITOR_HINTS),
        (GameState::Running, _, TurnPhase::PlayerTurn) => (BAR_CTX_ZONE, ZONE_HINTS),
        (GameState::InBaseHub, BaseHubMode::StashManagement, _) => (BAR_CTX_HUB_STASH, HUB_STASH_HINTS),
        (GameState::InBaseHub, BaseHubMode::Contracts, _) => (BAR_CTX_HUB_CONTRACTS, HUB_CONTRACTS_HINTS),
        // Modal phases and any other state: no bar (the modal's footer hints
        // take over - Layer 2 replaces Layer 1 while open).
        _ => (BAR_CTX_NONE, &[]),
    }
}

/// Keeps exactly one key bar alive, matching the current context.
/// Runs every frame; self-heals after state changes and modal open/close
/// without depending on system ordering.
pub fn maintain_key_bar_system(
    mut commands: Commands,
    bars: Query<(Entity, &KeyBarContext), With<KeyBarRoot>>,
    game_state: Res<State<GameState>>,
    hub_mode: Res<State<BaseHubMode>>,
    turn_phase: Res<State<TurnPhase>>,
) {
    let (wanted, hints) = bar_context_and_hints(game_state.get(), hub_mode.get(), turn_phase.get());

    match bars.get_single() {
        Ok((_entity, existing)) if existing.0 == wanted => {
            // Up to date.
        }
        Ok((entity, _)) => {
            commands.entity(entity).despawn();
            if wanted != BAR_CTX_NONE {
                spawn_key_bar(&mut commands, hints, wanted);
            }
        }
        Err(_) => {
            // No bar exists (or more than one - heal by rebuilding).
            for (entity, _) in bars.iter() {
                commands.entity(entity).despawn();
            }
            if wanted != BAR_CTX_NONE {
                spawn_key_bar(&mut commands, hints, wanted);
            }
        }
    }
}

/// Run condition: true when the quit-confirm modal is not open. Apply to
/// game input systems so keys never act underneath it. Phase-scoped modals
/// (inspect, inventory, briefing, ...) don't need this gate - their host
/// groups are already phase-scoped, and the base hub screens MUST NOT be
/// gated on their own presence (they are the active screen).
pub fn no_modal_open() -> impl Condition<()> + Clone {
    not(any_with_component::<QuitConfirmUiRoot>)
}

/// Run condition for camera zoom: the mouse wheel zooms the camera only when
/// it is not needed for UI scrolling (base hub panels, list modals) and no
/// quit confirm is open.
pub fn zoom_allowed(
    game_state: Res<State<GameState>>,
    turn_phase: Res<State<TurnPhase>>,
    quit: Query<(), With<QuitConfirmUiRoot>>,
) -> bool {
    if !quit.is_empty() {
        return false;
    }
    match game_state.get() {
        GameState::Editing => true,
        GameState::InBaseHub => false, // wheel scrolls hub panels
        GameState::Running => matches!(turn_phase.get(), TurnPhase::PlayerTurn),
    }
}

// --- Quit confirmation (ESC hardening) ---

/// State of the quit-confirm modal. `just_opened` swallows the same ESC
/// keypress that opened the modal so it is not immediately closed again.
#[derive(Resource, Default)]
pub struct QuitConfirmState {
    pub root: Option<Entity>,
    pub just_opened: bool,
}

/// Marker for the quit-confirm modal root.
#[derive(Component)]
pub struct QuitConfirmUiRoot;

pub const QUIT_CONFIRM_HINTS: &[KeyHint] = &[hint("E", "Quit"), hint("Esc", "Stay")];

fn spawn_quit_confirm_modal(commands: &mut Commands) -> Entity {
    spawn_modal(
        commands,
        QuitConfirmUiRoot,
        "Quit Picnic?",
        460.0,
        360.0,
        60.0,
        false,
        |content| {
            modal_text(
                content,
                "Quitting resets all progress: stash, run inventory and\ncontracts return to the starter loadout.",
                FONT_BODY,
                COL_TEXT,
            );
        },
        QUIT_CONFIRM_HINTS,
    )
}

/// Opens the quit-confirm modal on ESC in top-level contexts.
/// Replaces the old instant-quit `exit_on_escape_system`.
pub fn escape_menu_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    game_state: Res<State<GameState>>,
    turn_phase: Res<State<TurnPhase>>,
    mut quit: ResMut<QuitConfirmState>,
    mut commands: Commands,
) {
    if !keyboard.just_pressed(KeyCode::Escape) || quit.root.is_some() {
        return;
    }
    let top_level = match (game_state.get(), turn_phase.get()) {
        (GameState::Editing, _) => true,
        (GameState::Running, TurnPhase::PlayerTurn) => true,
        (GameState::InBaseHub, _) => true,
        // Modal phases own their ESC (close modal); PlayerDead requires E.
        _ => false,
    };
    if !top_level {
        return;
    }
    info!("[UI] ESC pressed - opening quit confirmation");
    quit.root = Some(spawn_quit_confirm_modal(&mut commands));
    quit.just_opened = true;
}

/// Resolves the quit-confirm modal: E quits, ESC stays.
pub fn quit_confirm_resolve_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut quit: ResMut<QuitConfirmState>,
    mut commands: Commands,
    mut exit: EventWriter<AppExit>,
) {
    let Some(root) = quit.root else {
        return;
    };
    if quit.just_opened {
        quit.just_opened = false;
        return;
    }
    if keyboard.just_pressed(KeyCode::KeyE) {
        info!("[UI] Quit confirmed - exiting");
        if let Ok(mut e) = commands.get_entity(root) {
            e.despawn();
        }
        quit.root = None;
        exit.write(AppExit::Success);
    } else if keyboard.just_pressed(KeyCode::Escape) {
        info!("[UI] Quit cancelled");
        if let Ok(mut e) = commands.get_entity(root) {
            e.despawn();
        }
        quit.root = None;
    }
}

/// Mouse-wheel scrolling for [`ModalScrollArea`] nodes under the cursor
/// (bevy_ui does not scroll overflow nodes by itself). Hit-testing mirrors
/// bevy_ui's own picking backend: node center from `GlobalTransform`, extent
/// from `ComputedNode.size()`.
pub fn modal_wheel_scroll_system(
    mut wheel_events: EventReader<MouseWheel>,
    mut scrollables: Query<(
        &GlobalTransform,
        &ComputedNode,
        &mut ScrollPosition,
    ), With<ModalScrollArea>>,
    windows: Query<&Window>,
) {
    let events: Vec<MouseWheel> = wheel_events.read().copied().collect();
    if events.is_empty() {
        return;
    }
    let Ok(window) = windows.get_single() else {
        return;
    };
    let Some(cursor) = window.cursor_position() else {
        return;
    };

    for (transform, node, mut scroll) in scrollables.iter_mut() {
        // ComputedNode sizes are physical px; convert to logical so they match
        // the cursor position space (scale factor is 1.0 in this game anyway).
        let logical_size = node.size() * node.inverse_scale_factor;
        let rect = Rect::from_center_size(transform.translation().truncate(), logical_size);
        if rect.size() == Vec2::ZERO || !rect.contains(cursor) {
            continue;
        }
        let logical_content = node.content_size.y * node.inverse_scale_factor;
        let max_offset = (logical_content - logical_size.y).max(0.0);
        for ev in &events {
            scroll.offset_y = (scroll.offset_y - ev.y * WHEEL_STEP_PX).clamp(0.0, max_offset);
        }
    }
}

// --- UI debug dump (F10) - temporary diagnostic tool ---

/// Prints the whole UI tree: entity id, computed position (top-left, logical),
/// and computed size. Bound to F10 for validating modal geometry.
pub fn debug_ui_dump_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut nodes: Query<
        (
            Entity,
            &GlobalTransform,
            &ComputedNode,
            &ChildOf,
            Option<&ScrollPosition>,
        ),
        With<Node>,
    >,
    names: Query<&Name>,
) {
    if !keyboard.just_pressed(KeyCode::F10) {
        return;
    }
    info!("==== UI TREE DUMP ====");
    let mut list: Vec<_> = nodes
        .iter()
        .map(|(e, t, n, _p, scroll)| {
            let pos = t.translation().truncate();
            let inv = n.inverse_scale_factor;
            (
                e,
                (pos.x, pos.y),
                (n.size().x * inv, n.size().y * inv),
                n.content_size().x * inv,
                n.content_size().y * inv,
                scroll.map(|s| s.offset_y).unwrap_or(f32::NAN),
            )
        })
        .collect();
    list.sort_by_key(|(_, pos, _, _, _, _)| (pos.1 as i32, pos.0 as i32));
    for (e, pos, size, cw, ch, scroll) in list {
        let name = names.get(e).map(|n| n.as_str()).unwrap_or("-");
        let scroll_str = if scroll.is_nan() {
            String::new()
        } else {
            format!(" scroll_y={:.0}", scroll)
        };
        info!(
            "  e={:?} pos=({:.0},{:.0}) size=({:.0}x{:.0}) content=({:.0}x{:.0}){} name={}",
            e, pos.0, pos.1, size.0, size.1, cw, ch, scroll_str, name
        );
    }
    info!("==== END UI TREE DUMP ====");
}

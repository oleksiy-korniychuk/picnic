use bevy::prelude::States;

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    Running,
    #[default]
    Editing,
}
use bevy::prelude::*;
use crate::parser::{GameBoard, GuardDirection};

pub struct GuardPlugin;

impl Plugin for GuardPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<InitGuard>()
            .add_systems(Update, (draw_guard, spawn_guard));
    }
}

#[derive(Resource, Debug, Clone)]
pub struct Guard {
    direction: GuardDirection,
    x: f32,
    y: f32,
}

impl Guard {
    fn translation(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
    
    fn from_init_event(event: &InitGuard) -> Self {
        Self {
            direction: event.direction,
            x: event.x,
            y: event.y,
        }
    }
}

fn draw_guard(mut gizmos: Gizmos, guard: Res<Guard>) {
    let length = 35.0;
    let end: Vec2 = match guard.direction {
        GuardDirection::Up => {[guard.x, guard.y  + length].into()},
        GuardDirection::Down => {[guard.x, guard.y - length].into()}
        GuardDirection::Right => {[guard.x + length, guard.y].into()}
        GuardDirection::Left => {[guard.x - length, guard.y].into()}
    };
    gizmos.arrow_2d(
        guard.translation(), end.into(), Color::srgb(3.0,3.0,3.0)
    );
}



#[derive(Debug, Event)]
pub struct InitGuard {
    x: f32,
    y: f32,
    direction: GuardDirection,
}

impl InitGuard {
    pub fn from_game_board(game_board: &GameBoard) -> Self {
        let offset = 48.0;
        Self {
            direction: game_board.guard.direction,
            x: game_board.guard.position.0 as f32 * offset - offset * game_board.width as f32 / 2.0,
            y: game_board.guard.position.1 as f32 * -offset + offset * game_board.width as f32 / 2.0,
        }
    }
}

fn spawn_guard(mut init_guard: EventReader<InitGuard>, mut commands: Commands) {
    for event in init_guard.read() {
        let guard = Guard::from_init_event(event);
        commands.insert_resource(guard)
    }
}

fn walk_guard(mut guard: ResMut<Guard>, mut game_board: ResMut<GameBoard>) {
    
}
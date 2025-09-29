use bevy::prelude::*;

mod component {
    pub mod position;
    pub mod snake;
}
mod entity {
    pub mod snake;
}
mod system {
    pub mod movement;
    pub mod rendering;
    pub mod input;
}

use entity::snake::{crear_serpiente, crear_camara};
use system::movement::{sistema_movimiento, sistema_colisiones, TemporizadorMovimiento};
use system::rendering::sistema_renderizar;
use system::input::sistema_entrada;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Juego de la Serpiente".to_string(),
                resolution: (500.0, 500.0).into(),
                ..default()
            }),
            ..default()
        }))
        .init_resource::<TemporizadorMovimiento>()
        .add_systems(Startup, (crear_camara, crear_serpiente))
        .add_systems(Update, (
            sistema_entrada,
            sistema_movimiento,
            sistema_colisiones.after(sistema_movimiento),
            sistema_renderizar.after(sistema_movimiento),
        ))
        .run();
}

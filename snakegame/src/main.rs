use bevy::prelude::*;

mod component {
    pub mod position;
    pub mod snake;
    pub mod food;
    pub mod game;
}
mod entity {
    pub mod snake;
}
mod system {
    pub mod rendering;
    pub mod spawn;
    pub mod ui;
    pub mod snake;
}
mod resources;

use entity::snake::{crear_serpiente, crear_camara};
use system::rendering::sistema_renderizar;
use system::spawn::{sistema_generar_comida, sistema_generar_bordes};
use system::ui::{sistema_ui_setup, sistema_ui_actualizar};
use system::snake::{
    Serpiente, 
    sistema_movimiento_unificado, 
    sistema_colisiones_unificado, 
    sistema_entrada_unificado,
    sistema_reinicio_entrada,
    sistema_reiniciar_unificado
};
use resources::{TemporizadorMovimiento, EstadoJuego, Score};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Juego de la Serpiente".to_string(),
                resolution: (500.0, 600.0).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(bevy::winit::WinitSettings::game())
        .init_state::<EstadoJuego>()
        .init_resource::<TemporizadorMovimiento>()
        .init_resource::<Score>()
        .init_resource::<Serpiente>()
        .add_systems(Startup, (
            crear_camara, 
            crear_serpiente,
            sistema_generar_bordes,
            sistema_ui_setup,
        ))
        .add_systems(Update, (
            // Sistemas de entrada (separados por estado)
            sistema_entrada_unificado.run_if(in_state(EstadoJuego::Jugando)),
            sistema_reinicio_entrada.run_if(in_state(EstadoJuego::GameOver)),
            
            // Sistemas de lógica de juego (en orden específico)
            sistema_movimiento_unificado.run_if(in_state(EstadoJuego::Jugando)),
            sistema_colisiones_unificado
                .after(sistema_movimiento_unificado)
                .run_if(in_state(EstadoJuego::Jugando)),
            sistema_generar_comida
                .after(sistema_colisiones_unificado)
                .run_if(in_state(EstadoJuego::Jugando)),
            
            // Sistemas de renderizado y UI (después de la lógica)
            sistema_renderizar
                .after(sistema_movimiento_unificado)
                .after(sistema_generar_comida),
            sistema_ui_actualizar,
            
            // Sistema de reinicio (para todos los estados)
            sistema_reiniciar_unificado,
        ))
        .run();
}

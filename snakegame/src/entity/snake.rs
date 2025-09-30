use bevy::prelude::*;
use crate::component::position::Posicion;
use crate::component::snake::{CabezaSerpiente, CuerpoSerpiente, Direccion};
use crate::resources::config;

/// Crea la cámara para el juego
pub fn crear_camara(mut commands: Commands) {
    commands.spawn(Camera2d);
}

/// Crea la entidad de la serpiente
pub fn crear_serpiente(mut commands: Commands) {
    // Crear cabeza de la serpiente
    commands.spawn((
        CabezaSerpiente,
        Posicion { x: 12, y: 12 },
        Direccion::Derecha,
        Sprite {
            color: Color::srgb(0.0, 1.0, 0.0),
            custom_size: Some(Vec2::new(config::TAMANO_CELDA, config::TAMANO_CELDA)),
            ..default()
        },
        Transform::from_translation(calcular_posicion_mundo(12, 12, 2.0)),
    ));

    // Crear segmentos iniciales del cuerpo
    for i in 1..=2 {
        commands.spawn((
            Posicion { x: 12 - i, y: 12 },
            CuerpoSerpiente { indice: i as usize },
            Sprite {
                color: Color::srgb(0.0, 0.8, 0.0),
                custom_size: Some(Vec2::new(config::TAMANO_CELDA, config::TAMANO_CELDA)),
                ..default()
            },
            Transform::from_translation(calcular_posicion_mundo(12 - i, 12, 1.0)),
        ));
    }
}

/// Función auxiliar para calcular posición en el mundo
fn calcular_posicion_mundo(x: i32, y: i32, z: f32) -> Vec3 {
    Vec3::new(
        x as f32 * config::TAMANO_CELDA - (config::ANCHO_MAPA as f32 * config::TAMANO_CELDA) / 2.0 + config::TAMANO_CELDA / 2.0,
        y as f32 * config::TAMANO_CELDA - (config::ALTO_MAPA as f32 * config::TAMANO_CELDA) / 2.0 + config::TAMANO_CELDA / 2.0,
        z,
    )
}

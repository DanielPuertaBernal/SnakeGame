use bevy::prelude::*;
use rand::Rng;
use crate::component::position::Posicion;
use crate::component::food::Comida;
use crate::component::snake::{CabezaSerpiente, CuerpoSerpiente};
use crate::component::game::Borde;
use crate::resources::config;

/// Sistema que genera comida aleatoriamente
pub fn sistema_generar_comida(
    mut commands: Commands,
    query_comida: Query<Entity, With<Comida>>,
    query_ocupado: Query<&Posicion, Or<(With<CabezaSerpiente>, With<CuerpoSerpiente>, With<Borde>)>>,
) {
    if query_comida.is_empty() {
        let mut rng = rand::thread_rng();
        let mut posicion_valida = false;
        let mut nueva_posicion = Posicion { x: 0, y: 0 };

        // Buscar posición válida
        while !posicion_valida {
            nueva_posicion.x = rng.gen_range(1..config::ANCHO_MAPA - 1);
            nueva_posicion.y = rng.gen_range(1..config::ALTO_MAPA - 1);

            // Verificar que no esté ocupada
            posicion_valida = !query_ocupado.iter().any(|pos| 
                pos.x == nueva_posicion.x && pos.y == nueva_posicion.y
            );
        }

        // Crear la comida
        commands.spawn((
            nueva_posicion,
            Comida,
            Sprite {
                color: Color::srgb(1.0, 0.0, 0.0),
                custom_size: Some(Vec2::new(config::TAMANO_CELDA, config::TAMANO_CELDA)),
                ..default()
            },
            Transform::from_translation(calcular_posicion_mundo(nueva_posicion.x, nueva_posicion.y, 1.0)),
        ));
    }
}

/// Sistema que genera los bordes del mapa
pub fn sistema_generar_bordes(mut commands: Commands) {
    // Bordes horizontales
    for x in 0..config::ANCHO_MAPA {
        crear_borde(&mut commands, x, 0); // Inferior
        crear_borde(&mut commands, x, config::ALTO_MAPA - 1); // Superior
    }

    // Bordes verticales (excluyendo esquinas)
    for y in 1..config::ALTO_MAPA - 1 {
        crear_borde(&mut commands, 0, y); // Izquierdo
        crear_borde(&mut commands, config::ANCHO_MAPA - 1, y); // Derecho
    }
}

/// Función auxiliar para crear un bloque de borde
fn crear_borde(commands: &mut Commands, x: i32, y: i32) {
    commands.spawn((
        Posicion { x, y },
        Borde,
        Sprite {
            color: Color::srgb(0.5, 0.5, 0.5),
            custom_size: Some(Vec2::new(config::TAMANO_CELDA, config::TAMANO_CELDA)),
            ..default()
        },
        Transform::from_translation(calcular_posicion_mundo(x, y, 0.5)),
    ));
}

/// Función auxiliar para calcular posición en el mundo
fn calcular_posicion_mundo(x: i32, y: i32, z: f32) -> Vec3 {
    Vec3::new(
        x as f32 * config::TAMANO_CELDA - (config::ANCHO_MAPA as f32 * config::TAMANO_CELDA) / 2.0 + config::TAMANO_CELDA / 2.0,
        y as f32 * config::TAMANO_CELDA - (config::ALTO_MAPA as f32 * config::TAMANO_CELDA) / 2.0 + config::TAMANO_CELDA / 2.0,
        z,
    )
}
use bevy::prelude::*;
use crate::component::position::Posicion;
use crate::resources::config;

pub fn sistema_renderizar(
    mut query: Query<(&Posicion, &mut Transform), Changed<Posicion>>
) {
    let offset_x = -(config::ANCHO_MAPA as f32 * config::TAMANO_CELDA) / 2.0 + config::TAMANO_CELDA / 2.0;
    let offset_y = -(config::ALTO_MAPA as f32 * config::TAMANO_CELDA) / 2.0 + config::TAMANO_CELDA / 2.0;

    for (posicion, mut transform) in query.iter_mut() {
        transform.translation = Vec3::new(
            posicion.x as f32 * config::TAMANO_CELDA + offset_x,
            posicion.y as f32 * config::TAMANO_CELDA + offset_y,
            0.0,
        );
    }
}
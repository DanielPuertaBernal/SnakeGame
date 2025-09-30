use bevy::prelude::*;
use crate::component::position::Posicion;
use crate::component::snake::{CabezaSerpiente, CuerpoSerpiente};
use crate::component::food::Comida;
use crate::resources::config;

/// Sistema que actualiza la posición visual de todos los elementos móviles
pub fn sistema_renderizar(
    mut query_serpiente: Query<(&Posicion, &mut Transform), (Or<(With<CabezaSerpiente>, With<CuerpoSerpiente>)>, Changed<Posicion>)>,
    mut query_comida: Query<(&Posicion, &mut Transform), (With<Comida>, Changed<Posicion>, Without<CabezaSerpiente>, Without<CuerpoSerpiente>)>,
) {
    // Renderizar serpiente (solo cuando la posición cambia)
    for (posicion, mut transform) in query_serpiente.iter_mut() {
        actualizar_transform(&mut transform, posicion);
    }

    // Renderizar comida (solo cuando la posición cambia)
    for (posicion, mut transform) in query_comida.iter_mut() {
        actualizar_transform(&mut transform, posicion);
    }
}

/// Función auxiliar para actualizar la transformación
fn actualizar_transform(transform: &mut Transform, posicion: &Posicion) {
    transform.translation.x = posicion.x as f32 * config::TAMANO_CELDA 
        - (config::ANCHO_MAPA as f32 * config::TAMANO_CELDA) / 2.0 
        + config::TAMANO_CELDA / 2.0;
    transform.translation.y = posicion.y as f32 * config::TAMANO_CELDA 
        - (config::ALTO_MAPA as f32 * config::TAMANO_CELDA) / 2.0 
        + config::TAMANO_CELDA / 2.0;
}
use bevy::prelude::*;
use crate::component::position::Posicion;

/// Dimensiones del mapa (en celdas)
const ANCHO_MAPA: i32 = 25;
const ALTO_MAPA: i32 = 25;
const TAMANO_CELDA: f32 = 20.0;

/// Sistema que traduce la posición lógica en el mapa
/// a la posición gráfica en la ventana
pub fn sistema_renderizar(
    mut query: Query<(&Posicion, &mut Transform)>
) {
    for (posicion, mut transform) in query.iter_mut() {
        // Centrar el mapa en la pantalla
        let offset_x = -(ANCHO_MAPA as f32 * TAMANO_CELDA) / 2.0 + TAMANO_CELDA / 2.0;
        let offset_y = -(ALTO_MAPA as f32 * TAMANO_CELDA) / 2.0 + TAMANO_CELDA / 2.0;
        
        transform.translation = Vec3::new(
            posicion.x as f32 * TAMANO_CELDA + offset_x,
            posicion.y as f32 * TAMANO_CELDA + offset_y,
            0.0,
        );
    }
}
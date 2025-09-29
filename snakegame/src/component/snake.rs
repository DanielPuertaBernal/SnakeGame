use bevy::prelude::*;

/// Marca la cabeza de la serpiente
#[derive(Component)]
pub struct CabezaSerpiente;

/// Dirección actual de movimiento
#[derive(Component, Clone, Copy)]
pub enum Direccion {
    Arriba,
    Abajo,
    Izquierda,
    Derecha,
}

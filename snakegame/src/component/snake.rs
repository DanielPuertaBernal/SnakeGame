use bevy::prelude::*;

/// Marca la cabeza de la serpiente
#[derive(Component)]
pub struct CabezaSerpiente;

/// Marca los segmentos del cuerpo de la serpiente
#[derive(Component)]
pub struct CuerpoSerpiente {
    pub indice: usize, // Índice del segmento (0 = más cerca de la cabeza)
}

/// Dirección actual de movimiento
#[derive(Component, Clone, Copy, PartialEq)]
pub enum Direccion {
    Arriba,
    Abajo,
    Izquierda,
    Derecha,
}

use bevy::prelude::*;

/// Configuración del mapa del juego
pub mod config {
    /// Dimensiones del mapa (en celdas)
    pub const ANCHO_MAPA: i32 = 25;
    pub const ALTO_MAPA: i32 = 25;
    
    /// Velocidad base del juego (segundos entre movimientos)
    pub const VELOCIDAD_BASE: f32 = 0.3;
}

/// Recurso para controlar el tiempo entre movimientos
#[derive(Resource)]
pub struct TemporizadorMovimiento {
    pub timer: Timer,
}

impl Default for TemporizadorMovimiento {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(config::VELOCIDAD_BASE, TimerMode::Repeating),
        }
    }
}


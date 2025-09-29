use bevy::prelude::*;
use crate::component::position::Posicion;
use crate::component::snake::{CabezaSerpiente, Direccion};

/// Dimensiones del mapa (en celdas)
const ANCHO_MAPA: i32 = 25;
const ALTO_MAPA: i32 = 25;

/// Recurso para controlar el tiempo entre movimientos
#[derive(Resource)]
pub struct TemporizadorMovimiento {
    pub timer: Timer,
}

impl Default for TemporizadorMovimiento {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.3, TimerMode::Repeating), // Movimiento cada 0.3 segundos
        }
    }
}

/// Sistema que mueve la serpiente según su dirección
pub fn sistema_movimiento(
    time: Res<Time>,
    mut temporizador: ResMut<TemporizadorMovimiento>,
    mut query: Query<(&mut Posicion, &Direccion), With<CabezaSerpiente>>,
) {
    // Actualizar el temporizador
    temporizador.timer.tick(time.delta());
    
    // Solo mover si el temporizador ha terminado
    if temporizador.timer.just_finished() {
        for (mut posicion, direccion) in query.iter_mut() {
            match direccion {
                Direccion::Arriba => posicion.y += 1,
                Direccion::Abajo => posicion.y -= 1,
                Direccion::Izquierda => posicion.x -= 1,
                Direccion::Derecha => posicion.x += 1,
            }
        }
    }
}

/// Sistema que detecta colisiones con los bordes del mapa
pub fn sistema_colisiones(
    query: Query<&Posicion, With<CabezaSerpiente>>,
    mut salir: EventWriter<AppExit>,
) {
    for posicion in query.iter() {
        if posicion.x < 0 || posicion.x >= ANCHO_MAPA || posicion.y < 0 || posicion.y >= ALTO_MAPA {
            println!("¡La serpiente chocó contra un muro!");
             salir.write(AppExit::Success);
        }
    }
}
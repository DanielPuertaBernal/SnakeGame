use bevy::prelude::*;
use crate::component::position::Posicion;
use crate::component::snake::{CabezaSerpiente, Direccion};
use crate::resources::{TemporizadorMovimiento, config};

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
        if posicion.x < 0 || posicion.x >= config::ANCHO_MAPA || posicion.y < 0 || posicion.y >= config::ALTO_MAPA {
            println!("¡La serpiente chocó contra un muro!");
             salir.write(AppExit::Success);
        }
    }
}
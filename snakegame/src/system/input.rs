use bevy::prelude::*;
use crate::component::snake::{CabezaSerpiente, Direccion};

/// Cambia la dirección de la serpiente según la tecla presionada
pub fn sistema_entrada(
    teclado: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Direccion, With<CabezaSerpiente>>,
) {
    for mut direccion in query.iter_mut() {
        if teclado.just_pressed(KeyCode::KeyW) || teclado.just_pressed(KeyCode::ArrowUp) {
            *direccion = Direccion::Arriba;
        }
        if teclado.just_pressed(KeyCode::KeyS) || teclado.just_pressed(KeyCode::ArrowDown) {
            *direccion = Direccion::Abajo;
        }
        if teclado.just_pressed(KeyCode::KeyA) || teclado.just_pressed(KeyCode::ArrowLeft) {
            *direccion = Direccion::Izquierda;
        }
        if teclado.just_pressed(KeyCode::KeyD) || teclado.just_pressed(KeyCode::ArrowRight) {
            *direccion = Direccion::Derecha;
        }
    }
}

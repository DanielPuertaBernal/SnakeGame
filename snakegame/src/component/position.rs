use bevy::prelude::*;

/// Posición en la cuadrícula del mapa
#[derive(Component, Clone, Copy)]
pub struct Posicion {
    pub x: i32,
    pub y: i32,
}
use bevy::prelude::*;
use crate::component::position::Posicion;
use crate::component::snake::{CabezaSerpiente, Direccion};

/// Crea la cámara para el juego
pub fn crear_camara(mut commands: Commands) {
    commands.spawn(Camera2d);
}

/// Crea la entidad de la serpiente
pub fn crear_serpiente(mut commands: Commands) {
    println!("Creando serpiente en posición inicial: x=12, y=12");
    commands.spawn((
        CabezaSerpiente,
        Posicion { x: 12, y: 12 }, // Más centrada en el mapa 25x25
        Direccion::Derecha,
        Sprite {
            color: Color::srgb(0.0, 1.0, 0.0), // Verde
            custom_size: Some(Vec2::splat(20.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        GlobalTransform::default(),
        Visibility::default(),
        InheritedVisibility::default(),
        ViewVisibility::default(),
    ));
}

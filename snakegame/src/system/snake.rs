use bevy::prelude::*;
use crate::component::position::Posicion;
use crate::component::snake::{CabezaSerpiente, CuerpoSerpiente, Direccion};
use crate::component::food::Comida;
use crate::component::game::Borde;
use crate::resources::{TemporizadorMovimiento, config, EstadoJuego, Score};

/// Recurso unificado para manejar la serpiente completa
#[derive(Resource)]
pub struct Serpiente {
    pub segmentos: Vec<Posicion>, // Primer elemento es la cabeza
    pub direccion: Direccion,
}

impl Default for Serpiente {
    fn default() -> Self {
        Self {
            segmentos: vec![
                Posicion { x: 12, y: 12 }, // Cabeza
                Posicion { x: 11, y: 12 }, // Cuerpo 1
                Posicion { x: 10, y: 12 }, // Cuerpo 2
            ],
            direccion: Direccion::Derecha,
        }
    }
}

/// Sistema unificado de movimiento
pub fn sistema_movimiento_unificado(
    time: Res<Time>,
    mut temporizador: ResMut<TemporizadorMovimiento>,
    mut serpiente: ResMut<Serpiente>,
    mut query_cabeza: Query<&mut Posicion, With<CabezaSerpiente>>,
    mut query_cuerpo: Query<(&mut Posicion, &CuerpoSerpiente), (With<CuerpoSerpiente>, Without<CabezaSerpiente>)>,
    estado: Res<State<EstadoJuego>>,
) {
    if *estado.get() != EstadoJuego::Jugando {
        return;
    }

    temporizador.timer.tick(time.delta());
    
    if temporizador.timer.just_finished() {
        // Calcular nueva posición de la cabeza
        let mut nueva_cabeza = serpiente.segmentos[0];
        match serpiente.direccion {
            Direccion::Arriba => nueva_cabeza.y += 1,
            Direccion::Abajo => nueva_cabeza.y -= 1,
            Direccion::Izquierda => nueva_cabeza.x -= 1,
            Direccion::Derecha => nueva_cabeza.x += 1,
        }

        // Mover todos los segmentos (el cuerpo sigue a la cabeza)
        for i in (1..serpiente.segmentos.len()).rev() {
            serpiente.segmentos[i] = serpiente.segmentos[i - 1];
        }
        serpiente.segmentos[0] = nueva_cabeza;

        // Actualizar posiciones en ECS
        if let Ok(mut pos_cabeza) = query_cabeza.single_mut() {
            *pos_cabeza = serpiente.segmentos[0];
        }

        let mut segmentos_cuerpo: Vec<_> = query_cuerpo.iter_mut().collect();
        segmentos_cuerpo.sort_by_key(|(_, cuerpo)| cuerpo.indice);

        for (i, (mut posicion, _)) in segmentos_cuerpo.into_iter().enumerate() {
            if let Some(nueva_pos) = serpiente.segmentos.get(i + 1) {
                *posicion = *nueva_pos;
            }
        }
    }
}

/// Sistema unificado de colisiones
pub fn sistema_colisiones_unificado(
    mut commands: Commands,
    mut serpiente: ResMut<Serpiente>,
    query_comida: Query<(Entity, &Posicion), With<Comida>>,
    query_borde: Query<&Posicion, With<Borde>>,
    mut score: ResMut<Score>,
    mut estado_juego: ResMut<NextState<EstadoJuego>>,
) {
    let cabeza = &serpiente.segmentos[0];

    // Colisión con bordes
    for pos_borde in query_borde.iter() {
        if cabeza.x == pos_borde.x && cabeza.y == pos_borde.y {
            estado_juego.set(EstadoJuego::GameOver);
            return;
        }
    }

    // Colisión con propio cuerpo
    for segmento in serpiente.segmentos.iter().skip(1) {
        if cabeza.x == segmento.x && cabeza.y == segmento.y {
            estado_juego.set(EstadoJuego::GameOver);
            return;
        }
    }

    // Colisión con comida
    for (entidad_comida, pos_comida) in query_comida.iter() {
        if cabeza.x == pos_comida.x && cabeza.y == pos_comida.y {
            commands.entity(entidad_comida).despawn();
            score.valor += 10;
            
            // Agregar nuevo segmento al final de la serpiente
            let ultimo_segmento = *serpiente.segmentos.last().unwrap();
            serpiente.segmentos.push(ultimo_segmento);
            
            // Crear la entidad visual para el nuevo segmento
            commands.spawn((
                ultimo_segmento,
                CuerpoSerpiente { indice: serpiente.segmentos.len() - 1 },
                Sprite {
                    color: Color::srgb(0.0, 0.8, 0.0),
                    custom_size: Some(Vec2::new(config::TAMANO_CELDA, config::TAMANO_CELDA)),
                    ..default()
                },
                Transform::from_translation(calcular_posicion(ultimo_segmento.x, ultimo_segmento.y, 1.0)),
            ));
            
            return;
        }
    }
}

/// Sistema de entrada unificado
pub fn sistema_entrada_unificado(
    teclado: Res<ButtonInput<KeyCode>>,
    mut serpiente: ResMut<Serpiente>,
    estado: Res<State<EstadoJuego>>,
) {
    // Solo procesar entrada de movimiento cuando estamos jugando
    if *estado.get() != EstadoJuego::Jugando {
        return;
    }

    let direccion_actual = serpiente.direccion;
    
    if teclado.just_pressed(KeyCode::ArrowUp) || teclado.just_pressed(KeyCode::KeyW) {
        if direccion_actual != Direccion::Abajo {
            serpiente.direccion = Direccion::Arriba;
        }
    } else if teclado.just_pressed(KeyCode::ArrowDown) || teclado.just_pressed(KeyCode::KeyS) {
        if direccion_actual != Direccion::Arriba {
            serpiente.direccion = Direccion::Abajo;
        }
    } else if teclado.just_pressed(KeyCode::ArrowLeft) || teclado.just_pressed(KeyCode::KeyA) {
        if direccion_actual != Direccion::Derecha {
            serpiente.direccion = Direccion::Izquierda;
        }
    } else if teclado.just_pressed(KeyCode::ArrowRight) || teclado.just_pressed(KeyCode::KeyD) {
        if direccion_actual != Direccion::Izquierda {
            serpiente.direccion = Direccion::Derecha;
        }
    }
}

/// Sistema separado para manejar el reinicio
pub fn sistema_reinicio_entrada(
    teclado: Res<ButtonInput<KeyCode>>,
    estado: Res<State<EstadoJuego>>,
    mut estado_juego: ResMut<NextState<EstadoJuego>>,
) {
    // Solo procesar cuando estamos en Game Over y se presiona Espacio
    if *estado.get() == EstadoJuego::GameOver {
        if teclado.just_pressed(KeyCode::Space) {
            estado_juego.set(EstadoJuego::Reiniciando);
        }
    }
}

/// Sistema de reinicio unificado
pub fn sistema_reiniciar_unificado(
    mut commands: Commands,
    query_serpiente: Query<Entity, Or<(With<CabezaSerpiente>, With<CuerpoSerpiente>)>>,
    query_comida: Query<Entity, With<Comida>>,
    mut score: ResMut<Score>,
    mut serpiente: ResMut<Serpiente>,
    mut estado_cambio: EventReader<StateTransitionEvent<EstadoJuego>>,
    mut estado_juego: ResMut<NextState<EstadoJuego>>,
) {
    for evento in estado_cambio.read() {
        // Cuando entra al estado de reinicio, hacer el reset y pasar a jugando
        if matches!(evento.exited, Some(EstadoJuego::GameOver)) && evento.entered == Some(EstadoJuego::Reiniciando) {
            // Eliminar entidades existentes
            for entidad in query_serpiente.iter().chain(query_comida.iter()) {
                commands.entity(entidad).despawn();
            }

            // Resetear recursos
            score.valor = 0;
            *serpiente = Serpiente::default();

            // Crear nueva serpiente
            crear_serpiente_unificada(&mut commands);
            
            // Inmediatamente cambiar a jugando
            estado_juego.set(EstadoJuego::Jugando);
        }
    }
}

/// Función auxiliar para crear serpiente
fn crear_serpiente_unificada(commands: &mut Commands) {
    // Cabeza
    commands.spawn((
        Posicion { x: 12, y: 12 },
        CabezaSerpiente,
        Sprite {
            color: Color::srgb(0.0, 1.0, 0.0),
            custom_size: Some(Vec2::new(config::TAMANO_CELDA, config::TAMANO_CELDA)),
            ..default()
        },
        Transform::from_translation(calcular_posicion(12, 12, 2.0)),
    ));

    // Cuerpo
    for i in 1..=2 {
        commands.spawn((
            Posicion { x: 12 - i, y: 12 },
            CuerpoSerpiente { indice: i as usize },
            Sprite {
                color: Color::srgb(0.0, 0.8, 0.0),
                custom_size: Some(Vec2::new(config::TAMANO_CELDA, config::TAMANO_CELDA)),
                ..default()
            },
            Transform::from_translation(calcular_posicion(12 - i, 12, 1.0)),
        ));
    }
}

/// Función auxiliar para calcular posición
fn calcular_posicion(x: i32, y: i32, z: f32) -> Vec3 {
    Vec3::new(
        x as f32 * config::TAMANO_CELDA - (config::ANCHO_MAPA as f32 * config::TAMANO_CELDA) / 2.0 + config::TAMANO_CELDA / 2.0,
        y as f32 * config::TAMANO_CELDA - (config::ALTO_MAPA as f32 * config::TAMANO_CELDA) / 2.0 + config::TAMANO_CELDA / 2.0,
        z,
    )
}
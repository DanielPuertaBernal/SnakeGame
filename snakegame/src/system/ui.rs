use bevy::prelude::*;
use crate::resources::{EstadoJuego, Score};

#[derive(Component)]
pub struct GameOverUI;

#[derive(Component)]
pub struct ScoreTexto;

/// Sistema que maneja la interfaz de usuario
pub fn sistema_ui_setup(mut commands: Commands) {
    // Crear la UI de Game Over (inicialmente oculta)
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
            Visibility::Hidden,
            GameOverUI,
        ))
        .with_children(|parent| {
            // Título "GAME OVER"
            parent.spawn((
                Text::new("GAME OVER"),
                TextFont {
                    font_size: 60.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 0.0, 0.0)),
                Node {
                    margin: UiRect::bottom(Val::Px(20.0)),
                    ..default()
                },
            ));

            // Score
            parent.spawn((
                Text::new("Score: 0"),
                TextFont {
                    font_size: 30.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 1.0, 1.0)),
                Node {
                    margin: UiRect::bottom(Val::Px(20.0)),
                    ..default()
                },
                ScoreTexto,
            ));

            // Instrucciones para reiniciar
            parent.spawn((
                Text::new("Presiona ESPACIO para reiniciar"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.8, 0.8)),
            ));
        });

    // Crear UI de puntuación durante el juego
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(50.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Score: 0"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 1.0, 1.0)),
                ScoreTexto,
            ));
        });
}

/// Sistema que actualiza la UI según el estado del juego
pub fn sistema_ui_actualizar(
    mut query_game_over: Query<&mut Visibility, (With<GameOverUI>, Without<ScoreTexto>)>,
    mut query_score: Query<&mut Text, With<ScoreTexto>>,
    estado: Res<State<EstadoJuego>>,
    score: Res<Score>,
) {
    // Actualizar visibilidad de la pantalla de Game Over
    for mut visibility in query_game_over.iter_mut() {
        *visibility = if matches!(*estado.get(), EstadoJuego::GameOver | EstadoJuego::Reiniciando) {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }

    // Actualizar texto de score
    for mut texto in query_score.iter_mut() {
        texto.0 = format!("Score: {}", score.valor);
    }
}
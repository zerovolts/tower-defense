#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    LoadingMap,
    Playing,
    Paused,
    GameOver,
}

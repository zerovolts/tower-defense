use bevy::prelude::*;

pub struct CurrencyPlugin;

impl Plugin for CurrencyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Currency { coins: 10 });
    }
}

#[derive(Deref)]
pub struct Currency {
    pub coins: i32,
}

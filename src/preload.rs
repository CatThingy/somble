use bevy::prelude::*;

#[derive(Deref, DerefMut)]
pub struct PreloadedAssets(Vec<HandleUntyped>);

pub struct Plugin;

impl Plugin {
    fn preload(mut cmd: Commands, assets: Res<AssetServer>) {
        const FILES: [&'static str; 32] = [
            "blinded.png",
            "bottle.png",
            "delayed_explosion.png",
            "earth_earth.png",
            "earth_elemental_attack.png",
            "fire_earth.png",
            "fire_elemental_attack.png",
            "fire_fire.png",
            "fire_lightning.png",
            "fire_water.png",
            "fire_wind.png",
            "lightning_earth.png",
            "lightning_elemental.png",
            "lightning_elemental_attack.png",
            "lightning_essence.png",
            "lightning_lightning.png",
            "on_fire.png",
            "shocked.png",
            "slowed.png",
            "water_earth.png",
            "water_elemental.png",
            "water_elemental_attack.png",
            "water_essence.png",
            "water_lightning.png",
            "water_water.png",
            "water_wind.png",
            "wind_earth.png",
            "wind_elemental.png",
            "wind_elemental_attack.png",
            "wind_essence.png",
            "wind_lightning.png",
            "wind_wind.png",
        ];

        let mut preloaded = PreloadedAssets(Vec::new());

        for file in FILES {
            preloaded.push(assets.load_untyped(file));
        }

        cmd.insert_resource(preloaded);
    }
}

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::preload);
    }
}

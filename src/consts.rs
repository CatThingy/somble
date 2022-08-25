pub const PLAYER_COLLISION_GROUP: u32 = 1 << 0;
pub const ENEMY_COLLISION_GROUP: u32 = 1 << 1;
pub const WALL_COLLISION_GROUP: u32 = 1 << 2;
pub const PLAYER_ATTACK_COLLISION_GROUP: u32 = 1 << 3;
pub const ENEMY_ATTACK_COLLISION_GROUP: u32 = 1 << 4;
pub const ESSENCE_COLLISION_GROUP: u32 = 1 << 5;

pub const PLAYER_SPEED: f32 = 200.0;
pub const PLAYER_KICK_RANGE: f32 = 16.0;
pub const PLAYER_KICK_FORCE: f32 = 100.0;
pub const PLAYER_KICK_HITSTUN_SECS: f32 = 1.0;

pub const PLAYER_IDLE_ANIM_OFFSET: usize = 0;
pub const PLAYER_WALK_ANIM_OFFSET: usize = 4;
pub const PLAYER_WALK_ANIM_FRAMES: usize = 2;

pub const CAMERA_PAN_SPEED: f32 = 20.0;
pub const CAMERA_PAN_RANGE: f32 = 16.0;
pub const CAMERA_PAN_SCALE: f32 = 0.0625;

pub const BREW_UI_SIZE: f32 = 128.0;
pub const BREW_UI_DEADZONE: f32 = 16.0;

pub const BREW_UI_ICON_SIZE: f32 = 8.0;
pub const BREW_UI_ICON_DISTANCE: f32 = 32.0;

pub const POTION_THROW_SPEED: f32 = 400.0;
pub const POTION_SPIN_SPEED: f32 = 32.0;

pub const GRID_SIZE: i32 = 16;

pub const ELEMENTAL_IDLE_ANIM_OFFSET: usize = 0;
pub const ELEMENTAL_IDLE_ANIM_FRAMES: usize = 3;

pub const ELEMENTAL_WALK_ANIM_OFFSET: usize = 0;
pub const ELEMENTAL_WALK_ANIM_FRAMES: usize = 3;

pub const ELEMENTAL_ATTACK_ANIM_OFFSET: usize = 3;
pub const ELEMENTAL_ATTACK_ANIM_FRAMES: usize = 7;
pub const ELEMENTAL_ATTACK_EMIT_FRAME: usize = 7;

pub const FIRE_ELEMENTAL_SPEED: f32 = 50.0;
pub const FIRE_ELEMENTAL_HEALTH: f32 = 100.0;
pub const FIRE_ELEMENTAL_AGGRO_RANGE: f32 = 100.0;
pub const FIRE_ELEMENTAL_FORGET_RANGE: f32 = 200.0;
pub const FIRE_ELEMENTAL_ATTACK_RANGE: f32 = 50.0;
pub const FIRE_ELEMENTAL_ATTACK_PERIOD: f32 = 0.7;
pub const FIRE_ELEMENTAL_ANIM_PERIOD: f32 = 0.1;
pub const FIRE_ELEMENTAL_ATTACK_VELOCITY: f32 = 100.0;

pub const WATER_ELEMENTAL_SPEED: f32 = 50.0;
pub const WATER_ELEMENTAL_HEALTH: f32 = 100.0;
pub const WATER_ELEMENTAL_AGGRO_RANGE: f32 = 100.0;
pub const WATER_ELEMENTAL_FORGET_RANGE: f32 = 200.0;
pub const WATER_ELEMENTAL_ATTACK_RANGE: f32 = 50.0;
pub const WATER_ELEMENTAL_ATTACK_PERIOD: f32 = 0.7;
pub const WATER_ELEMENTAL_ANIM_PERIOD: f32 = 0.1;
pub const WATER_ELEMENTAL_ATTACK_VELOCITY: f32 = 100.0;

pub const WIND_ELEMENTAL_SPEED: f32 = 50.0;
pub const WIND_ELEMENTAL_HEALTH: f32 = 100.0;
pub const WIND_ELEMENTAL_AGGRO_RANGE: f32 = 100.0;
pub const WIND_ELEMENTAL_FORGET_RANGE: f32 = 200.0;
pub const WIND_ELEMENTAL_ATTACK_RANGE: f32 = 50.0;
pub const WIND_ELEMENTAL_ATTACK_PERIOD: f32 = 0.7;
pub const WIND_ELEMENTAL_ANIM_PERIOD: f32 = 0.1;
pub const WIND_ELEMENTAL_ATTACK_VELOCITY: f32 = 100.0;

pub const LIGHTNING_ELEMENTAL_SPEED: f32 = 50.0;
pub const LIGHTNING_ELEMENTAL_HEALTH: f32 = 100.0;
pub const LIGHTNING_ELEMENTAL_AGGRO_RANGE: f32 = 100.0;
pub const LIGHTNING_ELEMENTAL_FORGET_RANGE: f32 = 200.0;
pub const LIGHTNING_ELEMENTAL_ATTACK_RANGE: f32 = 50.0;
pub const LIGHTNING_ELEMENTAL_ATTACK_PERIOD: f32 = 0.7;
pub const LIGHTNING_ELEMENTAL_ANIM_PERIOD: f32 = 0.1;
pub const LIGHTNING_ELEMENTAL_ATTACK_VELOCITY: f32 = 100.0;

pub const EARTH_ELEMENTAL_SPEED: f32 = 50.0;
pub const EARTH_ELEMENTAL_HEALTH: f32 = 100.0;
pub const EARTH_ELEMENTAL_AGGRO_RANGE: f32 = 100.0;
pub const EARTH_ELEMENTAL_FORGET_RANGE: f32 = 200.0;
pub const EARTH_ELEMENTAL_ATTACK_RANGE: f32 = 50.0;
pub const EARTH_ELEMENTAL_ATTACK_PERIOD: f32 = 0.7;
pub const EARTH_ELEMENTAL_ANIM_PERIOD: f32 = 0.1;
pub const EARTH_ELEMENTAL_ATTACK_VELOCITY: f32 = 100.0;

pub const FIRE_FIRE_RADIUS: f32 = 32.0;
pub const FIRE_FIRE_IMPULSE: f32 = 25.0;
pub const FIRE_FIRE_DAMAGE: f32 = 25.0;
pub const FIRE_FIRE_HITSTUN: f32 = 0.5;

pub const WATER_WATER_SPEED: f32 = 100.0;
pub const WATER_WATER_DURATION: f32 = 0.6;
pub const WATER_WATER_HALF_WIDTH: f32 = 32.0;
pub const WATER_WATER_HALF_HEIGHT: f32 = 8.0;
pub const WATER_WATER_FORCE: f32 = 15.0;

pub const WIND_WIND_DURATION: f32 = 2.0;
pub const WIND_WIND_RADIUS: f32 = 32.0;
pub const WIND_WIND_FORCE: f32 = 5.0;
pub const WIND_WIND_MAX_FALLOFF: f32 = 0.5;
pub const WIND_WIND_FALLOFF_START: f32 = 5.0;
pub const WIND_WIND_FALLOFF_END: f32 = 32.0;

pub const LIGHTNING_LIGHTNING_RADIUS: f32 = 2.0;
pub const LIGHTNING_LIGHTNING_HITSTUN: f32 = 3.0;
pub const LIGHTNING_LIGHTNING_DAMAGE: f32 = 250.0;

pub const EARTH_EARTH_RADIUS: f32 = 16.0;
pub const EARTH_EARTH_DURATION: f32 = 2.0;

pub const FIRE_WATER_DURATION: f32 = 2.0;
pub const FIRE_WATER_RADIUS: f32 = 32.0;
pub const FIRE_WATER_FORCE: f32 = 5.0;
pub const FIRE_WATER_MAX_FALLOFF: f32 = 0.5;
pub const FIRE_WATER_FALLOFF_START: f32 = 5.0;
pub const FIRE_WATER_FALLOFF_END: f32 = 32.0;

pub const FIRE_WIND_RADIUS: f32 = 48.0;
pub const FIRE_WIND_DURATION: f32 = 0.1;

pub const FIRE_LIGHTNING_RADIUS: f32 = 2.0;
pub const FIRE_LIGHTNING_DURATION: f32 = 0.1;

pub const FIRE_EARTH_RADIUS: f32 = 48.0;
pub const FIRE_EARTH_DAMAGE: f32 = 5.0;
pub const FIRE_EARTH_TICK: f32 = 0.1;
pub const FIRE_EARTH_DURATION: f32 = 1.5;

pub const WATER_WIND_CHASE_SPEED: f32 = 10.0;
pub const WATER_WIND_CHASE_RADIUS: f32 = 128.0;
pub const WATER_WIND_DURATION: f32 = 3.0;
pub const WATER_WIND_RADIUS: f32 = 48.0;

pub const WATER_LIGHTNING_RADIUS: f32 = 48.0;
pub const WATER_LIGHTNING_DURATION: f32 = 0.1;

pub const WATER_EARTH_HALF_WIDTH: f32 = 16.0;
pub const WATER_EARTH_HALF_HEIGHT: f32 = 48.0;
pub const WATER_EARTH_HITSTUN: f32 = 0.5;
pub const WATER_EARTH_DAMAGE: f32 = 5.0;
pub const WATER_EARTH_TICK: f32 = 0.1;
pub const WATER_EARTH_DURATION: f32 = 1.5;

pub const WIND_LIGHTNING_CHASE_SPEED: f32 = 10.0;
pub const WIND_LIGHTNING_CHASE_RADIUS: f32 = 128.0;
pub const WIND_LIGHTNING_DURATION: f32 = 3.0;
pub const WIND_LIGHTNING_RADIUS: f32 = 48.0;

pub const WIND_EARTH_RADIUS: f32 = 48.0;
pub const WIND_EARTH_DURATION: f32 = 0.1;

pub const LIGHTNING_EARTH_COUNT: u32 = 7;
pub const LIGHTNING_EARTH_RADIUS: f32 = 2.0;
pub const LIGHTNING_EARTH_SPEED: f32 = 600.0;
pub const LIGHTNING_EARTH_HITSTUN: f32 = 1.5;
pub const LIGHTNING_EARTH_DAMAGE: f32 = 0.0;
pub const LIGHTNING_EARTH_DURATION: f32 = 0.3;

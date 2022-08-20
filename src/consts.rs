pub static PLAYER_COLLISION_GROUP: u32 = 1 << 0;
pub static ENEMY_COLLISION_GROUP: u32 = 1 << 1;
pub static WALL_COLLISION_GROUP: u32 = 1 << 2;
pub static PLAYER_ATTACK_COLLISION_GROUP: u32 = 1 << 3;
pub static ENEMY_ATTACK_COLLISION_GROUP: u32 = 1 << 4;

pub static PLAYER_SPEED: f32 = 200.0;
pub static PLAYER_KICK_RANGE: f32 = 20.0;

pub static PLAYER_IDLE_ANIM_OFFSET: usize = 0;
pub static PLAYER_WALK_ANIM_OFFSET: usize = 4;
pub static PLAYER_WALK_ANIM_FRAMES: usize = 2;

pub static CAMERA_PAN_SPEED: f32 = 20.0;
pub static CAMERA_PAN_RANGE: f32 = 16.0;
pub static CAMERA_PAN_SCALE: f32 = 0.0625;

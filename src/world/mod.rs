pub mod player;
pub mod tilemap;
pub mod bullet;

use player::Player;
use bullet::Bullet;
use tilemap::TileMap;
use crate::app::controller::Controller;
use crate::vec::WorldCoord;

pub const TILE_SIZE: i32 = 32;
pub const TILE_SIZE_F: f32 = TILE_SIZE as f32;

pub struct World {
	pub players: [Player; 2],
	pub tilemap: TileMap,
    pub bullets: Vec<Bullet>,
}

impl World {
	pub fn new() -> World {
		World {
			players: [Player::new(), Player::new()],
            bullets: Vec::new(),
			tilemap: TileMap::from_filename("./map.png"),
		}
	}

    pub fn update(&mut self, controller: &Controller) {
        for (i, player) in self.players.iter_mut().enumerate() {
            player.update(controller.get_direction(i));

            if controller.is_skill1_used(i) && player.can_use_skill1() {
                self.bullets.push(Bullet::new(player.position, controller.get_direction(i)*WorldCoord::new(3)));
                player.reset_skill1_cooldown();
            }
        }
        for bullet in self.bullets.iter_mut() {
            bullet.update();
        }
        self.bullets.retain(|b| b.time_to_live > 0);
    }
}

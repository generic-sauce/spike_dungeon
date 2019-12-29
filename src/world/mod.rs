use crate::world::tilemap::Tile;
pub use crate::vec::TILESIZE;

pub mod player;
pub mod tilemap;
pub mod bullet;

use player::Player;
use bullet::Bullet;
use tilemap::TileMap;
use crate::vec::{TileVec, WorldVec};
use crate::app::controller::Controller;

pub const PLAYER_RADIUS: i32 = 32;
pub use self::tilemap::TILEMAP_SIZE;

lazy_static! {
	pub static ref WORLD_SIZE: WorldVec = TILEMAP_SIZE.into();
}

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
                self.bullets.push(Bullet::new(player.position, controller.get_direction(i)*3));
                player.reset_skill1_cooldown();
            }
        }
        for bullet in self.bullets.iter_mut() {
            bullet.update();
        }
		self.deglitch_players();
        self.bullets.retain(|b| b.time_to_live > 0);
    }

	fn deglitch_players(&mut self) {
        for player in self.players.iter_mut() {
			let player_tilepos: TileVec = player.position.into();

			for pos in player_tilepos.iter_around(4) { // TODO
				if self.tilemap.get_tile(pos) != Tile::WALL { continue; }
				let closest_tilepoint = player.position.max(pos.into()).min((pos+1).into());
				let dist = player.position - closest_tilepoint;
				let dist_len = dist.length();
				let dist_norm = dist / dist_len;

				if dist_len <= PLAYER_RADIUS {
					player.position = closest_tilepoint + dist_norm * PLAYER_RADIUS;
				}
			}
        }
	}
}

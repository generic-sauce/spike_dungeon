use std::cmp::{max, min};

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
        for (i, player) in self.players.iter_mut().enumerate() {
			let player_tilepos: TileVec = player.position.into();

			for pos in player_tilepos.iter_around(4) { // TODO
				if self.tilemap.get_tile(pos) != Tile::WALL { continue; }
				let TileVec { x, y } = pos;
				let closest_tile_x = min((x+1) * TILESIZE, max(x * TILESIZE, player.position.x));
				let closest_tile_y = min((y+1) * TILESIZE, max(y * TILESIZE, player.position.y));
				let distx = -1 * (closest_tile_x - player.position.x);
				let disty = -1 * (closest_tile_y - player.position.y);
				let dist = (((distx * distx) + (disty * disty)) as f32).sqrt() as i32;
				let distx_norm = distx / dist;
				let disty_norm = disty / dist;

				if dist <= 32 {
					player.position.x = closest_tile_x + distx_norm * 32; // TODO unhardcode
					player.position.y = closest_tile_y + disty_norm * 32;
				}
			}
        }
	}
}

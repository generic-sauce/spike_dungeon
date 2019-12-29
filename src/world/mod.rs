use std::cmp::{max, min};

use crate::world::tilemap::Tile;
pub use crate::vec::TILESIZE;

pub mod player;
pub mod tilemap;
pub mod bullet;

use player::Player;
use bullet::Bullet;
use tilemap::TileMap;
use crate::vec::TileVec;
use crate::vec::TileCoord;
use crate::vec::WorldVec;
use crate::vec::WorldCoord;
use crate::app::controller::Controller;
use crate::world::tilemap::TILEMAP_SIZE;

pub const TILESIZE_F: f32 = TILESIZE as f32;

pub const WORLD_SIZE: TileVec = TileVec::new(TileCoord::new(TILEMAP_SIZE.x.0 as i32 * TILESIZE), TileCoord::new(TILEMAP_SIZE.y.0 as i32 * TILESIZE));

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
		self.deglitch_players();
        self.bullets.retain(|b| b.time_to_live > 0);
    }

	fn deglitch_players(&mut self) {
        for (i, player) in self.players.iter_mut().enumerate() {
			for x in (player.position.x.round_tile_coord().0-4)..(player.position.x.round_tile_coord().0+4) {
				for y in (player.position.y.round_tile_coord().0-4)..(player.position.y.round_tile_coord().0+4) {
					if x < 0 || x >= 32 { continue; } // TODO un-hardcode
					if y < 0 || y >= 32 { continue; }
					if self.tilemap.tiles[x as usize][y as usize] != Tile::WALL { continue; }
					let closest_tile_x = min((x+1) * TILESIZE, max(x * TILESIZE, player.position.x.0));
					let closest_tile_y = min((y+1) * TILESIZE, max(y * TILESIZE, player.position.y.0));
					let distx = -1 * (closest_tile_x - player.position.x.0);
					let disty = -1 * (closest_tile_y - player.position.y.0);
					let dist = (((distx * distx) + (disty * disty)) as f32).sqrt() as i32;
					let distx_norm = distx / dist;
					let disty_norm = disty / dist;

					if dist <= 32 {
						player.position.x.0 = closest_tile_x + distx_norm * 32;
						player.position.y.0 = closest_tile_y + disty_norm * 32;
					}
					
				}
			}
        }
	}
}

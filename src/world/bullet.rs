use crate::vec::WorldVec;

pub struct Bullet {
	pub position: WorldVec,
    pub speed: WorldVec,
    pub radius: u32,
    pub time_to_live: u32,
}

impl Bullet {
    pub fn new(position: WorldVec, speed: WorldVec) -> Bullet {
        Bullet { position, speed, radius: 16u32, time_to_live: 40 }
    }

    pub fn update(&mut self) {
        self.position = self.position + self.speed;
        if self.time_to_live > 0 {
            self.time_to_live -= 1;
        }
    }
}

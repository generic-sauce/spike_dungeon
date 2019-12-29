use crate::vec::WorldVec;

pub struct Player {
	pub position: WorldVec,
    skill1_cooldown: u32,
}

impl Player {
	pub fn new() -> Player {
		Player {
			position: WorldVec::with(0),
            skill1_cooldown: 0
		}
	}

    pub fn update(&mut self, direction: WorldVec) {
        self.position = self.position + direction;
        if self.skill1_cooldown > 0 {
            self.skill1_cooldown -= 1;
        }
    }

    pub fn can_use_skill1(&self) -> bool {
        self.skill1_cooldown == 0
    }

    pub fn reset_skill1_cooldown(&mut self) {
        self.skill1_cooldown = 10;
    }
}

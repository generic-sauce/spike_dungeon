use sfml::window::Key;
use crate::vec::{WorldVec, WorldCoord};

const UP_KEY: Key = Key::W;
const LEFT_KEY: Key = Key::A;
const RIGHT_KEY: Key = Key::D;
const DOWN_KEY: Key = Key::S;

const ACCELERATION: i32 = 4;

pub struct Controller {
    pub direction: WorldVec
}

impl Controller {
    pub fn new() -> Controller {
        Controller { direction: WorldVec::new(WorldCoord::new(0), WorldCoord::new(0)) }
    }

    pub fn update(&mut self) {
        let old_direction = self.direction;

        let mut up: i32 = 0;
        let mut left: i32 = 0;

        if UP_KEY.is_pressed()    { up += ACCELERATION; }
        if DOWN_KEY.is_pressed()  { up -= ACCELERATION; }
 
        if LEFT_KEY.is_pressed()  { left += ACCELERATION; }
        if RIGHT_KEY.is_pressed() { left -= ACCELERATION; }

        self.direction.y = WorldCoord::new(up);
        self.direction.x = WorldCoord::new(left);
    }
}

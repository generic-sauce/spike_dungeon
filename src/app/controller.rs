use sfml::window::Key;
use crate::vec::WorldVec;

const UP_KEY_INDEX: usize = 0;
const LEFT_KEY_INDEX: usize = 1;
const RIGHT_KEY_INDEX: usize = 2;
const DOWN_KEY_INDEX: usize = 3;
const SKILL1_KEY_INDEX: usize = 4;

const ACCELERATION: i32 = 4;

struct PlayerController {
    pub player_direction: WorldVec,
    pub skill1_used: bool,
    pub keys: [Key; 5]
}

impl PlayerController {
    fn player1() -> PlayerController {
        PlayerController {
            player_direction: WorldVec::with(0),
            skill1_used: false,
            keys: [Key::W, Key::A, Key::D, Key::S, Key::Q],
        }
    }

    fn player2() -> PlayerController {
        PlayerController {
            player_direction: WorldVec::with(0),
            skill1_used: false,
            keys: [Key::Up, Key::Left, Key::Right, Key::Down, Key::Dash],
        }
    }
}

pub struct Controller {
    player_controllers: [PlayerController; 2]
}

impl Controller {
    pub fn new() -> Controller {
        Controller { player_controllers: [PlayerController::player1(), PlayerController::player2()] }
    }

    pub fn update(&mut self) {
        for player_controller in self.player_controllers.iter_mut() {
            let mut right: i32 = 0;
            let mut up: i32 = 0;

            if player_controller.keys[LEFT_KEY_INDEX].is_pressed()  { right -= ACCELERATION; }
            if player_controller.keys[RIGHT_KEY_INDEX].is_pressed() { right += ACCELERATION; }

            if player_controller.keys[UP_KEY_INDEX].is_pressed()    { up -= ACCELERATION; } // TODO change minus
            if player_controller.keys[DOWN_KEY_INDEX].is_pressed()  { up += ACCELERATION; }

            player_controller.skill1_used = player_controller.keys[SKILL1_KEY_INDEX].is_pressed();

            player_controller.player_direction.x = right;
            player_controller.player_direction.y = up;
        }
    }

    pub fn get_direction(&self, player_index: usize) -> WorldVec {
        self.player_controllers[player_index].player_direction
    }

    pub fn is_skill1_used(&self, player_index: usize) -> bool {
        return self.player_controllers[player_index].skill1_used;
    }
}

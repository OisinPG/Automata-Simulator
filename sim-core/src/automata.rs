use js_sys::Math;

/**
 * Enum used for automata state applied to all automata.
 * 
 * NOTE: #[derive(PartialEq)] is a Rust attribute macro used
 * to allow your type to be compared using == and !=
 */
#[derive(PartialEq)]
pub enum AutomatonState {
    Idle,
    Moving,
    Colliding,
    Dead,
}

/**
 * Interface for all Automata
 */
pub trait Automaton {
    fn tick(&mut self);
    fn x(&self) -> f32;
    fn y(&self) -> f32;
    fn move_step(&mut self, world_width: f32, world_height: f32);
    fn radius(&self) -> f32;
    fn on_collide(&mut self, nx: f32, ny: f32);
    fn push(&mut self, dx: f32, dy: f32);

    fn inner_state(&self) -> &AutomatonState;

    fn state(&self) -> u32 {
        match self.inner_state() {
            AutomatonState::Idle => 0,
            AutomatonState::Moving => 1,
            AutomatonState::Colliding => 2,
            AutomatonState::Dead => 3,
        }
    }

    fn is_dead(&self) -> bool {
        matches!(self.inner_state(), AutomatonState::Dead)
    }
}

/**
 * Struct for a simple cell.
 * Just kinda bounces around.
 */
pub struct SimpleCell {
    pub inner_state: AutomatonState,
    pub pos_x: f32,
    pub pos_y: f32,
    pub vel_x: f32,
    pub vel_y: f32,
}

/**
 * SimpleCell implementation of Automaton interface.
 */
impl Automaton for SimpleCell {
    fn tick(&mut self) {
        // State logic goes here — for now just stay Idle
    }

    fn push(&mut self, dx: f32, dy: f32) {
        self.pos_x += dx;
        self.pos_y += dy;
    }

    fn on_collide(&mut self, nx: f32, ny: f32) {
        // Reflect velocity off the collision normal (aka bounce)
        // Dot product tells us how much of the velocity is along the normal.
        let dot = self.vel_x * nx + self.vel_y * ny;
        self.vel_x -= 2.0 * dot * nx;
        self.vel_y -= 2.0 * dot * ny;

        self.inner_state = AutomatonState::Colliding;
        //self.inner_state = AutomatonState::Dead;
    }

    fn move_step(&mut self, world_width: f32, world_height: f32) {
        if matches!(self.inner_state, AutomatonState::Dead) { return; }

        self.pos_x += self.vel_x;
        self.pos_y += self.vel_y;

        //Wrap around the world
        if self.pos_x > world_width  { self.pos_x = 0.0; }
        if self.pos_x < 0.0          { self.pos_x = world_width; }
        if self.pos_y > world_height { self.pos_y = 0.0; }
        if self.pos_y < 0.0          { self.pos_y = world_height; }

        if matches!(self.inner_state, AutomatonState::Colliding) {
            self.inner_state = AutomatonState::Moving;
        }
    }

    fn x(&self) -> f32 { self.pos_x }
    fn y(&self) -> f32 { self.pos_y }
    fn radius(&self) -> f32 { 3.0 }
    fn inner_state(&self) -> &AutomatonState { &self.inner_state }
}

/**
 * This enum lists every automata kind
 */
pub enum AutomatonKind {
    Simple,
}

/**
 * Factory for making Automata.
 * 
 * @param kind The kind of automaton we're creating
 * @param x, y Starting position of the automaton
 * 
 * NOTES: Box<dyn Automaton> means
 *      Box = heap-allocated ( needed 'cause trait objects ahve unknown size)
 *      dyn = "dynamic dispatch" the actual type is resolved at runtime
 *      Automaton = The trait (interface) it must satisfy
 */
pub fn create_automaton(kind: AutomatonKind, x: f32, y: f32) -> Box<dyn Automaton> {
    // match case (like a switch case)
    match kind {
        //Simple automaton
        AutomatonKind::Simple => Box::new(SimpleCell {
            inner_state: AutomatonState::Moving,
            pos_x: x,
            pos_y: y,
            vel_x: (Math::random() as f32 - 0.5) * 2.0,
            vel_y: (Math::random() as f32 - 0.5) * 2.0,
        }),
    }
}

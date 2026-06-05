use wasm_bindgen::prelude::*; // Lets us use #[wasm_bindgem] attribute macro
mod automata; // Tells rust the module exists
use automata::{Automaton, AutomatonKind, create_automaton}; // Imports stuff from module

/**
 * Spatial Grid
 * 
 * Divides the world into a grid of buckets.
 * Each of which hold indices of the automata inside it.
 * We only check for collisions between automata in the same/adjacent buckets.
 */
struct SpatialGrid {
    cells: Vec<Vec<usize>>, // List of automata indices
    cols: usize,
    rows: usize,
    cell_size: f32,
}

/**
 * Implementation for the SpatialGrid
 */
impl SpatialGrid {
    /**
     * Making a new SpatialGrid
     */
    fn new(world_width: f32, world_height: f32, cell_size: f32) -> Self {
        let cols = (world_width  / cell_size).ceil() as usize;
        let rows = (world_height / cell_size).ceil() as usize;
        SpatialGrid {
            cells: vec![Vec::new(); cols * rows],
            cols,
            rows,
            cell_size,
        }
    }

    /**
     * Clear all the buckets.
     * Called at the start of each tick.
     */
    fn clear(&mut self) {
        for cell in self.cells.iter_mut() {
            cell.clear();
        }
    }

    /**
     * Finding out which bucket an automata is in.
     * Based on its (x, y) position.
     */
    fn cell_index(&self, x: f32, y: f32) -> usize {
        let col = (x / self.cell_size) as usize;
        let row = (y / self.cell_size) as usize;
        // Clamp so we never go out of bounds at the world edges
        let col = col.min(self.cols - 1);
        let row = row.min(self.rows - 1);
        row * self.cols + col
    }

    /**
     * Insert an automaton index into the correct bucket based on its position (x, y)
     */
    fn insert(&mut self, index: usize, x: f32, y: f32) {
        let idx = self.cell_index(x, y);
        self.cells[idx].push(index);
    }
    
    /**
     * Return the automata indices in the same cell as (x, y)
     * AND in the 8 neighboring cells.
     * This is to catch collisions across cell boundaries.
     */
    fn nearby(&self, x: f32, y: f32) -> Vec<usize> {
        let col = (x / self.cell_size) as usize;
        let row = (y / self.cell_size) as usize;

        let mut result = Vec::new();

        // Check a 3x3 block of cells around the automaton
        let row_min = row.saturating_sub(1);
        let row_max = (row + 1).min(self.rows - 1);
        let col_min = col.saturating_sub(1);
        let col_max = (col + 1).min(self.cols - 1);

        for r in row_min..=row_max {
            for c in col_min..=col_max {
                let idx = r * self.cols + c;
                result.extend_from_slice(&self.cells[idx]);
            }
        }

        result
    }
}

/**
 * World - What holds our simulation
 * 
 * Exposed to javascript.
 * 
 * NOTE: #[wasm_bindgen] marks this struct so wasm-bindgen
 * will generate a corresponding Typescript class that
 * JS/TS can isntantiate and call.
 */
#[wasm_bindgen]
pub struct World {
    // A list of heap-allocated Automata
    automata: Vec<Box<dyn Automaton>>,

    // World dimensions
    width: f32,
    height: f32,

    // The spatial grid for collision detection
    grid: SpatialGrid,
}

/**
 * Implementation of the World.
 * 
 * #[wasm_bindgen] exposes the implementation methods
 * to JS/TS. Only pub methods with compatible types export.
 */
#[wasm_bindgen]
impl World {
    // #[wasm_bindgen(constructor)] makes this a Typescript constructor
    #[wasm_bindgen(constructor)]
    pub fn new(count: usize, width: f32, height: f32) -> World {
        // Cell_size should be at least 2x the largest automaton radius
        // so nearby() only needs to check a 3x3 neighborhood
        let cell_size = 20.0;

        /*
            Loops from 0 to count.
            .map() turns each number into a new automata via the Factory.
            .collect() gathers all the automata into a Vec (list)
         */
        let automata = (0..count)
            .map(|_| create_automaton(
                AutomatonKind::Simple,
                js_sys::Math::random() as f32 * width,
                js_sys::Math::random() as f32 * height,
            ))
            .collect();

        World {
            automata,
            width,
            height,
            grid: SpatialGrid::new(width, height, cell_size),
        }
    }

    // Advance the simulation by one step
    //  1. Rebuild spatial grid
    //  2. Detect collisions
    //  3. Resolve collisions (call on_collide)
    //  4. Tick and move everyone
    pub fn tick(&mut self) {
        //Step 1: Rebuild spatial grid from current position
        self.grid.clear();
        for (i, a) in self.automata.iter().enumerate() {
            self.grid.insert(i, a.x(), a.y());
        }

        //Step 2: Find all colliding pairs
        //Create a list for the collisions
        let mut collisions: Vec<(usize, usize, f32, f32)> = Vec::new();

        for i in 0..self.automata.len() {
            let ax = self.automata[i].x();
            let ay = self.automata[i].y();
            let ar = self.automata[i].radius();

            // Only check automata in nearby grid cells
            for &j in self.grid.nearby(ax, ay).iter() {
                // Skip self-comparison and skip duplicate pairs
                if j <= i { continue; }

                let bx = self.automata[j].x();
                let by = self.automata[j].y();
                let br = self.automata[j].radius();

                // Squared distance between centers (avoids a sqrt for the check)
                let dx = bx - ax;
                let dy = by - ay;
                let dist_sq = dx * dx + dy * dy;
                let min_dist = ar + br;

                if dist_sq < min_dist * min_dist {
                    // They are overlapping - compute the collision normal
                    // The normal is the unit vector pointing from a to b
                    let dist = dist_sq.sqrt();
                    // Avoid division by zero if they're on each other
                    let (nx, ny) = if dist > 0.0 {
                        (dx / dist, dy / dist)
                    } else {
                        (1.0, 0.0)
                    };
                    collisions.push((i, j, nx, ny));
                }
            }
        }

        //Step 3: Resolve collisions
        for &(i, j, nx, ny) in &collisions {
            let ar = self.automata[i].radius();
            let br = self.automata[j].radius();

            // How deep they overlap
            let ax = self.automata[i].x();
            let ay = self.automata[i].y();
            let bx = self.automata[j].x();
            let by = self.automata[j].y();

            let dist = ((bx - ax) * (bx - ax) + (by - ay) * (by - ay)).sqrt();
            let overlap = (ar + br) - dist; // positive = they are sinking into each other

            // Pushes each automata back by half the overap distance along the collision normal, separating them
            let correction = (overlap / 2.0).min(1.5);

            let (left, right) = self.automata.split_at_mut(j);
            
            // Push automata i away (opposite normal)
            left[i].push(-nx * correction, -ny * correction);
            // Push automata j away (along normal)
            right[0].push(nx * correction, ny * correction);
            
            // Then resolve velocities as before
            left[i].on_collide(nx, ny);
            right[0].on_collide(-nx, -ny);
        }

        // Step 4: Tick and move everyone
        for a in self.automata.iter_mut() {
            a.tick();
            a.move_step(self.width, self.height);
        }

        // Remove all dead automata
        self.automata.retain(|a| !a.is_dead());
    }

    /**
     * Returns all automaton states as a list of numbers.
     * wasm-bindgen converts Vec<u32> -> Uint32Array
     */
    pub fn get_states(&self) -> Vec<u32> {
        self.automata.iter().map(|a| a.state() as u32).collect()
    }

    /**
     * Returns all automaton positions as a [x0, y0, x1, y1, ...] list.
     * wasm-bindgen converts Vec<f32> -> Float32Array
     */
    pub fn get_positions(&self) -> Vec<f32> {
        self.automata.iter().flat_map(|a| [a.x(), a.y()]).collect()
    }

    /**
     * Returns how many automata there are
     */
    pub fn count(&self) -> usize {
        self.automata.len()
    }
}
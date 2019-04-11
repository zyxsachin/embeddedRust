#![warn(clippy::all)]

// Cost and then count
pub struct Carbondioxide {
    tree: (u32, u32, u32),
    // special: (u32, u32, u32),
}

impl Carbondioxide {
    
    pub fn new() -> Self {
        // Cost, current number installed, co2 absorbtion 
        Carbondioxide {
            tree: (5, 0, 1),
            // special: (100_000, 0, 100_000),

        }
    }


    pub fn add_tree(&mut self) {
        self.tree.1 += 1;
    }

    // pub fn remove_trees(&mut self, num : u32) {
    //     if num < self.tree.1 {
    //         self.tree.1 -= num;
    //     } 
    //     else {
    //         self.tree.1 = 0;
    //     }
    // }

    pub fn get_tree_total_absorb(&mut self) -> u32 {
        self.tree.1 * self.tree.2
    }

    pub fn get_tree_cost(&mut self) -> u32 {
        self.tree.0
    }

    pub fn get_tree(&mut self) -> (u32, u32, u32) {
        (self.tree.0, self.tree.1, self.tree.2)
    }

    pub fn get_tree_coord(&mut self) -> (u32, u32) {
        (240, 135)
    }

    // pub fn add_special(&mut self) {
    //     self.special.1 += 1;
    // }

    // pub fn get_special_total_absorb(&mut self) -> u32 {
    //     self.special.1 * self.special.2
    // }

    // pub fn get_special_cost(&mut self) -> u32 {
    //     self.special.0
    // }

    // pub fn get_special(&mut self) -> (u32, u32, u32) {
    //     (self.special.0, self.special.1, self.special.2)
    // }

    // pub fn get_special_coord(&mut self) -> (u32, u32) {
    //     (360, 135)
    // }

    pub fn get_co2_absorb(&mut self) -> u32 {
        self.get_tree_total_absorb()
    }



}
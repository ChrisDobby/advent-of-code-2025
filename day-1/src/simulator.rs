// Simulator module for dial rotation

use crate::parser::{Direction, Rotation};

/// A dial with 100 positions (0-99)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dial {
    position: u32,
}

impl Dial {
    /// Create a new dial starting at position 50
    pub fn new() -> Self {
        Dial { position: 50 }
    }

    /// Apply a rotation to the dial and return the new position
    ///
    /// Right rotations add to the position, left rotations subtract.
    /// The dial wraps around at 100 (positions 0-99).
    pub fn rotate(&mut self, rotation: &Rotation) -> u32 {
        match rotation.direction {
            Direction::Right => {
                // Right rotation: add distance with modulo 100
                self.position = (self.position + rotation.distance) % 100;
            }
            Direction::Left => {
                // Left rotation: subtract distance with proper wraparound
                // Using rem_euclid to handle negative values correctly
                self.position = ((self.position as i32 - rotation.distance as i32).rem_euclid(100)) as u32;
            }
        }
        self.position
    }
}

/// Count how many times the dial passes through position 0 during a single rotation
///
/// We count how many times we visit position 0 (not including the starting position).
/// For right rotations from p by d: we visit p+1, p+2, ..., p+d (mod 100)
/// For left rotations from p by d: we visit p-1, p-2, ..., p-d (mod 100)
pub fn count_zeros_through_rotation(start_pos: u32, rotation: &Rotation) -> u32 {
    match rotation.direction {
        Direction::Right => {
            // Count how many times we visit 0 going from start_pos+1 to start_pos+distance
            // We visit 0 when (start_pos + k) mod 100 = 0, i.e., k = 100m - start_pos
            // For k in [1, distance], count how many satisfy this
            if start_pos == 0 {
                // Starting at 0, first visit to 0 is after 100 steps
                rotation.distance / 100
            } else {
                // First visit to 0 is at k = 100 - start_pos
                // Then every 100 steps after that
                if rotation.distance < 100 - start_pos {
                    0 // Don't reach 0
                } else {
                    1 + ((rotation.distance - (100 - start_pos)) / 100)
                }
            }
        }
        Direction::Left => {
            // Count how many times we visit 0 going from start_pos-1 to start_pos-distance
            if start_pos == 0 {
                // Starting at 0, first visit to 0 is after 100 steps (going 0->99->...->0)
                rotation.distance / 100
            } else {
                // First visit to 0 is at k = start_pos
                if rotation.distance < start_pos {
                    0 // Don't reach 0
                } else {
                    1 + ((rotation.distance - start_pos) / 100)
                }
            }
        }
    }
}

/// Count how many times the dial points at position 0 after applying rotations
///
/// The initial position (50) is not counted as a zero crossing.
/// Only positions reached after completing a rotation are counted.
pub fn count_zero_crossings(rotations: &[Rotation]) -> u32 {
    let mut dial = Dial::new();
    let mut count = 0;

    for rotation in rotations {
        let new_position = dial.rotate(rotation);
        if new_position == 0 {
            count += 1;
        }
    }

    count
}

/// Count all times the dial points at position 0 during rotations (method 0x434C49434B)
///
/// This counts every pass through 0 during each rotation, not just when ending at 0.
/// The initial position (50) is not counted.
pub fn count_all_zero_passes(rotations: &[Rotation]) -> u32 {
    let mut dial = Dial::new();
    let mut total_passes = 0;

    for rotation in rotations {
        // Count passes through 0 during this rotation
        let passes = count_zeros_through_rotation(dial.position, rotation);
        total_passes += passes;

        // Apply the rotation to update dial position for next iteration
        dial.rotate(rotation);
    }

    total_passes
}

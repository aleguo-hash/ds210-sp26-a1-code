use crate::player::{Player, PlayerTrait};
use crate::strategies::Strategy;

pub struct Part2 {}

// Terrible strategy: ask if the number is min, otherwise return max.
impl Strategy for Part2 {
    fn guess_the_number(player: &mut Player, mut min: u32, mut max: u32) -> u32 {
        // YOUR SOLUTION GOES HERE.
        loop{
        let middle = (min + max)/2;
        let mut x = player.ask_to_compare(middle);
        if x  == 1{
            min = middle +1;
        }else if x == -1{
            max = middle -1;
        }else if x == 0{
            return middle;
        }
        }
    }
}

// get_mask is used for pre_generating the moves for all of the pieces so that we can just
// lookup the moves instead of generating them every single time

// this is for pieces that can if the moves are different if whiteToMove changed
pub trait MultiSideMovingPiece {
    fn init(&mut self);
    fn get_mask(&self, sq: i32, white_to_move: bool) -> u64;
}

// this is for pieces that are not sliding(moves don't repeat along col-row) and don't change based
// on who's turn it is. So for this get_mask we will not need white_to_move
pub trait NonSlidingPiece {
    fn init(&mut self);
    fn get_mask(&mut self, sq: i32) -> u64;
}

// Sliding pieces will need get_mask and get_full_move.
// Were going to need both functions for magic moves which is basicly a lookup table for sliding
// pieces
pub trait SlidingPiece {
    fn init(&mut self);
    fn get_mask(&self, sq: i32) -> u64;
    // moves also don't changed based on whose side it is
    fn get_full_move(&self, sq: i32, block: u64) -> u64;
}

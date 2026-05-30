pub fn get_tile_by_coordinate(col: i32, row: i32) -> usize {
    return ((row * 8) + col) as usize;
}

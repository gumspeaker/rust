pub fn num_of_burgers(tomato_slices: i32, cheese_slices: i32) -> Vec<i32> {
    let mut vec = Vec::new();
    if tomato_slices > (cheese_slices * 4) || tomato_slices < (cheese_slices * 2) {
        return vec;
    }
}

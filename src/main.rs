struct Map {
    grid: [[u8; 4]; 4],
    number: u8,
}


fn main() {
    let array = [[2; 4];4];
    let map1 = Map {
        grid: array,
        number: 4,
    };

    println!("Array {} Number {}", map1.grid[0][0] , map1.number);
}

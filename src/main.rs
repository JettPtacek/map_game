struct MapToken{
    id :u8,
    class: u8,
}

impl Copy for MapToken {}

impl Clone for MapToken {
    fn clone(&self) -> MapToken {
        *self
    }
}
struct Map {
    id          :u8,
    terrian_grid : [[ u8; 10] ; 10],
    entity_grid  : [[ MapToken; 10 ]; 10],
}

/**
 * 2d map creation
 */


fn main() {
    let no_entity = MapToken{
        id :1,
        class :0
    };
    let entity_grid = [[no_entity; 10]; 10];
    let terrian_grid = [[2; 10];10];
    let map1 = Map {
        id: 0,
        terrian_grid: terrian_grid,
        entity_grid: entity_grid,
    };
    println!("Map ID: {}", map1.id);
    println!("entity grid id [0][0] {}", map1.entity_grid[0][0].id);
    println!("entity grid class [0][0] {}", map1.entity_grid[0][0].class);
    println!("terrian grid number [0][0] {}", map1.terrian_grid[0][0]);
}

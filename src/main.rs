use hlua::Lua;
use rand::Rng;
use std::fs;
use std::path::Path;

use edbs::map::{Map, MapPosition};

fn main() {
    let mut game_map = Map::new(5, 6);
    let start = MapPosition {x: 4, y: 5};
    let to = MapPosition {x: 3, y: 5};
    game_map.set(&start, 1).unwrap();
    game_map.swap(&start, &to).unwrap();
    println!("{}", game_map);

    let path = Path::new("./game/entities/Archer.lua");
    let contents = fs::read_to_string(path).unwrap();

    let mut lua = Lua::new();
    let mut rng = rand::thread_rng();

    lua.set("roll", hlua::function1(move |die: i32| rng.gen_range(1..=die)));

    let result = lua.execute::<i32>(contents.as_str()).unwrap();
    println!("result {}", result);
}

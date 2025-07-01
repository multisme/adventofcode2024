use std::{collections::HashMap, hash::Hasher};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn left(&self) -> Position {
        Position {
            x: self.x - 1,
            y: self.y,
        }
    }

    fn right(&self) -> Position {
        Position {
            x: self.x + 1,
            y: self.y,
        }
    }

    fn up(&self) -> Position {
        Position {
            x: self.x,
            y: self.y + 1,
        }
    }

    fn down(&self) -> Position {
        Position {
            x: self.x,
            y: self.y - 1,
        }
    }
}

fn explore_grid(
    grid: &mut HashMap<Position, (char, bool)>,
    pos: Position,
    plant: char,
    perimeter: &mut u32,
) -> u32 {
    match grid.get_mut(&pos) {
        Some((c_plant, true)) if *c_plant == plant => 0,
        Some(address) => {
            if address.0 == plant {
                //If visited
                *perimeter += 1;
                address.1 = true;
                explore_grid(grid, pos.left(), plant, perimeter)
                    + explore_grid(grid, pos.up(), plant, perimeter)
                    + explore_grid(grid, pos.right(), plant, perimeter)
                    + explore_grid(grid, pos.down(), plant, perimeter)
            } else {
                1
            }
        }
        _ => 1,
    }
}

fn main() {
    let data = include_str!("../test.txt");
    let mut map: HashMap<Position, (char, bool)> = HashMap::new();
    for (y, line) in data.lines().enumerate() {
        for (x, plot) in line.chars().enumerate() {
            let point = Position {
                x: x as i32,
                y: y as i32,
            };
            map.entry(point).or_insert((plot, false));
        }
    }
    let mut grid = map.clone();
    let result = map
        .iter()
        .map(|(pos, (plant, visited))| {
            let mut perimeter = 0;
            if !*visited {
                let corners = explore_grid(&mut grid, pos.left(), *plant, &mut perimeter);
                println!("{:?} {} {}", pos, plant, corners);
                corners
            } else {
                0
            }
        })
        .collect::<Vec<_>>();
    println!("{:?}", result);
}

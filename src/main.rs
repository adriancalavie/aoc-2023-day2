use std::fs;

const MAXIMUM_CUBE_SET: CubeSet = CubeSet {
    red: 12,
    green: 13,
    blue: 14,
};

#[derive(Debug)]
struct Game {
    id: i32,
    cube_sets: Vec<CubeSet>,
}

impl Game {
    fn from_string(str: &str) -> Self {
        let parts = str.trim().split(':').collect::<Vec<&str>>();

        // let id = parts[0].chars().last().unwrap().to_digit(10).unwrap() as i32;
        let id = parts[0].trim().split(' ').collect::<Vec<&str>>()[1]
            .parse::<i32>()
            .unwrap();
        let cubes = parts[1]
            .trim()
            .split(';')
            .collect::<Vec<&str>>()
            .iter()
            .map(|s| CubeSet::create_from_string(s))
            .collect();

        Game {
            id,
            cube_sets: cubes,
        }
    }

    fn get_minimum_required_cube_set(&self) -> CubeSet {
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        for cube_set in &self.cube_sets {
            if cube_set.red > max_red {
                max_red = cube_set.red
            }
            if cube_set.green > max_green {
                max_green = cube_set.green
            }
            if cube_set.blue > max_blue {
                max_blue = cube_set.blue
            }
        }

        CubeSet::from(max_red, max_green, max_blue)
    }
}

#[derive(Debug)]
struct CubeSet {
    red: i64,
    green: i64,
    blue: i64,
}

impl CubeSet {
    fn new() -> CubeSet {
        CubeSet {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    #[allow(dead_code)]
    fn from(red: i64, green: i64, blue: i64) -> Self {
        CubeSet { red, green, blue }
    }

    fn create_from_string(str: &str) -> Self {
        let mut new_cube = CubeSet::new();

        str.trim().split(',').for_each(|str| {
            let parts = str.trim().split(' ').collect::<Vec<&str>>();
            let num = parts[0].parse::<i64>().unwrap();
            let color = parts[1];

            match color {
                "red" => new_cube.red = num,
                "green" => new_cube.green = num,
                "blue" => new_cube.blue = num,
                _ => {
                    println!("Unknown color: {}", color);
                }
            }
        });

        new_cube
    }

    fn is_valid(&self) -> bool {
        self.red <= MAXIMUM_CUBE_SET.red
            && self.green <= MAXIMUM_CUBE_SET.green
            && self.blue <= MAXIMUM_CUBE_SET.blue
    }

    fn power(&self) -> i64 {
        self.red * self.green * self.blue
    }
}

/// The `get_input` function reads the contents of a file at the given path and returns them as a vector of strings, with
/// leading and trailing whitespace removed from each line.
///
/// Arguments:
///
/// * `path`: The `path` parameter in the `get_input` function is a `String` that represents the file path from which you
/// want to read the input.
///
/// Returns:
///
/// The function `get_input` returns a `Vec<String>`, which is a vector of strings.
fn get_input(path: &str) -> Vec<String> {
    let content = fs::read_to_string(path).expect("Couldn't read input");
    let lines = content
        .lines()
        .map(|s| -> String { s.trim().to_string() })
        .collect();

    lines
}

fn main() {
    let path = "res/data.txt";
    // let path = "res/data_light.txt";
    let lines = get_input(path);

    // let mut count = 0;
    let mut sum = 0;

    for line in lines {
        let game = Game::from_string(&line);
        println!("Game {}: {:?}", game.id, game.cube_sets);

        // solution for part 1
        // let mut should_increment: bool = true;
        // for cube_set in game.cube_sets {
        //     if !cube_set.is_valid() {
        //         should_increment = false;
        //         break;
        //     }
        // }
        // if should_increment {
        //     count += game.id
        // }

        // solution for part 2
        let cube_set = game.get_minimum_required_cube_set();
        sum += cube_set.power();
    }

    // println!("Count: {}", count);
    println!("Count: {}", sum);
}

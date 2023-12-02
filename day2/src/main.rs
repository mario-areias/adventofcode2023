const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

fn main() {
    // run from root folder
    let f = lib::open_file("day2/input.txt".to_string()).unwrap();
    let lines = lib::read_lines_from_file(f).unwrap();

    let mut sum_index = 0;
    let mut sum_power = 0;

    for line in lines {
        let mut red: u32 = 0;
        let mut green: u32 = 0;
        let mut blue: u32 = 0;

        let mut possible: bool = true;

        // split games and sets: ["Game 1", "3 blue, 4 red; 1 red, 2 green"]
        let mut iter = line.split(':');
        let game = iter.next().unwrap();
        let sets = iter.next().unwrap();

        let index = get_index(game.to_string());

        // split into sets ["3 blue, 4 red", "1 red, 2 green"]
        for set in sets.split(';') {
            // split into cubes ["3 blue", "4 red"]
            for cubes in set.split(',') {
                // split into number and color ["3", "blue"]
                let mut iter = cubes.trim().split(' ');

                let number = iter.next().unwrap();
                let color = iter.next().unwrap();

                let number = number.parse::<u32>().unwrap();
                match color {
                    "red" => {
                        if number > MAX_RED {
                            possible = false;
                        }

                        if number > red {
                            red = number;
                        }
                    }
                    "green" => {
                        if number > MAX_GREEN {
                            possible = false;
                        }
                        if number > green {
                            green = number;
                        }
                    }
                    "blue" => {
                        if number > MAX_BLUE {
                            possible = false;
                        }

                        if number > blue {
                            blue = number;
                        }
                    }
                    _ => {}
                }
            }
        }

        sum_power += red * green * blue;
        if possible {
            sum_index += index;
        }
    }

    println!("Sum Index: {}", sum_index);
    println!("Sum Power: {}", sum_power);
}

fn get_index(game: String) -> usize {
    let mut iter = game.split(' ');
    // skipping the first element
    iter.next().unwrap();
    let index = iter.next().unwrap();

    index.parse::<usize>().unwrap()
}

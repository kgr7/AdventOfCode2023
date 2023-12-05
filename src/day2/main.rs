use std::fs;

pub async fn part_1() {
    let games = parse_input();

    let limit = Round {
        red: 12,
        green: 13,
        blue: 14,
    };

    let mut total: i32 = 0;

    'outer: for game in &games {
        // let game_id = extract_game_id(game);
        let game_products = extract_rounds(game);
        total += game_products;
        // for round in rounds {
        //     if round.red > limit.red || round.green > limit.green || round.blue > limit.blue {
        //         continue 'outer;
        //     }
        // }
        // total += game_id;
    }

    println!("Game products sum: {}", total);
}
#[derive(Default)]
struct Round {
    red: i32,
    green: i32,
    blue: i32,
}

fn parse_input() -> Vec<String> {
    let loc = "../../inputs/day2.txt";
    let contents = fs::read_to_string(loc).expect("Could not read from file.");
    let mut games = contents.split("\n").map(String::from).collect::<Vec<_>>();
    games.pop();

    games
}

fn extract_rounds(line: &str) -> i32 {
    let all_rounds_str = line.split(": ").collect::<Vec<_>>()[1];
    let rounds = all_rounds_str.split(";").collect::<Vec<_>>();

    let valid_colours = vec!["red", "green", "blue"];

    let mut rnds: Vec<Round> = Vec::new();

    for round in rounds {
        let colours = round.split(",").collect::<Vec<_>>();

        let mut rnd = Round {
            red: 0,
            green: 0,
            blue: 0,
        };

        for colour in colours {
            for v_colour in &valid_colours {
                if colour.contains(v_colour) {
                    let num = colour.replace(v_colour, "").trim().parse::<i32>().unwrap();
                    match *v_colour {
                        "red" => rnd.red = num,
                        "green" => rnd.green = num,
                        "blue" => rnd.blue = num,
                        _ => panic!("Invalid colour found"),
                    }
                }
            }
        }
        rnds.push(rnd);
    }

    let mut max_cubes = Round::default();

    for round in rnds {
        if round.red > max_cubes.red {
            max_cubes.red = round.red;
        }
        if round.green > max_cubes.green {
            max_cubes.green = round.green;
        }
        if round.blue > max_cubes.blue {
            max_cubes.blue = round.blue;
        }
    }

    let power = max_cubes.red * max_cubes.green * max_cubes.blue;

    power
    // rnds
}

fn extract_game_id(line: &str) -> i32 {
    let start_token = "Game ";
    let start_bytes = line.find(&start_token).unwrap() + start_token.len();
    let end_bytes = line.find(":").unwrap();

    line[start_bytes..end_bytes].parse::<i32>().unwrap()
}

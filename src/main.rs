extern crate serde;
extern crate toml;
#[macro_use]
extern crate serde_derive;

mod util;

use util::*;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
struct Player {
    name: String,
    elo: i32,
    draw: usize,
}

enum TournamentStructure {
    SingleElimination,
    DoubleElimination
}

struct TournamentInfo {
    structure: TournamentStructure,
    size: usize
}

fn probability_wins<F>(player_index: usize, players: &[Option<Player>], predictor: &F) -> f64
where
    F: Fn(&Player, &Player) -> f64,
{
    let pred = |p: &Option<Player>, q: &Option<Player>| {
        if let Some(p) = p {
            if let Some(q) = q {
                predictor(p, q)
            } else {
                1f64
            }
        } else {
            0f64
        }
    };
    assert_eq!(players.len() % 2, 0);
    if players.len() == 2 {
        pred(&players[player_index], &players[(player_index + 1) % 2])
    } else {
        let fulcrum = players.len() / 2;
        let top = &players[..fulcrum];
        let bottom = &players[fulcrum..];
        assert_eq!(top.len(), bottom.len());
        if player_index < fulcrum {
            probability_wins(player_index, top, predictor)
                * bottom
                    .iter()
                    .enumerate()
                    .map(|(i, b)| {
                        pred(&players[player_index], b) * probability_wins(i, bottom, predictor)
                    })
                    .sum::<f64>()
        } else {
            probability_wins(player_index - fulcrum, bottom, predictor)
                * top.iter()
                    .enumerate()
                    .map(|(i, t)| {
                        pred(&players[player_index], t) * probability_wins(i, top, predictor)
                    })
                    .sum::<f64>()
        }
    }
}

fn victory_probability(player_elo: i32, opponent_elo: i32) -> f64 {
    use std::f64;
    let diff = f64::from(opponent_elo - player_elo);
    let m: f64 = diff / 400.0;
    1.0 / (1.0 + (10.0 as f64).powf(m))
}



use toml::Value;
use std::collections::BTreeMap;
fn parse_tournament_data(tournament_data: &BTreeMap<String, toml::Value>) -> TournamentInfo {
    let size = match tournament_data.get("size") {
        Some(Value::Integer(n)) =>
            if 1 < *n && is_power_of_2(*n as usize){
                *n
            } else {
                eprintln!("Invalid Tournament size: {}", n);
                std::process::exit(2);
            },
        Some(v) => {
            eprintln!("Invalid value for tournament size: {:?}", v);
            std::process::exit(2);
        }
        _ => 0
    } as usize;
    let structure = if let Some(Value::String(structure)) = tournament_data.get("structure") {
        match structure.as_ref() {
            "Double Elimination" => TournamentStructure::DoubleElimination,
            "Single Elimination" => TournamentStructure::SingleElimination,
            s => {
                eprintln!("Invalid tournament structure: {}", s);
                std::process::exit(2);
            }
        }
    } else {
        TournamentStructure::SingleElimination
    };
    TournamentInfo {
        structure,
        size
    }
}


fn read_data(file_name: &str) -> Vec<Option<Player>> {
    use std::collections::BTreeMap;    
    use std::fs::File;
    use std::io::Read;
    let mut file = File::open(file_name).unwrap();
    let mut contents = String::new();
    let _ = file.read_to_string(&mut contents);
    let mut map: BTreeMap<String, Value> = toml::from_str(&contents).unwrap();
    let players = match  map.remove("Players").unwrap() {
        Value::Table(players) => players,
        _ => {
            eprintln!("No valid players section in tournament file");
            std::process::exit(2);
        }
    };
    let mut input_buffer = Vec::<Player>::new();
    let mut max_draw = 0;
    for (name, data) in &players {
        let draw = data["draw"].as_integer().unwrap() as usize;
        let elo = data["elo"].as_integer().unwrap() as i32;
        if max_draw < draw { max_draw = draw };
        let player = Player {
            name: name.clone(),
            elo,
            draw
        };
        input_buffer.push(player);
    }

    let tournament_info = if let Some(Value::Table(tournament_info)) = map.get("Tournament") {
        parse_tournament_data(&tournament_info)
    } else {
        parse_tournament_data(&BTreeMap::new())
    };
    let tournament_size = find_next_power_of_2(players.len());
    let mut output_buffer: Vec<Option<Player>> = vec![None; tournament_size];

    for player in input_buffer {
        let index = player.draw - 1;
        output_buffer[index] = Some(player);
    }

    output_buffer
}

fn main() {
    let arg = std::env::args().nth(2);
    let file_name = match arg {
        Some(s) => s,
        None => "tournament.toml".into(),
    };
    let players = read_data(&file_name);
    let elo_predictor = |p: &Player, q: &Player| victory_probability(p.elo, q.elo);
    for (i, player) in players.iter().enumerate() {
        let p = probability_wins(i, &players, &elo_predictor);
        if let Some(player) = player {
            println!("player {} wins {}% of the time", player.name, p * 100f64);
        }
    }
}

#[cfg(test)]
mod test {
#[test]
fn test_elo() {
    use ::Player;
    let players = [
        Player {
            name: "Ettu".into(),
            elo: 1210,
            draw: 0,
        },
        Player {
            name: "Ser".into(),
            elo: 1180,
            draw: 1,
        },
        Player {
            name: "Sujoy".into(),
            elo: 1425,
            draw: 2,
        },
        Player {
            name: "Fragga".into(),
            elo: 1550,
            draw: 3,
        },
        Player {
            name: "Cooller".into(),
            elo: 1310,
            draw: 4,
        },
        Player {
            name: "Atomic".into(),
            elo: 3580,
            draw: 5,
        },
        Player {
            name: "Pietro".into(),
            elo: 1325,
            draw: 6,
        },
        Player {
            name: "Coerj".into(),
            elo: 1625,
            draw: 7,
        },
        Player {
            name: "Ettu".into(),
            elo: 2410,
            draw: 8,
        },
        Player {
            name: "Ser".into(),
            elo: 2180,
            draw: 9,
        },
        Player {
            name: "Sujoy".into(),
            elo: 2425,
            draw: 10,
        },
        Player {
            name: "Fragga".into(),
            elo: 2500,
            draw: 11,
        },
        Player {
            name: "Cooller".into(),
            elo: 2410,
            draw: 12,
        },
        Player {
            name: "Atomic".into(),
            elo: 2780,
            draw: 13,
        },
        Player {
            name: "Pietro".into(),
            elo: 2425,
            draw: 14,
        },
        Player {
            name: "Coerj".into(),
            elo: 2500,
            draw: 15,
        },
        Player {
            name: "Ettu".into(),
            elo: 1210,
            draw: 16,
        },
        Player {
            name: "Ser".into(),
            elo: 1180,
            draw: 17,
        },
        Player {
            name: "Sujoy".into(),
            elo: 1425,
            draw: 18,
        },
        Player {
            name: "Fragga".into(),
            elo: 150,
            draw: 19,
        },
        Player {
            name: "Cooller".into(),
            elo: 1310,
            draw: 20,
        },
        Player {
            name: "Atomic".into(),
            elo: 3580,
            draw: 21,
        },
        Player {
            name: "Pietro".into(),
            elo: 1325,
            draw: 22,
        },
        Player {
            name: "Coerj".into(),
            elo: 1625,
            draw: 23,
        },
        Player {
            name: "Ettu".into(),
            elo: 2410,
            draw: 24,
        },
        Player {
            name: "Ser".into(),
            elo: 2180,
            draw: 25,
        },
        Player {
            name: "Sujoy".into(),
            elo: 2425,
            draw: 26,
        },
        Player {
            name: "Fragga".into(),
            elo: 2500,
            draw: 27,
        },
        Player {
            name: "Cooller".into(),
            elo: 2410,
            draw: 28,
        },
        Player {
            name: "Atomic".into(),
            elo: 2780,
            draw: 29,
        },
        Player {
            name: "Pietro".into(),
            elo: 2425,
            draw: 30,
        },
        Player {
            name: "Coerj".into(),
            elo: 2500,
            draw: 31,
        },
    ];
    for p in &players {
        for q in &players {
            let t = p.beats(q) + q.beats(p);
            assert!(0.99999f64 < t && t < 1.00001f64)
        }
    }
}
}
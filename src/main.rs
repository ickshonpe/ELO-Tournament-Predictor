#[macro_use] extern crate maplit;

use std::collections::HashMap;



struct PlayerData {
    odds: HashMap<String, f64>,
    draw_number: u32
}


struct TournamentData {
    players: HashMap<String, PlayerData>
}

impl TournamentData {
    fn new(players: HashMap<String, PlayerData>) -> TournamentData {
        TournamentData {
                players 
        }
    }

    fn victory_probability(&self, player: &str, opponent: &str) -> f64 {
        self.players[player].odds[opponent]
    }

    fn names(&self) -> Vec<String> {
        self.players.keys().cloned().collect()
    }
}
 

fn get_opponent_r2(name: &String, matches: &Vec<(String, String)>) -> (String, String) {      
    for (a, b) in matches {
        if a != name && b != name {
            return (a.clone(), b.clone())
        }
    }       
    ("".to_string(), "".to_string())
}

fn main() {
    let input_data: HashMap<String, PlayerData> = convert_args!(hashmap!(
        "Ettu" => 
            PlayerData {            
                draw_number: 0,
                odds: convert_args!(hashmap!(
                        "Fragga" => 0.25,            
                        "Ser" => 0.8,
                        "Sujoy" => 0.4,                
                    ))
            },
        "Fragga" => 
            PlayerData {
                draw_number: 1,
                odds: convert_args!(hashmap!(
                        "Ettu" => 0.75,
                        "Ser" => 0.95,
                        "Sujoy" => 0.6,                
                    ))
            },            
        "Ser" =>
            PlayerData {
                draw_number: 2,
                odds: convert_args!(hashmap!(
                    "Fragga" => 0.05,            
                    "Ettu" => 0.2,                
                    "Sujoy" => 0.15,                    
                ))
            },
        "Sujoy" =>
            PlayerData {
                draw_number: 3,
                odds: convert_args!(hashmap!(
                    "Fragga" => 0.4,            
                    "Ettu" => 0.55,
                    "Ser" => 0.85,
                ))
            }
    ));
    let player_data = TournamentData { players: input_data };
    

    for name in player_data.names() {                   
        
        // let r1_opponent = get_opponent(&name, &round1_matches).unwrap();
        // let p_r1_win = player_data.victory_probability(&name, &r1_opponent);        
        // println!("{} round 1 opponent: {}", name, r1_opponent);
        // println!("\twinning {}%", p_r1_win * 100f64);
        // let r2 = get_opponent_r2(&name, &round1_matches);
        // println!("{:?}", r2);
    }
}
extern crate rand;
extern crate serde;
extern crate toml;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate maplit;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
struct Player {
    name: String,
    elo: i32,
    draw: i32
}

impl Player {
    fn beats(&self, opponent: &Self) -> f64 {
        victory_probability(self.elo, opponent.elo)
    }
}

fn probability_wins<F>(player: &Player,  players: &[Player], predictor: &F) -> f64 
    where F: Fn(&Player, &Player) -> f64 {
    assert_eq!(players.len() % 2, 0);
    if players.len() == 2 {            
        if player.draw == players[0].draw {            
            predictor(&players[0], &players[1])
        } else {            
            predictor(&players[1], &players[0])            
        }
    } else {                        
        let fulcrum = players.len() / 2;
        let top = &players[..fulcrum];
        let bottom = &players[fulcrum..];
        assert_eq!(top.len(), bottom.len());        
        if player.draw < bottom[0].draw {                                
            probability_wins(player, top, predictor) * bottom.iter().map(|b| predictor(player, b) * probability_wins(b, bottom, predictor)).sum::<f64>()           
        } else {                
            probability_wins(player, bottom, predictor) * top.iter().map(|t| predictor(player, t) * probability_wins(t, top, predictor)).sum::<f64>()                              
        }
    }
}

fn victory_probability(player_elo: i32, opponent_elo: i32) -> f64 {
    use std::f64;
    let diff = f64::from(opponent_elo - player_elo);
    let m: f64 = diff / 400.0;
    1.0/ (1.0 + (10.0 as f64).powf(m))   
}

fn save_test_data_2() {
    use std::fs::File;
    use std::io::Write;
    use std::collections::BTreeMap;
    use toml::Value;

        let data = [
            ("Jack", 1500, 0), 
            ("Jill", 1400, 1), 
            ("James", 1800, 2), 
            ("Jacques", 2400, 3), 
            ("Jacob", 1300, 4), 
            ("Jacqueline", 1500, 5), 
            ("Jesus", 2200, 6), 
            ("Jerome", 2100, 7), 
            ("Jeremy", 2300, 8), 
            ("Jane", 1900, 9), 
            ("Janice", 1890, 10), 
            ("Jasmine", 1300, 11), 
            ("Jasper", 2000, 12), 
            ("Jay", 2500, 13), 
            ("Jordan", 2350, 14), 
            ("John", 2260, 15)
        ].iter().map(|(name, elo, draw)| Player { name: name.to_string().to_string(), elo: *elo, draw: *draw }).collect::<Vec<_>>();
        let mut map: BTreeMap<String, Vec<Player>> = std::collections::BTreeMap::new();
        
        map.insert("Players".into(), data);
        let toml_output = toml::to_string(&map).unwrap();
        let mut file = File::create("players.toml").unwrap();
        file.write_all(toml_output.as_bytes());
}


fn read_data() -> Vec<Player> {
    use std::fs::File;
    use std::io::Read;
    use std::collections::BTreeMap;
    use toml::Value;
    let mut file = File::open("players.toml").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents);
    //println!("{}", contents);
    // let data = toml::from_str(&contents).unwrap();
    // println!("{:?}", data);
    let mut map: BTreeMap<String, Vec<Player>> = toml::from_str(&contents).unwrap();
    map.remove("Players").unwrap()
}

fn main() {
    let players = read_data();
    let elo_predictor =|p: &Player, q: &Player| victory_probability(p.elo, q.elo);
    let another_predictor = |p: &Player, q: &Player| { 
        if p.draw == 5 {
            1.0f64
        } else if q.draw == 5 {
            0.0f64
        } else {
            0.5f64
        }
    };
        let mut m = 2;
    for i in 1..=4 {
        let mut accum = 0f64;
        for player in &players[0..m] {
            let p = probability_wins(&player, &players[0..m], &elo_predictor);    
            
            println!("player {} wins {}% of the time", player.name, p * 100f64);
            accum += p;
        }   
        println!("total ={}", accum); 
        m *= 2;
    }
    
}

#[test] 
fn test_elo() {
    let players = [
        Player { name: "Ettu".into(), elo: 1210, draw: 0}, 
        Player { name: "Ser".into(), elo: 1180, draw: 1} ,        
        Player { name: "Sujoy".into(), elo: 1425, draw: 2}, 
        Player { name: "Fragga".into(), elo: 1550, draw: 3},
        Player { name: "Cooller".into(), elo: 1310, draw: 4}, 
        Player { name: "Atomic".into(), elo: 3580, draw: 5} ,        
        Player { name: "Pietro".into(), elo: 1325, draw: 6}, 
        Player { name: "Coerj".into(), elo: 1625, draw: 7},
        Player { name: "Ettu".into(), elo: 2410, draw: 8}, 
        Player { name: "Ser".into(), elo: 2180, draw: 9} ,        
        Player { name: "Sujoy".into(), elo: 2425, draw: 10}, 
        Player { name: "Fragga".into(), elo: 2500, draw: 11},
        Player { name: "Cooller".into(), elo: 2410, draw: 12}, 
        Player { name: "Atomic".into(), elo: 2780, draw: 13} ,        
        Player { name: "Pietro".into(), elo: 2425, draw: 14}, 
        Player { name: "Coerj".into(), elo: 2500, draw: 15},
        Player { name: "Ettu".into(), elo: 1210, draw: 16}, 
        Player { name: "Ser".into(), elo: 1180, draw: 17} ,        
        Player { name: "Sujoy".into(), elo: 1425, draw: 18}, 
        Player { name: "Fragga".into(), elo: 150, draw: 19},
        Player { name: "Cooller".into(), elo: 1310, draw: 20}, 
        Player { name: "Atomic".into(), elo: 3580, draw: 21} ,        
        Player { name: "Pietro".into(), elo: 1325, draw: 22}, 
        Player { name: "Coerj".into(), elo: 1625, draw: 23},
        Player { name: "Ettu".into(), elo: 2410, draw: 24}, 
        Player { name: "Ser".into(), elo: 2180, draw: 25} ,        
        Player { name: "Sujoy".into(), elo: 2425, draw: 26}, 
        Player { name: "Fragga".into(), elo: 2500, draw: 27},
        Player { name: "Cooller".into(), elo: 2410, draw: 28}, 
        Player { name: "Atomic".into(), elo: 2780, draw: 29} ,        
        Player { name: "Pietro".into(), elo: 2425, draw: 30}, 
        Player { name: "Coerj".into(), elo: 2500, draw: 31},  
    ];        
    for p in &players {
        for q in &players {
            let t = p.beats(q) + q.beats(p);            
            //println!("{} p {} q, t", p, q, t);
            assert!(0.99999f64 < t && t < 1.00001f64 )
            
        }
    }
}
use crate::tnm::Tournament;

mod tnm;
mod tournament;

fn main() {
    let mut t = Tournament::new();

    // load data.tsv (tab separated) where the first 4 columns are the players and the last is the winner
    let data = std::fs::read_to_string("data.tsv").expect("Unable to read data.tsv");
    for line in data.lines() {
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() < 5 {
            continue;
        }

        let players = [parts[0], parts[1], parts[2], parts[3]];
        let winner = parts[4].to_string();

        let game = t.create_game(players);
        let _ = t.submit_game(game, winner);
    }

    dbg!(t);
}

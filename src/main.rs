use crate::tournament::Tournament;

mod tournament;

fn main() {
    let mut t = Tournament::new();
    dbg!(t.create_game(["a","b","c","d"]));
}

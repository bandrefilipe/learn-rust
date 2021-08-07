mod guessing_game;
mod hello_world;

fn main() {
    topic_1_2();
    topic_2();
}

fn topic_1_2() {
    hello_world::execute();
}

#[allow(dead_code)]
fn topic_2() { guessing_game::execute(); }

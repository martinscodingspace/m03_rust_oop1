mod racer;

fn main() {
    println!("\nEin objektorientiertes Renn-Spiel\n");
    let vec_racers = racer::enter_racer();
    let mut race = racer::Race::new(vec_racers);
    race.start();
}

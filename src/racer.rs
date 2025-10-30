use std::io;
use rand::Rng;

pub struct Racer {
    name_name : String,
    position  : usize,
}

impl Racer {
    pub fn new(name: &str) -> Racer {
        Racer {
            name_name : name.to_string(),
            position  : 0,
        }
    }
    fn step(&mut self) {
        let zuf = rand::rng().random_range(1..=4);
        self.position = self.position + zuf;
    }
}

pub fn enter_racer() -> Vec<Racer> {
    let mut racers = Vec::new();
    loop {
        print!("Eingabe Teilnehmer-Name :  ");
        use std::io::Write;
        io::stdout().flush().unwrap();

        let mut name: String = String::new();
        io::stdin().read_line(&mut name).expect("Eingabe-Fehler!");
        let name = name.trim();

        if name.is_empty() { break; }

        racers.push(Racer::new(name));
    }

    if racers.is_empty() {
        racers = vec! [
            Racer::new("Rindt"),
            Racer::new("Lauda"),
            Racer::new("Berger"),
        ]
    }
    racers
}

pub struct Race {
    racers: Vec<Racer>,
    }

impl Race {
    pub fn new(racers: Vec<Racer>) -> Race {
        Race {racers}
    }
    pub fn start(&mut self) {
        let mut tick = 0;
        let rundenzahl = 120;
        loop {
            tick += 1;
            for r in &mut self.racers {
                r.step();
            }
            if tick >= rundenzahl { break;}
        }
        self.get_winner();
    }
    fn get_winner(&mut self) {
        let mut winner = &self.racers[0];
        for racer in self.racers.iter().skip(1) {
            if winner.position <= racer.position {
                winner = racer;
            }
        }
        println!("Der Gewinner ist : {}", winner.name_name);
    }
}

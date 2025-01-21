use std::thread::sleep;
use std::time::Duration;

#[derive(Debug)]
enum Trafic {
    Red,
    Yellow,
    Green,
}

impl Trafic {
    fn check(&self) -> &str {
        match self {
            Trafic::Red => "Red Light - Please stop",
            Trafic::Yellow => "Yellow Light - Ready to go",
            Trafic::Green => "Green Light - Go!",
        }
    }

    fn next(&self) -> Trafic {
        match self {
            Trafic::Red => Trafic::Yellow,
            Trafic::Yellow => Trafic::Green,
            Trafic::Green => Trafic::Red,
        }
    }

    fn time(&self) -> u64 {
        match self {
            Trafic::Red => 20,
            Trafic::Yellow => 5,
            Trafic::Green => 10,
        }
    }
}

fn main() {
    let mut light = Trafic::Red;

    loop {
        println!("{} for {} seconds", light.check(), light.time());
        interval(light.time(), 0);
        light = light.next();
    }
}

fn interval(sec: u64, nanosec: u32) {
    sleep(Duration::new(sec, nanosec));
}

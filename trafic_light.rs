use std::thread::sleep;
use std::time::Duration;
enum Trafic {
    Red,
    Green,
    Yellow,
}

impl Trafic {
    fn check(&self) -> &str {
        match self {
            Trafic::Red => "red Please Stop a",
            Trafic::Yellow => "yellew wait to go in  ",
            Trafic::Green => " Green go under  ",
        }
    }

    fn time(&self) -> u64 {
        match self {
            Trafic::Red => 20,
            Trafic::Yellow => 5,
            Trafic::Green => 10,
        }
    }

    fn update(&self) -> Self {
        match self {
            Trafic::Red => Trafic::Yellow,
            Trafic::Yellow => Trafic::Green,
            Trafic::Green => Trafic::Red,
        }
    }
    fn char(&self) -> char {
        match self {
            Trafic::Red => '🛑',
            Trafic::Yellow => '⏳',
            Trafic::Green => '🚀',
        }
    }
}

fn main() {
    let mut light = Trafic::Red;

    loop {
        println!(
            "signal {} {} {} second ",
            light.char(),
            light.check(),
            light.time()
        );
        sleep(Duration::new(light.time(), 0));
        light = light.update();
    }
}

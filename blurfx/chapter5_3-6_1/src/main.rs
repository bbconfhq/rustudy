#[derive(Debug)]
enum Tier {
    Bronze,
    Silver,
    Gold,
    Platinum,
    Diamond,
    Ruby,
    Master,
}

struct BOJ {
    handle: String,
    tier: Option<Tier>,
}

impl BOJ {
    fn new(handle: String, tier: Option<Tier>) -> Self {
        Self {
            handle,
            tier,
        }
    }

    fn print(&self) {
        match &self.tier {
            Some(tier) => println!("{}'s tier is {:?}", self.handle, tier),
            None => println!("{}'s tier is None. Maybe its Iron", self.handle),
        }
    }
}

fn main() {
    let mystika = BOJ::new("mystika".to_string(), Some(Tier::Platinum));
    let yun = BOJ::new("yungoon".to_string(), Some(Tier::Diamond));
    let heyongsik = BOJ::new("woo2484".to_string(), None);

    mystika.print();
    yun.print();
    heyongsik.print();
}

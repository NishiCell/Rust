trait PoiTrait {
    fn poi(&self) -> u32;
    fn poy(&self, z: u32) -> u32;
}
struct Poi {
    x: u32,
    y: u32,
}

impl PoiTrait for Poi {
    fn poi(&self) -> u32 {
        self.x * 2
    }

    fn poy(&self, z: u32) -> u32 {
        self.y + z
    }
}

fn main() {
    let p = Poi { x: 10, y: 20 };

    println!("{}{}", p.poi(), p.poy(3));
}

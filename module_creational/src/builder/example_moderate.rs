#[derive(Debug, PartialEq)]
pub struct House {
    has_walls: bool,
    has_doors: bool,
    has_windows: bool,
    has_roof: bool,
    has_garage: bool,
}

impl House {}

#[derive(Default)]
pub struct HouseBuilder {
    has_walls: bool,
    has_doors: bool,
    has_windows: bool,
    has_roof: bool,
    has_garage: bool,
}

impl HouseBuilder {
    pub fn new() -> HouseBuilder {
        HouseBuilder::default()
    }

    pub fn build_walls(mut self, build_walls: bool) -> HouseBuilder {
        self.has_walls = build_walls;
        self
    }

    pub fn build_doors(mut self, build_doors: bool) -> HouseBuilder {
        self.has_doors = build_doors;
        self
    }

    pub fn build_windows(mut self, build_windows: bool) -> HouseBuilder {
        self.has_windows = build_windows;
        self
    }

    pub fn build_roof(mut self, build_roof: bool) -> HouseBuilder {
        self.has_roof = build_roof;
        self
    }

    pub fn build_garage(mut self, build_garage: bool) -> HouseBuilder {
        self.has_garage = build_garage;
        self
    }

    pub fn get_result(self) -> House {
        House {
            has_walls: self.has_walls,
            has_doors: self.has_doors,
            has_windows: self.has_windows,
            has_roof: self.has_roof,
            has_garage: self.has_garage,
        }
    }
}

#[test]
fn builder_test() {
    let default_house = HouseBuilder::new()
        .build_walls(true)
        .build_doors(true)
        .build_roof(true)
        .get_result();

    let garage_house = HouseBuilder::new()
        .build_walls(true)
        .build_doors(true)
        .build_garage(true)
        .get_result();

    assert_eq!(default_house, garage_house);
}

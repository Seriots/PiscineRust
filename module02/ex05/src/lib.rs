struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    const WHITE: Self = Self::new(255, 255, 255);
    const RED: Self = Self::new(255, 0, 0);
    const GREEN: Self = Self::new(0, 255, 0);
    const BLUE: Self = Self::new(0, 0, 255);

    const fn new(red: u8, green: u8, blue: u8) -> Self {
        Self {
            red,
            green,
            blue,
        }
    }
}

// impl Color {
//     fn closest_mix(self, palette: &[(Self, u8)], max: u32) -> Self;
// }

// J'ai rien compris à ce qu'il fallait faire ici
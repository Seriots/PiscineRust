enum PizzaStatus {
    Ordered, //2
    Cooking, //5
    Cooked, //3
    Delivering, //7
    Delivered, //0
}

impl PizzaStatus {
    pub fn from_delivery_time(ordered_days_ago: u32) -> Self {
        match ordered_days_ago {
            0..=1 => Self::Ordered,
            2..=6 => Self::Cooking,
            7..=9 => Self::Cooked,
            10..=16 => Self::Delivering,
            _ => Self::Delivered,
        }
    }

    pub fn get_delivery_time_in_days(&self) -> u32 {
        match self {
            Self::Ordered => 17,
            Self::Cooking => 15,
            Self::Cooked => 10,
            Self::Delivering => 7,
            Self::Delivered => 0,
        }
    }
}

#[cfg(test)]
#[test]
fn pizza_test() {
    let pizza = PizzaStatus::from_delivery_time(8);
    assert_eq!(pizza.get_delivery_time_in_days(), 10);
}


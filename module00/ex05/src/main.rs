fn is_leap_year(year: u32) -> bool {
    if year == 0 { panic!("Year can't be 0"); }
    else if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) { true }
    else { false }
}

fn num_days_in_month(year: u32, month: u32) -> u32 {
    
    let days = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    if month > 11 {
        panic!("there is only 12 month (0-11)");
    }
    let m : usize = month.try_into().unwrap();
    if is_leap_year(year) && month == 1 {
        return 29;
    }
    else {
        return days[m];
    }
}

fn all_fridays() {
    let mut day = 2;
    let month = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "Novembre", "December"];
    for n in 1..=2023 {
        for m in 0..=11 {
            if (day + 13) % 7 == 0 {
                println!("Friday, {} 13, {}", month[m], n);
            }
            day += num_days_in_month(n, m as u32);
        }
    }
}

#[test]
#[cfg(test)]
#[should_panic(expected = "Year can't be 0")]
fn test_is_leap_year() {
    is_leap_year(0);
}

#[test]
#[cfg(test)]
fn all_test() {
    assert_eq!(is_leap_year(9872), true);
    assert_eq!(is_leap_year(987), false);
    assert_eq!(is_leap_year(25), false);
    assert_eq!(is_leap_year(1500), false);
    assert_eq!(is_leap_year(1200), true);
    assert_eq!(is_leap_year(1300), false);
    assert_eq!(is_leap_year(1600), true);
    assert_eq!(is_leap_year(2004), true);
    assert_eq!(is_leap_year(2003), false);
    assert_eq!(num_days_in_month(4, 1), 29);
    assert_eq!(num_days_in_month(256, 9), 31);
    assert_eq!(num_days_in_month(5, 1), 28);
    assert_eq!(num_days_in_month(4, 0), 31);
}

fn main() {
    all_fridays();
}

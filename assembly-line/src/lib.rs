// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
// #![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let total_cars: f64 = (speed as f64)*221.0;
    if speed >= 1 && speed <= 4 {
        return total_cars
    } else if speed >= 5 && speed <= 8 {
        return total_cars-(total_cars*0.1)
    } else {
        return total_cars-(total_cars*0.23)
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    let total_cars: f64 = (speed as f64)*3.6833;
    if speed >= 1 && speed <= 4 {
        return total_cars as u32
    } else if speed >= 5 && speed <= 8 {
        return (total_cars-(total_cars*0.1)) as u32
    } else {
        return (total_cars-(total_cars*0.23)) as u32
    }
}

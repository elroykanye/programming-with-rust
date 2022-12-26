fn main() {
    enum CardPoint {
        NORTH(String), 
        SOUTH(i32),
        EAST(f64),
        WEST(bool)
    }

    let north = CardPoint::NORTH(String::from("North"));
    let nord = CardPoint::NORTH(String::from("Nord"));

    let west = CardPoint::WEST(true);


// &str

    /* 
    const NORTH: char = 'N';
    const SOUTH: char = 'S';
    const EAST: char = 'E';
    const WEST: char = 'W';

    let point = NORTH;

    if point == NORTH {print!("{}", "North");}
    else if point == SOUTH {print!("{}", "South");}
    else if point == EAST {print!("{}", "East");}
    else if point == WEST {print!("{}", "West");}
    else {print!("Wahla");}
    */
}

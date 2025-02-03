struct ColorRegularStruct {
    red: u8,
    green: u8,
    blue: u8,
}

struct ColorTupleStruct(u8, u8, u8);

fn main() {
    let green_regular = ColorRegularStruct {
        red: 0,
        green: 255,
        blue: 0,
    };
    println!("Regular struct - Red: {}, Green: {}, Blue: {}", green_regular.red, green_regular.green, green_regular.blue);

    let green_tuple = ColorTupleStruct(0, 255, 0);
    println!("Tuple struct - Red: {}, Green: {}, Blue: {}", green_tuple.0, green_tuple.1, green_tuple.2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regular_structs() {
        let green = ColorRegularStruct {
            red: 0,
            green: 255,
            blue: 0,
        };
        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        let green = ColorTupleStruct(0, 255, 0);
        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }
}

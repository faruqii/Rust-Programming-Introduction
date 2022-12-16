fn trafic(condition: String) {
    if condition == "Green" {
        println!("Go!");
    } else if condition == "Yellow" {
        println!("Slow down!");
    } else if condition == "Red" {
        println!("Stop!");
    } else {
        println!("Not a valid traffic light color!");
    }
}

fn main() {
    let user_input = "Green";
    trafic(user_input.to_string());
}
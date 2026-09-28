mod report;
mod supplies;

fn main() {
<<<<<<< HEAD
    println!("Oregon Trail Supply Checks");
=======
    println!("Oregon Trail Supply Checkss");
>>>>>>> 6487595e78af05ec307b48371c9e1e4529f7ddd1

    let party: u32 = 4;
    let days: u32 = 30;

    let food = supplies::food_needed(party, days);

    report::print_report(party, days, food);
}

fn main() {
    let city_name = "Rustville";

    println!("\nThe city of {}:\n", city_name);

    print_population(1_324_578, 114_293, 108_097);
}

fn print_population(adults: u64, kids: u32, buildings: u32) {
    // DONE 👉 TODO compute population by adding adults to kids
    //
    // 💡 TIP: Use the `as` keyword to convert between numeric types!
    let population = adults + kids as u64;

    // DONE 👉 TODO compute buildings_per_person by dividing buildings by population
    //
    // 💡 TIP: To get a f64 answer here, both numerator and denominator must be f64 values
    let buildings_per_person = buildings as f64 / population as f64;

    println!("\tPopulation: {}", population);
    println!("\tAdults: {}", adults);
    println!("\tKids: {}", kids);
    println!("\tBuildings: {}", buildings);
    // DONE
    println!("\tBuildings per person: {:.2}\n", buildings_per_person);

    if buildings_per_person >= 1.0 {
        println!("\tEveryone can have their own building!");
    } else {
        println!("\tBuildings must be shared!");
    }
}

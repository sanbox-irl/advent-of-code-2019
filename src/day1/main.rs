use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input: Vec<usize> = include_str!("input.txt")
        .lines()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut fuel = 0;
    for mass in input {
        let module_req = convert_mass_to_fuel(mass);
        fuel += module_req + fuel_for_fuel(module_req);
    }

    println!("Final is {}", fuel);

    Ok(())
}

fn convert_mass_to_fuel(mass: usize) -> usize {
    ((mass as f32) / 3.0) as usize - 2
}

fn fuel_for_fuel(fuel_mass: usize) -> usize {
    // 9 is the most we give a shit about
    let mut additional_fuel = 0;
    let mut this_iteration_fuel = fuel_mass;

    while this_iteration_fuel > 8 {
        this_iteration_fuel = convert_mass_to_fuel(this_iteration_fuel);
        additional_fuel += this_iteration_fuel;
    }

    additional_fuel
}

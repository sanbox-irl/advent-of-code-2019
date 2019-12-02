use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input: Vec<usize> = include_str!("input.txt")
        .lines()
        .map(|s| s.parse().unwrap())
        .collect();

    // part 1
    println!(
        "(part1) Fuel for Module is {}",
        input.iter().map(convert_mass_to_fuel).sum::<usize>()
    );

    println!(
        "(part2) Fuel for Module and Fuel is {}",
        input
            .iter()
            .map(|m| fuel_for_fuel(convert_mass_to_fuel(&m)))
            .sum::<usize>()
    );
    Ok(())
}

fn convert_mass_to_fuel(mass: &usize) -> usize {
    ((*mass as f32) / 3.0) as usize - 2
}

fn fuel_for_fuel(fuel_mass: usize) -> usize {
    let mut additional_fuel = 0;
    let mut this_iteration_fuel = fuel_mass;

    // 8 is a heuristic, as we can check that we'll never get below or at 0 until 9
    while this_iteration_fuel > 8 {
        this_iteration_fuel = convert_mass_to_fuel(&this_iteration_fuel);
        additional_fuel += this_iteration_fuel;
    }

    additional_fuel + fuel_mass
}

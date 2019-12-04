const START: usize = 156218;
const END: usize = 652527;

fn main() {
    println!(
        "Number of valid combinations is {:?}",
        (START..END).filter(|n| check_is_valid(*n)).count()
    );
}

fn check_is_valid(input: usize) -> bool {
    let digit_array: [usize; 6] = {
        let mut array = [0; 6];

        let mut our_input = input;

        for digit in (0..6).rev() {
            array[digit] = our_input % 10;
            our_input /= 10;
        }

        array
    };

    // Digits do not decrease
    let mut max_numb = digit_array[0];
    for digit in digit_array.iter() {
        if digit < &max_numb {
            return false;
        } else {
            max_numb = *digit;
        }
    }

    // Duplicate Digit!
    let mut digit_counter = [0usize; 10];
    for digit in digit_array.iter() {
        digit_counter[*digit] += 1;
    }

    digit_counter.iter().filter(|d| **d == 2).count() != 0
}

const X: usize = 25;
const Y: usize = 6;

fn main() {
    let input: Vec<usize> = include_str!("input.txt")
        .chars()
        .map(|character| character.to_digit(10).unwrap() as usize)
        .collect();

    let layers: Vec<Vec<usize>> = {
        let mut layers = vec![];
        let mut current_layer = Some(vec![]);
        for (i, digit) in input.iter().enumerate() {
            current_layer.as_mut().map(|i| i.push(*digit));

            if (i + 1) % (X * Y) == 0 {
                layers.push(current_layer.take().unwrap());
                current_layer = Some(vec![]);
            }
        }

        layers
    };
    assert_eq!(layers.len(), input.len() / (X * Y));

    part1(layers.clone());
    part2(layers);
}

fn part1(layers: Vec<Vec<usize>>) {
    let (_, index) = layers
        .iter()
        .enumerate()
        .map(|(i, layer)| (layer.iter().filter(|&&digit| digit == 0).count(), i))
        .min()
        .unwrap();

    let mut ones = 0;
    let mut twos = 0;
    for digit in layers[index].iter() {
        match *digit {
            1 => ones += 1,
            2 => twos += 1,
            _ => {}
        }
    }

    println!("{}", ones * twos);
}

fn part2(layers: Vec<Vec<usize>>) {
    let mut output = vec![2; X * Y];

    for layer in layers {
        for (i, digit) in layer.into_iter().enumerate() {
            if output[i] == 2 {
                output[i] = digit;
            }
        }
    }

    for (i, digit) in output.iter().enumerate() {
        if *digit == 1 {
            print!("[]");
        } else {
            print!("  ");
        }

        if (i + 1) % X == 0 {
            println!();
        }
    }
}

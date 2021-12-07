fn main() {
    let s = include_str!("input.txt");

    let crab_positions: Vec<i64> = s
        .split('\n')
        .next()
        .unwrap()
        .split(',')
        .map(|pos| pos.parse::<i64>().unwrap())
        .collect();

    let mut lowest_fuel = i64::MAX;
    let crab_count = crab_positions.len();
    for target_position in 0..crab_count {
        let mut fuel = 0;
        for crab in &crab_positions {
            fuel += calc_move_cost(crab, &(target_position as i64));
        }
        if fuel < lowest_fuel {
            lowest_fuel = fuel;
        }
    }

    println!("lowest_fuel={}", lowest_fuel);
}

fn calc_move_cost(position: &i64, target: &i64) -> i64 {
    if position == target {
        return 0;
    }
    let mut cost = 0;
    let mut fuel_rate = 1;
    let mut initial_position = *position;
    let step_op = match position.cmp(target) {
        std::cmp::Ordering::Less => 1,
        std::cmp::Ordering::Equal => {
            panic!("position and target were the same, should have been caught earlier")
        }
        std::cmp::Ordering::Greater => -1,
    };

    while initial_position != *target {
        initial_position += step_op;
        cost += fuel_rate;
        fuel_rate += 1;
    }

    cost
}

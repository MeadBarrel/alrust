use core::panic;
use std::io;
use float_cmp::approx_eq;


pub fn prompt_str(prompt: &str) -> String {
    let mut result = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut result).unwrap();
    result.trim().to_string()
}


pub fn prompt_float(prompt: &str) -> f64 {
    match prompt_str(prompt).parse::<f64>() {
        Ok(x) => x,
        Err(_) => prompt_float(prompt)
    }
}


pub fn greet() {
    let response = prompt_str("Does your ingredient have alchemical weight (Y/N):");
    match response.as_str() {
        "Y" => solve_with_aw(),
        "N" => panic!("I DUNO HOW :("),
        _ => {
            println!("Please try again");
            greet();
        }
    }
}


fn solve_with_aw() {
    let result_initial = prompt_float("Put 11 units of ingredient in question into potion and type the result:");
    let lore_mp = prompt_float("What is the lore multiplier for this ingredient? (usually 1.66666)? ");
    
    if result_initial == 0. {
        return solve_for_mp_with_secondary(lore_mp);
    }

    let result_with_water = prompt_float("Put 11 units of ingredient in question and 11 units of purified water into potion and type the result: ");

    if approx_eq!(f64, result_initial / result_with_water, 2.0, epsilon = 0.01) {
        let result = result_initial / (2.4 / lore_mp);
        println!("Base value is {}, multiplier is 0", result);
    }

    let (base, multiplier) = solve_sys_with_water(lore_mp, result_initial, result_with_water);
    println!("Base: {}, Multiplier: {}", base, multiplier);
}


fn solve_for_mp_with_secondary(lore_mp: f64) {
    let result = prompt_float("Put 11 units of ingredient in question and 11 units of ingredient with known base and 0 multiplier and type the result: ");
    let base_value = prompt_float("What is the base value of the second ingredient for the property that you're seeking? ");
    let A = 1.2;
    let B = base_value;
    let L = lore_mp;

    let multiplier = (2.82843 * result) / (A * B * L) - std::f64::consts::SQRT_2;
    println!("Base is 0, Multiplier: {}", multiplier)
}


fn solve_sys_with_water(lore_mp: f64, result_1: f64, result_2: f64) -> (f64, f64) {
    let R = result_1;
    let Z = result_2;
    let L = lore_mp;
    let A = 1.2;

    let B = (6.82843 * Z - 2.41421 * R) / (A * L);
    let M = - (5.46084 * 10E+6 * (R - 2. * Z)) / (3.8614 * 10E+6 * R - 1.09217 * 10E+7 * Z);

    return (B, M)
}
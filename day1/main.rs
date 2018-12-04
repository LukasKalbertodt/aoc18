use std::{
    collections::HashSet,
    error::Error,
    fs,
    iter,
};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    let deltas = input.lines().map(|s| s.trim().parse::<i64>().unwrap());


    // ===== PART 1 ==========================================================
    println!("Part 1 solution: {}", deltas.clone().sum::<i64>());


    // ===== PART 2 (iterator version) =======================================
    let mut set = HashSet::new();

    let result = iter::repeat(deltas.clone())
        .flatten()
        .scan(0, |state, delta| {
            *state += delta;
            Some(*state)
        })
        .find(|sum| !set.insert(*sum));

    println!("Part 2 solution: {}", result.unwrap());


    // ===== PART 2 (loop version) ===========================================
    let mut set = HashSet::new();
    let mut sum = 0;

    for delta in iter::repeat(deltas).flatten() {
        sum += delta;
        if !set.insert(sum) {
            println!("Part 2 solution: {}", sum);
            break;
        }
    }



    Ok(())
}

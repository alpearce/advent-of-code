use itertools::Itertools;
fn main() {
    // Part 1
    let elven_lead = include_str!("input.txt")
        .lines()
        .map(|v| v.parse::<u64>().ok())
        .batching(|it| {
            let mut sum = None;
            while let Some(Some(v)) = it.next() {
                sum = Some(sum.unwrap_or(0) + v);
            }
            sum
        })
        .max()
        .unwrap();
    println!("elven lead: {elven_lead:?}");

    // Part 2
    let top_three = include_str!("input.txt")
        .lines()
        .map(|v| v.parse::<u64>().ok())
        .batching(|mut it| (&mut it).map_while(|x| x).sum1::<u64>())
        .map(std::cmp::Reverse)
        .k_smallest(3)
        .map(|x| x.0)
        .sum::<u64>();
    println!("top three: {top_three:?}");
}

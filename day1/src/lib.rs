// day1/src/lib.rs

// dependencies
use domain::Day1Puzzle1;
use parser::parse_numeric;

// function which solves the Day 1 challenge
pub fn day1_puzzle1_challenge(buffer: String) {
    println!("Day 1, Puzzle 1 Challenge");
    let mut location_id_lists = Day1Puzzle1::default();

    // parse the input into two lists
    println!("Parsing input...");
    for line in buffer.lines() {
        let (right_item, left_item) = parse_numeric(line).unwrap();
        location_id_lists.left_list.push(left_item.trim().parse().unwrap());
        location_id_lists.right_list.push(right_item.trim().parse().unwrap());
    }

    // sort the lists from lowest to highest
    println!("Sorting lists...");
    location_id_lists.left_list.sort();
    location_id_lists.right_list.sort();

    // calculate the distance between corresponding list elements
    println!("Calculating distances...");
    let mut distances: Vec<u32> = Vec::new();
    let pairs_list = location_id_lists.left_list.iter().zip(location_id_lists.right_list);
    for item in pairs_list {
        if item.0 < &item.1 {
            distances.push(item.1 - item.0)
        } else {
            distances.push(item.0 - item.1)
        }  
    } 

    // sum all the individual distances to get the total distance between pairs of location ids 
    println!("Calculating total distance...");
    let total_distance = distances.iter().sum::<u32>();
    println!("Total Distance between lists: {}", total_distance);
}

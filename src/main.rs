
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
fn main() {
    // File hosts must exist in current path before this produces output
    let file_path = "/home/stephen/src/rustprojects/advent-of-code-2022/day01/src/food.txt";
    // Size of 4 so you can update 4th each time and sort
    let mut top_three = vec![0, 0, 0, 0];
    if let Ok(lines) = read_lines(file_path) {
        // Consumes the iterator, returns an (Optional) String
        let mut biggest = 0;
        let mut running_total = 0;
        for line in lines {
            if let Ok(calories) = line {
                // println!("{}", calories);
                if calories == "" {
                    // End of block
                    // compare running_total with biggest. 
                    // If bigger then update biggest and reset running total
                    update_biggest(running_total, &mut biggest , &mut top_three );
                    running_total = 0;
                    continue;
                }
                if let Ok(int_calories) = calories.parse::<i32>() {
                    running_total = running_total + int_calories;
                }                
            }
        }

        // in case last line isn't empty string check the last block
        update_biggest(running_total, &mut biggest, &mut top_three );
        // Print final biggest block of calories
        println!("Biggest Block: {}", biggest);
        let result = &top_three[1..4];
        let mut sum = 0;
        for i in result.iter() {
            sum += i;
        }
        println!("Largest three payloads: {result:?}");
        println!("Total of biggest three: {}", sum);
    } else {
        println!("Unable to open file");
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn update_biggest(  running_total: i32, 
                    biggest: &mut i32, 
                    top_three: &mut Vec<i32> ) {
    // compare running_total with biggest. 
    // If bigger then update biggest and reset running total
    // println!("Block total: {}", running_total);
     
    if running_total > *biggest {
       *biggest = running_total;
       println!("Biggest Block: {}", biggest);
    }

    // Push update 4th vector field with block total and sort
    top_three[0] = running_total;
    top_three.sort();
    // println!("{top_three:?}");
}
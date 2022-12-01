use std::fs;

fn main() {
    let mut current_total: u32 = 0;
    let mut n = 0;
    let mut block_count = 1;

    let contents = fs::read_to_string("input.txt")
        .expect("Unable to read the file!");

    let x= contents.split("\n");

    for n in x {
        if !(n.is_empty()) {
            block_count += 1;
        } 
    }
    
    let mut val_array = vec![0; block_count];

    let l= contents.split("\n");
    for s in l {

        if !(s.is_empty()) {
            let val: u32 = s.parse().unwrap();
            current_total += val;
        } 
        else{
            val_array[n] = current_total;
            n += 1;
            current_total = 0;
        }
    } 

    val_array.sort();
    val_array.reverse();

    let total = val_array[0]+val_array[1]+val_array[2];
    println!("Top calories: {}", val_array[0]);
    println!("Total of top 3: {}", total);

}
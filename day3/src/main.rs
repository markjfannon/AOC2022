use std::fs::read_to_string;

fn main() {
    task1();
    task2();
}

fn task1() {
    //Opens file and splits into lines
    let contents = read_to_string("input.txt")
        .expect("Unable to read file!");
    let splits = contents.split("\n");
    let mut score = 0;

    //Loops through each line, calculates length and splits in half
    for n in splits {
        let mid = n.len() / 2;
        let (comp1, comp2) = n.split_at(mid);
        //Finds common character and converts into a priority to be added to total score
        let val: u32 = find_common(comp1, comp2).into();
        if val < 91 {
            score += val - 38;
        }
        else {
            score += val - 96;
        }
    }

    println!("Task1 total is: {score}");

}

fn task2() {
    //Opens file and splits into lines
    let contents = read_to_string("input.txt")
        .expect("Unable to read file!");
    let splits = contents.split("\n");
    let mut score = 0;
    let mut it = 0;
    let mut rucksacks: Vec<&str> = Vec::new();


    //Pushes each line to a vec
    for n in splits {
        rucksacks.push(n);
    }

    //Defines length of rucksacks to make iteration easier
    let rucksacks_length = rucksacks.len();

    loop {
        //Exits code if the end of rucksacks is hit
        if it == rucksacks_length {
            break;
        }

        //Finds common value of 3 lines, adds relavent score
        let val: u32 = find_common_3(rucksacks[it], rucksacks[it+1], rucksacks[it+2]).into();
        if val < 91 {
            score += val - 38;
        }
        else {
            score += val - 96;
        }

        //Moves to next block of 3 lines
        it += 3
    };

    println!("Task2 total is: {score}");


}

//Finds common char value for 2 str inputs
fn find_common (a: &str, b: &str) -> char{
    let mut found = false;
    for c in a.chars() {
        for d in b.chars() {
            if c == d {
                found = true;
                if found == true {
                    return c;
                }
            }
        }
    }

    if found == false {
        return '0';
    }

    else {
        return '0';
    }
}

//Finds common char value for 3 str inputs;
fn find_common_3 (a: &str, b: &str, c: &str) -> char{
    let found = false;
    for i in a.chars() {
        for j in b.chars() {
            for k in c.chars() {
                if i == j && j == k {
                    return i;
                }

            }
        }
    }

    if found == false {
        return '0';
    }

    else {
        return '0';
    }
}

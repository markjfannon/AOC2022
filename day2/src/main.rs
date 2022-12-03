use std::fs::read_to_string;

fn main() {

    task1();
    task2();
}

fn task1() {
    let mut score = 0;

    let contents = read_to_string("input.txt")
        .expect("Unable to read file!");
    let splits = contents.split("\n");

    for n in splits {
        let moves : Vec<&str> = n.split(" ").collect();
        if moves[0] == "A" {
            match moves[1] {
                "X"=>score+=4,
                "Y"=>score+=8,
                "Z"=>score+=3,
                _ => {}
            }
        }

        if moves[0] == "B" {

            match moves[1] {
                "X"=>score+=1,
                "Y"=>score+=5,
                "Z"=>score+=9,
                _ => {}
            }
        }

        if moves[0] == "C" {

            match moves[1] {
                "X"=>score+=7,
                "Y"=>score+=2,
                "Z"=>score+=6,
                _ => {}
            }
        }
    }

    println!("Original strategy score is: {score}");
}


fn task2() {
    let mut score = 0;

    let contents = read_to_string("input.txt")
        .expect("Unable to read file!");
    let splits = contents.split("\n");

    for n in splits {
        let moves : Vec<&str> = n.split(" ").collect();
        if moves[0] == "A" {
            match moves[1] {
                "X"=>score+=3,
                "Y"=>score+=4,
                "Z"=>score+=8,
                _ => {}
            }
        }

        if moves[0] == "B" {

            match moves[1] {
                "X"=>score+=1,
                "Y"=>score+=5,
                "Z"=>score+=9,
                _ => {}
            }
        }

        if moves[0] == "C" {
            match moves[1] {
                "X"=>score+=2,
                "Y"=>score+=6,
                "Z"=>score+=7,
                _ => {}
            }
        }
    }

    println!("True strategy score is: {score}");
}
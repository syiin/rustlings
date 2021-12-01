// option1.rs
// Make me compile! Execute `rustlings hint option1` for hints


// you can modify anything EXCEPT for this function's sig
fn print_number(maybe_number: Option<u16>) {
    println!("printing: {}", maybe_number.unwrap());
}

fn main() {
    print_number(Option::Some(13));
    print_number(Option::Some(99));

    let mut numbers: [Option<u16>; 5] = [None, None, None, None, None];
    // let mut numbers: Option<[u16; 5]> = Some([1,1,1,1,1]);
    for iter in 0..5 {
        let number_to_add: u16 = {
            ((iter * 1235) + 2) / (4 * 16)
        };
        match numbers[iter as usize] {
            Some(mut arr_item) => { arr_item = number_to_add },
            None => println!("Nothing inside!")
        }

        // match numbers {
        //     Some(mut arr) => { arr[iter as usize] = number_to_add },
        //     None => println!("Nothing inside!")
        // }
    }
}
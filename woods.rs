use std::io;

fn main() {
    println!("Woods - a text-based game written in Rust by Dante Falzone");
    println!("Source code in /home/public/devel/rust/woods.rs");
    println!("---------------------------------------------------------------");
    println!("You awaken to find yourself in the middle of a forest.");
    println!("Looking around yourself, you see a floor of soft leaf");
    println!("litter and tall fir trees with moss clinging to the reddish");
    println!("bark. Birds and rodents wander in the highest branches.");

    // Wow, I'm so glad that Rust allows for nested function definitions!
    // I'd either have to use a goto or put this whole function somewhere else
    // to do this in C or C++.
    fn choice_0() {
        let mut input = String::new();
        println!("\nPress a number to select an option:");
        println!("0 - get up");
        println!("1 - inspect yourself");
        println!("2 - do nothing");

        io::stdin().read_line(&mut input).expect("oof");
        input.pop();
        evaluate_choice_0(input);
    }

    // Although I must admit that the way Rust takes arguments strikes me as a
    // little unintuitive, coming from Python and C++. But it's not *too* crazy.
    fn evaluate_choice_0(input: String) {
        if input == "0" {
            get_up();
        } else if input == "1" {
            inspect_self();
        } else if input == "2" {
            do_nothing();
        } else {
            println!("Error - invalid option");
            choice_0();
        }
    }

    choice_0();

    fn do_nothing() {
        println!("\nBeing still sleepy, you go back to sleep for a little while.");
        choice_0();
    }
}

fn get_up() {
    println!("\nGetting up, you brush a few leaves out of your hair.");
    println!("You stretch; it feels very good to do so. The air smells");
    println!("fresh and you see that the sun has just risen over the");
    println!("horizon a bit. It suddenly occurs to you that you are hungry.");
    println!("You see your backpack sitting on the ground a few meters away.");
    choice_1();

    fn choice_1() {
        println!("\nPress a number to select an option:");
        println!("0 - go to the backpack");
        println!("1 - go walking");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("oof");
        input.pop();
        evaluate_choice_1(input);
    }

    fn evaluate_choice_1(input: String) {
        if input == "0" {
            go_to_backpack();
        } else if input == "1" {
            go_walking(0, 0);
            // first argument indicates how much food you
            // have; second indicates whether you have the backpack
        } else {
            println!("Error - invalid option");
            choice_1();
        }
    }
}

fn inspect_self() {
    println!("\nStill lying down, you look at yourself. You're wearing");
    println!("comfortable clothes: a light jacket, some old jeans, a t-shirt.");
    println!("There's a sleeping bag under you, but you're not wrapped in it.");
    println!("You decide to get up.");

    get_up();
}

fn go_to_backpack() {
    println!("\nYou go over to your backpack. It is made of olive-green canvas with");
    println!("brown leather straps. You unzip it and find three bottles of purified");
    println!("water, some granola bars, a first-aid kit, a bag of beef jerky, and a");
    println!("folding knife with a wooden handle. You unzip another compartment and");
    println!("find a cell phone and an empty book.");
    choice_3(3); // the integer value represents roughly how much food is available: eating decrements it by 1

    fn choice_3(food: u8) {
        println!("\nPress a number to select an option:");
        println!("0 - eat");
        println!("1 - try the cell phone");
        println!("2 - inspect the contents of the first-aid kit");
        println!("3 - put the backpack on and go walking");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("oof");
        input.pop();
        evaluate_choice_3(input, food);
    }

    fn evaluate_choice_3(input: String, food: u8) {
        // Wow, look how fancy I am, I'm using pointers like a Real Programmer
        let foodptr = &food;
        let mut food_amount = *foodptr;

        if input == "0" {
            if food_amount > 0 {
                food_amount -= 1; // Rust doesn't have the ++ or -- operators, to which I can only say, "eh"
                println!("\nYou have some of the granola bars and beef jerky. You feel less hungry.");
                choice_3(food_amount);
            } else {
                println!("\nYou're all out of food.");
                choice_3(food_amount);
            }
        } else if input == "1" {
            println!("\nThe cell phone is an old flip-phone, probably from the late aughts. Upon opening it,");
            println!("you find that it has only half a battery charge, and no bars.");
            choice_3(food_amount);
        } else if input == "2" {
            println!("\nYou open the first-aid kit. It contains rubbing alcohol, bandaids, an emergency suture");
            println!("kit, a box of matches, some painkillers, and a few rolls of gauze wrap.");
            choice_3(food_amount);
        } else if input == "3" {
            go_walking(food_amount, 1); // second argument indicates whether you have the backpack or not
        } else {
            println!("Error - invalid option");
            choice_3(food_amount);
        }        
    }
}

fn go_walking(food: u8, have_backpack: u8) {
    println!("Amount of food: {}", food);
    println!("Whether you have the backpack: {}", have_backpack);
    println!("\nYou walk for a while.");
    println!("\nThe just-risen sun casts little spots of light upon the ground, tinged");
    println!("greenish in the most thickly-forested areas.");
}

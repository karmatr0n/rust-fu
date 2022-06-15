#[derive(PartialEq, Debug)]
enum FarmAnimal {
    Worm,
    Cow,
    Bull,
    Chicken { num_eggs: usize },
    Dog { name: String },
}

fn what_does_the_animal_say(animal: &FarmAnimal) {
    /* TODO: fill in the match statement below to make this code compile */

    let noise = match animal {
        FarmAnimal::Cow | FarmAnimal::Bull => "moo".to_string(),
        //FarmAnimal::Chicken { num_eggs: _ } => "cluck, cluck!".to_string(),
        FarmAnimal::Chicken { .. } => "cluck, cluck!".to_string(),
        //FarmAnimal::Dog { name } if name == "Lassie" => format!("woof, woof!"),
        FarmAnimal::Dog { name } => format!("woof, woof! I am {}!", name),
        FarmAnimal::Dog { name } if name == "Lassie" => format!("woof, woof!"),
        _ => "-- (silence)".to_string(),
    };

    /* Bonus task: Give Dogs named Lassie a different output */

    println!("{:?} says: {:?}", animal, noise);
}

fn main() {
    what_does_the_animal_say(&FarmAnimal::Dog {
        name: "Lassie".to_string(),
    });
    what_does_the_animal_say(&FarmAnimal::Cow);
    what_does_the_animal_say(&FarmAnimal::Bull);
    what_does_the_animal_say(&FarmAnimal::Chicken { num_eggs: 3 });
    what_does_the_animal_say(&FarmAnimal::Worm);
}

use rand::{prelude::*, rng};

// Shuffle users using the Fisher-Yates shuffle algorithm
pub fn shuffle(users: &mut Vec<String>) {
    let mut rng = rng();

    // Here we create a for loop with
    // a reverse of the range of numbers from 1 to the length of the users(exclusive)
    for i in (1..users.len()).rev() {
        let random_index = rng.random_range(0..=i);
        users.swap(i, random_index);
    }
}

pub fn generate_pairings(users: &Vec<String>) -> Vec<(String, String)> {
    // TODO: Add an error check for empty arrays

    //create the givers iterator
    let givers = users.iter();

    // To create a cyclic receiver sequence, we create a user list that moves the
    // first user at the bottom or end of the list
    // basically
    let receivers_skipping_first_user = users.iter().skip(1);
    let first_user = users.iter().take(1);

    //Then we chain these together, this puts the first user at the bottom
    let receivers = receivers_skipping_first_user.chain(first_user);

    // We can also skip the unecessary declarations like this
    //let receivers = users.iter().skip(1).chain(users.iter().take(1));

    // zip: This combines the giver and receiver iterators and produces a (&String, &String) tuple
    // map: This transformes the string references to owned strings using the clone method
    // collect: This gathers all the tuples into a new vector
    givers
        .zip(receivers)
        .map(|(giver, receiver)| (giver.clone(), receiver.clone()))
        .collect()
}

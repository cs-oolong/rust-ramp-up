fn main() {
    println!("Hello from Cassino binary!");
}

// user can add cash (not real cash though) to their account
// user can list available pending battles
// user can create a simple bets
    // has to provide the battle id
    // has to pick just one among the possible events
// user can create a combined bet regarding a single battle, but multiple events within it
    // provide battle id
        // pick N events
// user can create a combined bet regarding N battles and M events
    // provide battle id
        // provide events
    // provide battle id
        // provide events
    // and so on, until done
// placing a bet deducts from the user's cash
// can't bet if no cash
// events have odds, if they occur, player receives cash
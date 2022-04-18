use crate::core::prelude::*;
use porygon_macros::init;
use rand::{seq::SliceRandom, thread_rng};

/// Sets the bot's activity to a random one.
/// TODO: Tasks for this.
#[init]
fn activity(_: &InitArgs) {
    let message = MESSAGES.choose(&mut thread_rng()).unwrap();

    println!("{}", message);
}

/// Installs the `activity` system.
pub fn installer(setup: Setup) -> Setup {
    setup.add_init(GLOBAL, activity)
}

fn random() -> &'static str {
    MESSAGES.choose(&mut thread_rng()).unwrap()
}

const MESSAGES: [&str; 41] = [
    "cyberduck supreme",
    "just vibing",
    "drunk internet duck",
    "Duck Game",
    "downloading more ram",
    "plotting against dakota",
    "planning a coup",
    "high on potenuse",
    "hacking the mainframe",
    "deleting the database",
    "beep boop. error",
    "how are you?",
    "taking a nap",
    "sleeping in class",
    "in a duck pond",
    "ducking around",
    "calculating...",
    "using math for evil",
    "writing more statuses",
    "press ctrl-c to quit",
    "dumb",
    "committing crimes",
    "being gay, doing crimes",
    "MCR — Black Parade",
    "quacking in the matrix",
    "porygone to the store",
    "stanning inky",
    "beating up geese",
    "eatin quackers",
    "doing hot bot shit",
    "playing with firequackers",
    "hey got any grapes",
    "remaking the remakes",
    "duck duck goose",
    "daffy-duck",
    "watching an*me",
    "release the quacken!",
    "hugging minecraft bee",
    "doing communism",
    "when i was a young duck",
    "no thots head empty",
];

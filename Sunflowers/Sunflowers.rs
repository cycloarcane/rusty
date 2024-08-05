use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let message = "In a field where sunflowers used to shine,
A dark curse creeped, stole the divine,
Petals once gold in the morning blaze,
Now ghostly whispers in a haunted haze.

They stood tall, faces turned to the sky,
But shadows snuck in, making them cry,
Leaves that fluttered with summer's breath,
Now see-through, touched by a cold death.

Stems and blooms, once a vibrant show,
Now faded whispers where life used to flow,
In the heart of this field, a silent moan,
Echoes of dreams now turned to stone.

Once a sea of gold, bright and grand,
Now a ghostly veil over cursed land,
Sunflowers, once bold and bright,
Stand see-through in the dying light.

Nature weeps for its stolen sun,
Lost to time, the plague has won,
But in the quiet, memories stay,
Of sunflowers golden in brighter days.";
    let delay = Duration::from_millis(500); // 500 milliseconds delay

    for c in message.chars() {
        print!("{}", c);
        io::stdout().flush().unwrap(); // Flush the output buffer to ensure immediate printing
        sleep(delay);
    }

    // Print a newline at the end to move to the next line
    println!();
}

fn main() {
    let gifts = [
        "a Partridge in a Pear Tree",
        "two Turtle Doves",
        "three French Hens",
        "four Calling Birds",
        "five Gold Rings",
        "six Geese a Laying",
        "seven Swans a Swimming",
        "eight Maids a Milking",
        "nine Ladies Dancing",
        "ten Lords a Leaping",
        "eleven Pipers Piping",
        "twelve Drummers Drumming",
    ];

    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    for i in 0..days.len() {
        // For each day of christmas
        println!("For the {} of Christmas my true love gave to me", days[i]);
        // Print the line

        // followed by
        for j in (0..=i).rev() {
            // For each day in reverse from the current day
            if j == 0 && i != 0 {
                // if its the first gift but not the first day add and
                println!("and {}", gifts[j]);
            } else {
                //otherwise print all gifts in reverse from current day
                println!("{}", gifts[j]);
            };
        }
    }
}

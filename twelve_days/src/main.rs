use ordinal::Ordinal;

fn main() {
    let gifts = [
        "a partridge in a pear tree",
        "two turtle doves",
        "three french hens",
        "four calling birds",
        "five gold rings",
        "six geese a-laying",
        "seven swans a-swimming",
        "eight maids a-milking",
        "nine ladies dancing",
        "ten lords a-leaping",
        "eleven pipers piping",
        "twelve drummers drumming"
    ];

    for i in 1..gifts.len() + 1 {
        println!("on the {} day of christmas my true love sent to me...", Ordinal(i).to_string());
        for j in (0..i).rev() {
            if i != 1 && j == 0 {
                print!("and ");
            }
            println!("{}", gifts[j]);
        }
    }
}

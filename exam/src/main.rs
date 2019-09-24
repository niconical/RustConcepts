const NUMBER: [&str; 12] = [
    "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth",
    "11th", "12th",
];

fn song_of_christmas(n: usize) {
  match n {
    0 => println!("A partridge in a pear tree"),
    1 => println!("Two turtle doves, and"),
    2 => println!("Three french hens"),
    3 => println!("Four calling birds"),
    4 => println!("Fiven golden rings"),
    5 => println!("Six geese a-laying"),
    6 => println!("Seven swans a-swimming"),
    7 => println!("Eight maids a-milking"),
    8 => println!("Nine ladies dancing"),
    9 => println!("10 lords a-leaping"),
    10 => println!("11 pipers piping"),
    11 => println!("12 drummers drumming"),
    _ => println!("ERROR"),
  }
  if n > 0 {
    song_of_christmas(n - 1);
  }
}

fn main() {
    for element in (0..12) {
      println!("[Verse {}]", element + 1);
      println!("On the {} day of Christmas my true love sent to me", NUMBER[element]);
      song_of_christmas(element);
      println!();
    }
}

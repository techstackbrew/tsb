use std::env;
use open;

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() > 2 {
    let command = &args[1];
    let option = &args[2];

    if command == "brew" && option == "upgrade" {
      let path = "http://techstackbrew.com";

      match open::that(path) {
        Ok(()) => println!("TSB brew upgrade in progress @ techstackbrew.com..."),
        Err(err) => eprintln!("An error occurred when opening '{}': {}", path, err),
      }
    } else {
      println!("To get started, please run:\n\n  tsb brew upgrade");
    }
  } else {
    println!("\n\n
    @@@@@@@@@@@@@@@@@@.    @@@@@@@@@@@@@@@@    @@@@@@@@@@@@@@@@@@@.
    @@@@@@@@@@@@@@@@@@.   @@@@@@@@@@@@@@@@@@   @@@@@@@@@@@@@@@@@@@@
    @@@@@@@@@@@@@@@@@@.   @@@@@@@@  @@@@@@@@   @@@@@@@@@@@@@@@@@@@@
        @@@@@@@#         @@@@@@@    @@@@@@@    @@@@@@@@    @@@@@@@@
        @@@@@@@#         @@@@@@@@   @@@@@@@    @@@@@@@@    @@@@@@@@
        @@@@@@@#         @@@@@@@@@@            @@@@@@@@   #@@@@@@@,
        @@@@@@@#           @@@@@@@@@@%         @@@@@@@@@@@@@@@@@@#
        @@@@@@@#             @@@@@@@@@@@       @@@@@@@@@@@@@@@@@@
        @@@@@@@#                @@@@@@@@@@     @@@@@@@@@@@@@@@@@@@
        @@@@@@@#                  @@@@@@@@@    @@@@@@@@    @@@@@@@@
        @@@@@@@#         @@@@@@@   @@@@@@@@    @@@@@@@   .@@@@@@@@@
        @@@@@@@#         @@@@@@@    @@@@@@@    @@@@@@    @@@@@@@@@@
        @@@@@@@#         @@@@@@@@*(@@@@@@@@    @@@@@@@.    @@@@@@@@
        @@@@@@@#         @@@@@@@@@@@@@@@@@@    @@@@@@@@@.   @@@@@@@
        @@@@@@@#          @@@@@@@@@@@@@@@@     @@@@@@@@@@@@@@@@@@@.
        @@@@@@@#           @@@@@@@@@@@@@@      @@@@@@@@@@@@@@@@@@\n\n\n");

    println!("  Welcome to the Tech Stack Brewing CLI tool!\n  To get started, please run:\n\n    tsb brew upgrade\n");
  }
}

mod user_interface;

fn main() {
    println!("Hello, world!");
    let result = user_interface::main();
    println!("{:?}", result);
    println!("bye bye!");
}

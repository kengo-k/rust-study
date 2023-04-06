mod foo;

fn main() {
    let message = foo::bar::util::hello("World");
    println!("{}", message);
}

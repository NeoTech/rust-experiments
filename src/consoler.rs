/* A very complicated way of writing hello world in console. */

/* Creates a trait with the fn new, set, write */
trait Writer {
    fn new() -> Self;
    fn set(&mut self, data: String);
    fn write(&self);
}

/* Creates a struct with the trait Writer */
struct ConsoleWriter {
    data: String,
}

/* Implements the trait Writer for the struct ConsoleWriter */
impl Writer for ConsoleWriter {
    fn new() -> Self {
        ConsoleWriter { data: String::new() }
    }

    fn set(&mut self, data: String) {
        self.data = data;
    }

    fn write(&self) {
        println!("{}", self.data);
    }
}

fn consoler() {
    let mut writer = ConsoleWriter::new();
    writer.set("Hello, World!".to_string());
    writer.write();
}

fn main() {
    consoler();
}
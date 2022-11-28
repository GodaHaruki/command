use std::collections::HashMap;

pub struct Command<T: Sized, V: Sized> {
    command_name: String,
    f: Box<dyn Fn(Option<&[T]>) -> V>,
}

impl<T, V> Command<T, V> {
    pub fn exec(&self, args: Option<&[T]>) -> V {
        (self.f)(args)
    }
}

pub struct Commands<T: Sized, V: Sized> {
    commands: HashMap<String, Command<T, V>>,
}

impl<T, V> Commands<T, V> {
    pub fn add(&mut self, command_name: &String, command_func: Box<dyn Fn(Option<&[T]>) -> V>) {
        self.commands.insert(
            command_name.clone(),
            Command::<T, V> {
                command_name: command_name.clone(),
                f: command_func,
            },
        );
    }

    pub fn get(&self, key: &String) -> Option<&Command<T, V>>{
        self.commands.get(key)
    }
}
fn main() {
    let mut commands = Commands::<(), String> {
        commands: {
            let mut h = HashMap::new();
            h.insert(
                "ping".to_string(),
                Command::<(), String> {
                    command_name: "ping".to_string(),
                    f: Box::new(|args| "pong".to_string()),
                },
            );
            h
        }
    };

    let c = commands.get(&"ping".to_string()).unwrap();
    let log = c.exec(None);
    println!("{}", log);

    commands.add(&"add_test".to_string(), Box::new(|args| "Wow add_test was successful".to_string()));
    let c = commands.get(&"add_test".to_string()).unwrap();
    let log = c.exec(None);
    println!("{}", log)
}

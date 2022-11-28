use command::commands::*;
use std::collections::HashMap;

fn main() {
    let mut commands = Commands::<(), String> {
        commands: {
            let mut h = HashMap::new();
            h.insert(
                "ping".to_string(),
                Command::<(), String>::from(
                    "ping".to_string(),
                    Box::new(|_args| "pong".to_string()),
                ),
            );
            h
        },
    };

    let c = commands.get(&"ping".to_string()).unwrap();
    let log = c.exec(None);
    println!("{}", log);

    commands.add(
        &"add_test".to_string(),
        Box::new(|_args| "Wow add_test was successful".to_string()),
    );
    let c = commands.get(&"add_test".to_string()).unwrap();
    let log = c.exec(None);
    println!("{}", log)
}

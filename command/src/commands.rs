use std::collections::HashMap;

pub struct Command<T: Sized, V: Sized> {
    pub command_name: String,
    pub f: Box<dyn Fn(Option<&[T]>) -> V>,
}

impl<T, V> Command<T, V> {
    pub fn from(command_name: String, f: Box<dyn Fn(Option<&[T]>) -> V> ) -> Command<T, V>{
        Command{
            command_name,
            f
        }
    }
    pub fn exec(&self, args: Option<&[T]>) -> V {
        (self.f)(args)
    }
}

pub struct Commands<T: Sized, V: Sized> {
    pub commands: HashMap<String, Command<T, V>>,
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_get(){
        let commands = Commands::<(), String> {
            commands: {
                let mut h = HashMap::new();
                h.insert(
                    "ping".to_string(),
                    Command::<(), String> {
                        command_name: "ping".to_string(),
                        f: Box::new(|_args| "pong".to_string()),
                    },
                );
                h
            }
        };
    
        let c = commands.get(&"ping".to_string()).unwrap();
        let res = c.exec(None);
        assert_eq!("pong".to_string(), res);
    }

    #[test]
    fn add_test(){
        let mut commands = Commands::<(), String> {
            commands: HashMap::new()
        };
    
        commands.add(&"added".to_string(), Box::new(|_args| "added_by_program".to_string()));
        let added_message = commands.get(&"added".to_string()).unwrap().exec(None);
        assert_eq!("added_by_program".to_string(), added_message);
    }
}
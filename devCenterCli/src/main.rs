use std::env;
mod dev_commands;

fn main() {
    let args: Vec<String> = env::args().collect();

    let command = args.get(1);

    let mut dev_center = dev_commands::DevCenter::new();

    match command {
        Some(cmd) if cmd == "list" => dev_center.list_proyects(),
        Some(cmd) if cmd == "add" => {
            let name = args.get(2).expect("Project name is required for add command");
            dev_center.create_proyect(name);
        }
        Some(cmd) if cmd == "remove" => {
            let name = args.get(2).expect("Project name is required for remove command");
            dev_center.delete_proyect(name);
        }
        Some(cmd) if cmd == "execute" => {
            let name = args.get(2).expect("Project name is required for execute command");
            dev_center.execute_proyect(name);
        }
        _ => println!("Unknown command. Available commands: list, add <name>, remove <name>, execute <name>"),
    }
}

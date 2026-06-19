use std::env;
mod dev_commands;

fn main() {
    let args: Vec<String> = env::args().collect();

    let command = args.get(1);

    match command {
        Some(cmd) if cmd == "list" => dev_commands::list_proyects(),
        Some(cmd) if cmd == "create" => {
            let name = args.get(2).expect("Project name is required for create command");
            dev_commands::create_proyect(name);
        }
        Some(cmd) if cmd == "delete" => {
            let name = args.get(2).expect("Project name is required for delete command");
            dev_commands::delete_proyect(name);
        }
        Some(cmd) if cmd == "execute" => {
            let name = args.get(2).expect("Project name is required for execute command");
            dev_commands::execute_proyect(name);
        }
        _ => println!("Unknown command. Available commands: list, create <name>, delete <name>, execute <name>"),
    }
}

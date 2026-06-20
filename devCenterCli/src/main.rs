use std::env;
mod dev_commands;
mod proyect;

fn main() {
    let args: Vec<String> = env::args().collect();

    let command = args.get(1);

    let mut dev_center = dev_commands::DevCenter::new();

    match command {
        Some(cmd) if cmd == "list" => dev_center.list_proyects(),
        //%get es para obtener un proyecto por nombre o id, el valor se toma del tercer argumento (args[2]).
        // basicamente si no se puede parsear como un numero, se asume que es un nombre.
        Some(cmd) if cmd == "get" => {
            let value = args
                .get(2)
                .expect("Tenes que pasar un nombre o id del proyecto");

            let project = match value.parse::<usize>() {
                Ok(id) => dev_center.get_proyect(None, Some(id)),
                Err(_) => dev_center.get_proyect(Some(value), None),
            };

            match project {
                Some(project) => println!("Encontrado: {} - {}", project.id, project.name),
                None => println!("Proyecto no encontrado"),
            }
        }
        Some(cmd) if cmd == "add" => {
            let name = args.get(2).expect("El nombre del proyecto es requerido para el comando add");
            dev_center.create_proyect(name);
        }
        Some(cmd) if cmd == "remove" => {
            let name = args.get(2).expect("El nombre del proyecto es requerido para el comando remove");
            dev_center.delete_proyect(name);
        }
        Some(cmd) if cmd == "execute" => {
            let name = args.get(2).expect("El nombre del proyecto es requerido para el comando execute");
            dev_center.execute_proyect(name);
        }
        _ => println!("Comando no reconocido. Usa 'list', 'get <name>', 'add <name>', 'remove <name>' o 'execute <name>'."),
    }
}

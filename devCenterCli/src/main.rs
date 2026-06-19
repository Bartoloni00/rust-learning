mod devCommands;

fn main() {
    devCommands::list_proyects();
    devCommands::create_proyect("MyProject");
    devCommands::delete_proyect("MyProject");
    devCommands::execute_proyect("MyProject");
}

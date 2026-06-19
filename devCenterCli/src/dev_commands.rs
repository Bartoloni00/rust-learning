/* 
esto no Funciona porque no se puede usar una variable global mutable sin un contenedor de seguridad como Mutex o RefCell, y además,4
 el vector projects no está definido en el ámbito de las funciones.
pub struct Proyect {
    name: String,
    // Add more fields as needed
}

// la palabra reservada mut es para indicar que la variable puede ser modificada después de su creación. 
// En este caso, se declara un vector mutable llamado projects que almacenará instancias de la estructura Proyect.
// Usar un vector es como crear un array
let mut projects: Vec<Proyect> = Vec::new();
*/
pub struct Proyect {
    name: String,
    // Add more fields as needed
}

pub struct DevCenter {
    projects: Vec<Proyect>,
}

impl DevCenter {

    // Esto seria un constructor para la estructura DevCenter, que inicializa el vector projects como un vector vacío.
    pub fn new() -> Self {
        DevCenter {
            projects: Vec::new(),
        }
    }

    // %self indica que solo sera de lectura, ademas da a entender que no es una funcion estatica y puede llamarse con "." en lugar de "::"
    pub fn list_proyects(&self) {
    println!("Listing projects...");
    }

    pub fn create_proyect(&mut self, name: &str) {
        // tenemos que utilizar self para acceder a la variable projects (es como un this.)
        self.projects.push(Proyect {
            name: name.to_string(),
        });
        println!("Project '{}' created.", name);
    }

    pub fn delete_proyect(&mut self, name: &str) {
        println!("Deleting project: {}", name);
    }

    pub fn execute_proyect(&self, name: &str) {
        println!("Executing project: {}", name);
    }
}
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

use std::fs;
use crate::proyect::Proyect;

pub struct DevCenter {
    projects: Vec<Proyect>,
    next_id: usize,
}

impl DevCenter {

    // Esto seria un constructor para la estructura DevCenter, que inicializa el vector projects como un vector vacío.
    pub fn new() -> Self {
        let mut dev_center = DevCenter {
            projects: Vec::new(),
            next_id: 0,
        };

        dev_center.load_proyects_from_file();

        dev_center
    }

    fn load_proyects_from_file(&mut self) -> () {
        let data = fs::read_to_string("projects.json")
            .expect("Unable to read file");
        
        self.projects = Proyect::json_to_vec(&data);
        if let Some(max_id) = self.projects.iter().map(|p| p.id).max() {
            self.next_id = max_id + 1;
        } else {
            self.next_id = 0;
        }
    }

    fn projects_to_json(&self) -> String {
        let mut json = String::from("[");
        for (i, proyect) in self.projects.iter().enumerate() {
            if i > 0 {
                json.push(',');
            }
            json.push_str(&proyect.to_json());
        }
        json.push(']');
        json
    }

    fn find_project_index(&self, name: Option<&str>, id: Option<usize>) -> Option<usize> {
    self.projects.iter().position(|p| {
        match (name, id) {
            (Some(name), _) => p.name == name,
            (_, Some(id)) => p.id == id,
            _ => false,
        }
    })
}
    // %self indica que solo sera de lectura, ademas da a entender 'que no es una funcion estatica y puede llamarse con "." en lugar de "::"
    pub fn list_proyects(&self) -> () {
        let data = fs::read_to_string("projects.json")
            .expect("Unable to read file");

        let projects = Proyect::json_to_vec(&data);

        for project in &projects {
            println!("Name: {}", project.name);
        }

        println!("Cantidad: {}", projects.len());
    }

    // Con Option<&str> y Option<usize> se permite que los parámetros name e id sean opcionales, lo que significa que pueden ser Some(valor) o None.
    pub fn get_proyect(&self, name: Option<&str>, id: Option<usize>) -> Option<&Proyect> {
        if name.is_none() && id.is_none() {
            panic!("Tenes que pasarme un valor. asi no puedo encontrar el proyecto");
        }

        self.projects.iter().find(|p| {
            (name.is_some() && p.name == name.unwrap()) ||
            (id.is_some() && p.id == id.unwrap())
        })
    }

    pub fn create_proyect(&mut self, name: &str, script_path: Option<&str>) -> () {
        let id = self.next_id;
        self.next_id += 1;

        self.projects.push(Proyect {
            id,
            name: name.to_string(),
            script_path: script_path.map(|s| s.to_string()),
        });

        let projects_json = self.projects_to_json();

        fs::write("projects.json", projects_json).expect("Unable to write file");
        
        println!("Project '{}' created.", name);
    }

    pub fn delete_proyect(&mut self, name: Option<&str>, id: Option<usize>) {

        let index = self.find_project_index(name, id);

        match index {
            Some(index) => {
                let removed = self.projects.remove(index);

                let projects_json = self.projects_to_json();

                fs::write("projects.json", projects_json)
                    .expect("Unable to write file");

                println!("Proyecto eliminado: {}", removed.name);
            }

            None => {
                println!("No se encontró el proyecto");
            }
        }
    }

    pub fn execute_proyect(&self, name: &str) -> () {
        println!("Executing project: {}", name);
    }       
}
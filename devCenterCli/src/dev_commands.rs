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

pub struct Proyect {
    id: usize,
    name: String,
    // Add more fields as needed
}

pub struct DevCenter {
    projects: Vec<Proyect>,
    next_id: usize,
}

impl Proyect {
    pub fn to_json(&self) -> String {
        format!(r#"{{"id": {}, "name": "{}"}}"#, self.id, self.name)
    }

    pub fn json_to_vec(json: &str) -> Vec<Proyect> {
        let mut projects = Vec::new();
        let json = json.trim_matches(|c| c == '[' || c == ']');
        for item in json.split("},") {
            println!("Parsing item: {}", item);
            let item = item.trim_matches(|c| c == '{' || c == '}');
            let mut id = 0;
            let mut name = String::new();
            for pair in item.split(',') {
                let mut kv = pair.split(':');
                let key = kv.next().unwrap().trim_matches('"').trim();
                let value = kv.next().unwrap().trim_matches('"').trim();
                match key {
                    "id" => id = value.parse().unwrap(),
                    "name" => name = value.to_string(),
                    _ => (),
                }
            }
            projects.push(Proyect { id, name });
        }
        projects
    }
}

impl DevCenter {

    // Esto seria un constructor para la estructura DevCenter, que inicializa el vector projects como un vector vacío.
    pub fn new() -> Self {
        DevCenter {
            projects: Vec::new(),
            next_id: 0,
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

    // %self indica que solo sera de lectura, ademas da a entender que no es una funcion estatica y puede llamarse con "." en lugar de "::"
    pub fn list_proyects(&self) -> () {
        let data = fs::read_to_string("projects.json")
            .expect("Unable to read file");
        
        println!("{}", data);

        let projects = Proyect::json_to_vec(&data);

        println!("Cantidad: {}", projects.len());
    }

    pub fn create_proyect(&mut self, name: &str) -> () {
        let id = self.next_id;
        self.next_id += 1;

        self.projects.push(Proyect {
            id,
            name: name.to_string(),
        });

        let projects_json = self.projects_to_json();

        fs::write("projects.json", projects_json).expect("Unable to write file");
        
        println!("Project '{}' created.", name);
    }

    pub fn delete_proyect(&mut self, name: &str) -> () {
        println!("Deleting project: {}", name);
    }

    pub fn execute_proyect(&self, name: &str) -> () {
        println!("Executing project: {}", name);
    }
}
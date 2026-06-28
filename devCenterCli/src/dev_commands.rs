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

use std::process::Command;
use crate::proyect::Proyect;
use std::path::PathBuf;
use std::{env, fs};

pub struct DevCenter {
    projects: Vec<Proyect>,
    next_id: usize,
}

impl DevCenter {

    // Esto seria un constructor para la estructura DevCenter, que inicializa el vector projects como un vector vacío.
    pub fn new() -> Self {
        Self::initialize_storage();

        let mut dev_center = DevCenter {
            projects: Vec::new(),
            next_id: 0,
        };

        dev_center.load_proyects_from_file();

        dev_center
    }

    fn save_projects(&self) {
        let json = serde_json::to_string_pretty(&self.projects)
            .expect("No se pudo serializar");

        fs::write(Self::get_projects_file(), json)
            .expect("No se pudo escribir el archivo");
    }

    fn initialize_storage() {
        let devcenter_dir = Self::get_devcenter_dir();

        fs::create_dir_all(
            devcenter_dir.join("scripts")
        ).expect("No se pudo crear la carpeta scripts");

        let projects_file = Self::get_projects_file();
        if !projects_file.exists() {
            fs::write(projects_file, "[]").expect("No se pudo crear el archivo projects.json");
        }
    }

    fn load_proyects_from_file(&mut self) -> () {
        let data = fs::read_to_string(Self::get_projects_file())
            .expect("Unable to read file");
        
        self.projects = serde_json::from_str(&data)
            .expect("No se pudo leer el JSON");
        
        if let Some(max_id) = self.projects.iter().map(|p| p.id).max() {
            self.next_id = max_id + 1;
        } else {
            self.next_id = 0;
        }
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

    /**
     * Busca el directorio de configuración de DevCenter en el sistema del usuario.
     * DevCenter utiliza este directorio para almacenar archivos de configuración y datos relacionados con los proyectos
     */
    fn get_devcenter_dir() -> PathBuf {
        let home = env::var("HOME").expect("No se pudo obtener Home");

        PathBuf::from(home).join(".devcenter")
    }

    fn get_projects_file() -> PathBuf {
        Self::get_devcenter_dir().join("projects.json")
    }

    // %self indica que solo sera de lectura, ademas da a entender 'que no es una funcion estatica y puede llamarse con "." en lugar de "::"
    pub fn list_proyects(&self) {
        if self.projects.is_empty() {
            println!("No hay proyectos registrados.");
            return;
        }

        for project in &self.projects {
            println!("Name: {}", project.name);
        }

        println!("Cantidad: {}", self.projects.len());
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

        let script_path = match script_path {
            Some(path) => {
                let destination = Self::get_devcenter_dir()
                    .join("scripts")
                    .join(format!("{}_script.sh", name));
                
                println!("Copiando desde: {}", path);
                println!("Hacia: {:?}", destination);

                fs::copy(path, &destination).expect("No se pudo copiar el script");

                Some(destination.to_string_lossy().to_string())
            }

            None => None,
        };

        self.projects.push(Proyect {
            id,
            name: name.to_string(),
            script_path,
        });

        self.save_projects();

        println!("Project '{}' created.", name);
    }

    pub fn update_name(&mut self, id: usize, name: &str) {
        if let Some(index) = self.find_project_index(None, Some(id)) {
            self.projects[index].name = name.to_string();
            self.save_projects();
            println!("Nombre actualizado.");
        } else {
            println!("Proyecto no encontrado.");
        }
    }

    pub fn update_script(&mut self, id: usize, script: &str) {
        if let Some(index) = self.find_project_index(None, Some(id)) {
            self.projects[index].script_path = Some(script.to_string());
            self.save_projects();
            println!("Script actualizado.");
        } else {
            println!("Proyecto no encontrado.");
        }
    }

    pub fn delete_proyect(&mut self, name: Option<&str>, id: Option<usize>) {

        let index = self.find_project_index(name, id);

        match index {
            Some(index) => {
                let removed = self.projects.remove(index);

                if let Some(path) = &removed.script_path {
                    let _ = fs::remove_file(path);
                }
                self.save_projects();
                println!("Proyecto eliminado: {}", removed.name);
            }

            None => {
                println!("No se encontró el proyecto");
            }
        }
    }

    pub fn execute_proyect(&self, name: Option<&str>, id: Option<usize>) -> () {
        let project = self.get_proyect(name, id);
        
        match project {
            Some(project) => {
                match &project.script_path {
                    Some(path) => {
                        let result = Command::new("bash")
                            .arg(path)
                            .spawn();

                        match result {
                            Ok(__) => println!("Proyecto '{}' ejecutado correctamente.", project.name),
                            Err(e) => println!("Error al ejecutar el proyecto '{}': {}", project.name, e),
                        }
                    }
                    None => {
                        println!("El proyecto '{}' no tiene una ruta de script especificada.", project.name);
                    }

                }
            }
            None => {
                println!("No se encontró el proyecto {:?}.", name);
            }
        }
    }       
}
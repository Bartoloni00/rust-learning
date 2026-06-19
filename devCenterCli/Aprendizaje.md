# Notas de aprendizaje Rust

## Convenciones

- Los nombres de variables y funciones utilizan `snake_case`.

```rust
fn create_project() {}
let project_name = "Broker";
```

- Los nombres de structs, enums y traits utilizan `PascalCase`.

```rust
struct Project {}
struct DevCenter {}
enum Command {}
```

- Si no seguís estas convenciones el compilador generalmente no falla, pero muestra warnings.

---

## Estructura del proyecto

### target/

- Es equivalente a `vendor/` de Composer o `node_modules/` en Node.js.
- Contiene artefactos compilados, dependencias compiladas y cachés.
- No debe subirse a Git.
- Se agrega al `.gitignore`.

```gitignore
target/
```

---

## Módulos

Para importar un módulo propio:

```rust
mod dev_commands;
```

Esto le dice al compilador que existe:

```text
src/
├── main.rs
└── dev_commands.rs
```

---

## use

Permite traer elementos a nuestro scope actual.

```rust
use std::env;
```

En lugar de escribir:

```rust
std::env::args()
```

podemos escribir:

```rust
env::args()
```

---

## Argumentos de línea de comandos

Podemos acceder a los argumentos usando:

```rust
use std::env;

let args: Vec<String> = env::args().collect();
```

Ejemplo:

```bash
cargo run add Broker
```

Produce:

```rust
[
    "target/debug/devCenterCli",
    "add",
    "Broker"
]
```

Posiciones:

```rust
args[0] // ejecutable
args[1] // comando
args[2] // parámetro
```

---

## Vec

Un `Vec<T>` es un vector dinámico.

Se parece a un array de PHP o JavaScript, pero puede crecer dinámicamente.

```rust
let mut projects: Vec<Project> = Vec::new();
```

Agregar elementos:

```rust
projects.push(project);
```

---

## mut

Las variables son inmutables por defecto.

Esto NO funciona:

```rust
let projects = Vec::new();

projects.push(project);
```

Porque estamos intentando modificarla.

Debemos usar:

```rust
let mut projects = Vec::new();
```

---

## Struct

Equivalente aproximado a una clase con propiedades.

```rust
pub struct Project {
    name: String,
}
```

Crear una instancia:

```rust
let project = Project {
    name: String::from("Broker"),
};
```

---

## impl

Permite definir métodos para un struct.

```rust
impl DevCenter {
    pub fn create_project(&mut self, name: &str) {
        // ...
    }
}
```

---

## self

Es similar a `this` en PHP o JavaScript.

```rust
self.projects.push(...)
```

Significa:

> usar la propiedad `projects` de esta instancia de `DevCenter`.

---

## &self

Permite leer datos de la instancia.

```rust
pub fn list_projects(&self)
```

La función puede consultar información pero no modificarla.

---

## &mut self

Permite modificar datos de la instancia.

```rust
pub fn create_project(&mut self, name: &str)
```

Necesitamos esto porque:

```rust
self.projects.push(...)
```

modifica el vector.

---

## Función vs Método

### Función asociada

No recibe `self`.

```rust
impl DevCenter {
    pub fn new() -> Self {
        Self {
            projects: Vec::new(),
        }
    }
}
```

Se invoca así:

```rust
DevCenter::new()
```

---

### Método

Recibe `self`.

```rust
pub fn create_project(&mut self, name: &str)
```

Se invoca así:

```rust
dev_center.create_project("Broker");
```

---

## Option

Rust evita los `null`.

Cuando un valor puede existir o no existir se utiliza:

```rust
Option<T>
```

Ejemplo:

```rust
let command = args.get(1);
```

Retorna:

```rust
Some("add")
```

o

```rust
None
```

---

## match

Permite manejar distintos casos.

```rust
match command {
    Some(cmd) if cmd == "add" => {
        println!("Agregar proyecto");
    }
    _ => {
        println!("Comando desconocido");
    }
}
```

Es uno de los mecanismos más importantes de Rust.

---

## Ownership (primer contacto)

Rust siempre necesita saber quién es el dueño de un dato.

Por ejemplo:

```rust
let mut projects = Vec::new();

projects.push(Project {
    name: String::from("Broker"),
});
```

Después del `push`, el vector pasa a ser dueño del proyecto.

No existen variables globales mutables libres como en otros lenguajes.

Rust obliga a definir claramente quién posee cada dato.

---

## Borrowing (primer contacto)

En lugar de copiar datos constantemente, Rust permite prestarlos.

```rust
pub fn create_project(&mut self, name: &str)
```

- `&str` es una referencia a un string.
- No se copia el string.
- Se presta temporalmente.

Otro ejemplo:

```rust
for project in &self.projects {
    println!("{}", project.name);
}
```

- `&self.projects` presta el vector para lectura.
- No mueve los proyectos fuera del vector.

---

## Warnings vs Errors

### Warning

El programa compila.

Ejemplo:

```text
warning: field `name` is never read
```

Significa que declaraste algo pero todavía no lo utilizaste.

### Error

El programa no compila.

Ejemplo:

```text
error[E0599]: no method named `list_projects`
```

Hay que corregirlo antes de poder ejecutar el programa.
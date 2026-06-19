# Mini Neofetch en Rust - Apuntes de Aprendizaje

## 1. Biblioteca estándar (`std`)

Rust incluye una biblioteca estándar llamada `std` (Standard Library).

Ejemplos de módulos:

```rust
use std::fs;
use std::process::Command;
```

### ¿Qué significan?

- `std::fs` → manejo de archivos y directorios.
- `std::process` → ejecución de procesos del sistema.
- `std::io` → entrada y salida.
- `std::thread` → hilos.
- `std::net` → networking.

### ¿Qué hace `use`?

Permite evitar escribir rutas completas.

Sin `use`:

```rust
std::fs::read_to_string(...)
```

Con `use`:

```rust
fs::read_to_string(...)
```

---

# 2. Funciones

Definición:

```rust
fn get_hostname() -> String {
    ...
}
```

## Componentes

```rust
fn
```

Indica que estamos declarando una función.

```rust
get_hostname
```

Nombre de la función.

```rust
()
```

Parámetros de entrada.

```rust
-> String
```

Tipo de retorno.

---

# 3. Tipado estático

Rust exige conocer el tipo de cada dato.

Ejemplos:

```rust
let numero: i32 = 10;
let texto: String = "Hola".to_string();
```

En el proyecto:

```rust
fn get_hostname() -> String
```

La función devuelve un `String`.

---

# 4. Leer archivos

```rust
fs::read_to_string("/etc/hostname")
```

Lee un archivo completo y devuelve su contenido.

Ejemplo:

Archivo:

```text
archlinux
```

Resultado:

```rust
"archlinux\n"
```

---

# 5. `Result<T, E>`

Muchas operaciones pueden fallar.

Rust obliga a manejar errores.

Por ejemplo:

```rust
fs::read_to_string("/etc/hostname")
```

devuelve:

```rust
Result<String, Error>
```

Puede ser:

```rust
Ok("archlinux")
```

o

```rust
Err(error)
```

---

# 6. `unwrap()`

```rust
.unwrap()
```

Extrae el valor exitoso de un `Result`.

Internamente es parecido a:

```rust
match resultado {
    Ok(valor) => valor,
    Err(error) => panic!()
}
```

Si ocurre un error:

```text
thread 'main' panicked
```

y el programa termina.

---

# 7. `trim()`

```rust
.trim()
```

Elimina:

- espacios
- tabs
- saltos de línea

Ejemplo:

```rust
"archlinux\n".trim()
```

Resultado:

```rust
"archlinux"
```

---

# 8. `String` vs `&str`

## `String`

Texto propietario.

```rust
let nombre = String::from("Abraham");
```

Puede crecer y modificarse.

---

## `&str`

Referencia a texto.

```rust
let nombre = "Abraham";
```

Es más liviano.

---

Conversión:

```rust
.to_string()
```

Convierte:

```rust
&str
```

a

```rust
String
```

---

# 9. Ejecutar comandos del sistema

```rust
Command::new("uname")
```

Permite ejecutar programas externos.

Ejemplo:

```rust
Command::new("uname")
    .arg("-r")
    .output()
```

Equivale a:

```bash
uname -r
```

---

# 10. `output()`

```rust
.output()
```

Ejecuta el comando y devuelve:

```rust
Output
```

Contiene:

```rust
stdout
stderr
status
```

---

Ejemplo:

```bash
uname -r
```

Resultado:

```text
6.15.0-arch1
```

queda almacenado en:

```rust
output.stdout
```

---

# 11. ¿Qué es `Vec<u8>`?

La salida de un proceso no es texto.

Es una secuencia de bytes:

```rust
Vec<u8>
```

Ejemplo:

```rust
[72, 101, 108, 108, 111]
```

Representa:

```text
Hello
```

---

# 12. Referencias (`&`)

```rust
&output.stdout
```

El símbolo:

```rust
&
```

significa referencia.

También se lo llama:

- préstamo
- borrow

La idea es:

> "Usá este valor, pero no te lo lleves."

---

Ejemplo:

```rust
let nombre = String::from("Abraham");

imprimir(&nombre);
```

La función recibe una referencia y no consume la variable.

---

# 13. Ownership (primera aproximación)

Rust tiene una regla fundamental:

> Cada valor tiene un único dueño.

Ejemplo:

```rust
let a = String::from("hola");
let b = a;
```

Ahora:

```rust
a
```

ya no es válido.

La propiedad pasó a:

```rust
b
```

---

Para evitarlo:

```rust
let a = String::from("hola");

let b = &a;
```

Ahora ambos pueden usar el dato.

---

# 14. `from_utf8_lossy()`

```rust
String::from_utf8_lossy(...)
```

Convierte bytes en texto.

Ejemplo:

```rust
[72,101,108,108,111]
```

Resultado:

```text
Hello
```

---

¿Por qué se llama "lossy"?

Porque si encuentra bytes inválidos:

```rust
[255,255]
```

los reemplaza por:

```text
�
```

en lugar de fallar.

---

# 15. Macros

Rust tiene macros.

Ejemplos:

```rust
println!()
vec!()
format!()
panic!()
```

Se identifican por:

```rust
!
```

---

## `println!`

```rust
println!("Hola {}", nombre);
```

Imprime texto en consola.

El `{}` es un placeholder.

Ejemplo:

```rust
let nombre = "Abraham";

println!("Hola {}", nombre);
```

Salida:

```text
Hola Abraham
```

---

# 16. Código actual

```rust
use std::fs;
use std::process::Command;

fn get_hostname() -> String {
    fs::read_to_string("/etc/hostname")
        .unwrap()
        .trim()
        .to_string()
}

fn get_kernel() -> String {
    let output = Command::new("uname")
        .arg("-r")
        .output()
        .unwrap();

    String::from_utf8_lossy(&output.stdout)
        .trim()
        .to_string()
}

fn main() {
    println!("Hostname: {}", get_hostname());
    println!("Kernel: {}", get_kernel());
}
```

---

# Conceptos aprendidos hasta ahora

- Biblioteca estándar (`std`)
- Imports (`use`)
- Funciones (`fn`)
- Tipado estático
- Tipos de retorno (`->`)
- Lectura de archivos
- `Result<T,E>`
- `unwrap()`
- `String`
- `&str`
- `trim()`
- `to_string()`
- Ejecución de procesos
- `Output`
- `Vec<u8>`
- Referencias (`&`)
- Ownership (introducción)
- `from_utf8_lossy()`
- Macros (`println!`)

---

# Próximo objetivo

Agregar al mini Neofetch:

- CPU (`/proc/cpuinfo`)
- RAM (`/proc/meminfo`)
- Uptime (`/proc/uptime`)
- Distribución Linux (`/etc/os-release`)

Y comenzar a dividir el código en módulos:

```text
src/
├── main.rs
└── system.rs
```

para aprender organización de proyectos en Rust.
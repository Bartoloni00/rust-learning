# Aprendizaje Rust - DevCenter CLI

Durante esta etapa del desarrollo del DevCenter CLI continué construyendo una aplicación de consola para administrar proyectos utilizando Rust. El objetivo principal fue dejar de trabajar únicamente con estructuras en memoria y empezar a implementar persistencia real de información, separación de responsabilidades y manejo de datos de una forma más cercana a una aplicación real.

El primer paso fue crear la funcionalidad para agregar proyectos. Para esto se creó la estructura `Proyect`, que representa un proyecto dentro del sistema, con campos como `id` y `name`. La función `create_proyect` se encarga de generar un nuevo proyecto, asignarle un identificador único, agregarlo al vector interno de proyectos (`Vec<Proyect>`) y finalmente guardar los cambios en un archivo JSON.

Al principio el problema principal era que los datos existían únicamente mientras el programa estaba ejecutándose. Al cerrar la aplicación toda la información se perdía porque el vector estaba almacenado solamente en memoria. Para solucionar esto se agregó persistencia utilizando el módulo nativo `std::fs`, sin utilizar dependencias externas. La aplicación ahora puede leer un archivo `projects.json`, cargar los proyectos existentes, modificarlos y volver a guardar el estado actualizado.

Una parte importante del aprendizaje fue entender que escribir un archivo no significa agregar información automáticamente. La función `fs::write` reemplaza el contenido completo del archivo. Por esto la estrategia utilizada fue reconstruir todo el JSON desde el estado actual del vector en memoria y luego sobrescribir el archivo. El flujo terminó siendo: leer JSON, convertirlo en estructuras Rust, modificar el `Vec<Proyect>`, convertir nuevamente a JSON y guardar el archivo actualizado.

También se trabajó en la separación del código en módulos. Inicialmente todo el proyecto estaba dentro de un único archivo, pero al crecer la aplicación empezó a ser más difícil de mantener. Se separó la lógica en diferentes archivos: `main.rs` quedó encargado de manejar los argumentos de consola y decidir qué comando ejecutar, `dev_commands.rs` contiene la lógica del sistema y `proyect.rs` contiene la estructura del proyecto junto con las funciones relacionadas a convertir información entre Rust y JSON.

Para conectar estos módulos se utilizó el sistema de módulos de Rust mediante `mod` y `use`. Esto permitió importar estructuras definidas en otros archivos y comenzar a organizar el proyecto de una forma más cercana a una aplicación real.

Luego se implementó la funcionalidad para obtener proyectos mediante el comando `get`. La idea fue permitir buscar un proyecto tanto por nombre como por identificador. Para esto se utilizó `Option`, permitiendo que una búsqueda pueda recibir un nombre (`Some("sensor")`), un id (`Some(2)`) o indicar que uno de los valores no existe (`None`).

En el lado del CLI se aprendió a interpretar los argumentos dinámicamente. El tercer argumento recibido por consola (`args[2]`) puede representar diferentes tipos de datos. Para resolver esto se intentó convertir el valor recibido a un número usando `parse::<usize>()`. Si la conversión funciona significa que el usuario ingresó un id; si falla significa que probablemente ingresó un nombre. De esta forma un mismo comando puede soportar:

`cargo run get sensor`

o:

`cargo run get 2`

sin necesidad de crear comandos separados.

Durante la implementación del `remove` apareció un concepto importante relacionado con los vectores en Rust. Para eliminar un proyecto primero es necesario encontrar dónde se encuentra dentro del `Vec<Proyect>`. Para esto se utilizó `position()`, que recorre el vector y devuelve el índice donde se encontró el elemento.

Es importante diferenciar el índice del vector del id del proyecto. El índice representa la posición física del elemento dentro de la lista, mientras que el id es un valor definido por la aplicación. Pueden coincidir, pero no necesariamente representan lo mismo.

Por ejemplo, si el vector contiene:

0 -> proyecto A  
1 -> proyecto B  
2 -> proyecto C  

el índice de proyecto B es 1. Si se utiliza `remove(1)`, Rust elimina el segundo elemento del vector.

La eliminación ocurre primero en memoria. El vector queda modificado, pero el archivo JSON todavía contiene la información anterior. Para que el cambio sea permanente se vuelve a generar el JSON desde el vector actualizado y se vuelve a escribir el archivo. Por lo tanto, no se elimina una línea específica del archivo, sino que se reconstruye todo el archivo a partir del nuevo estado de la aplicación.

También se comenzó a comprender la complejidad de las operaciones realizadas. La búsqueda utilizando `find()` o `position()` sobre un `Vec` es una búsqueda lineal, con complejidad O(n), porque en el peor caso debe recorrer todos los elementos hasta encontrar el resultado o determinar que no existe. Para una aplicación pequeña como esta, utilizar un `Vec` es una solución correcta y simple. En sistemas con millones de elementos podría ser más conveniente utilizar estructuras como `HashMap`, que permiten búsquedas más rápidas por clave.

Otro aprendizaje importante fue comprender mejor el manejo de errores y valores opcionales en Rust. En lugar de asumir que un valor siempre existe, Rust obliga a expresar la posibilidad de ausencia mediante tipos como `Option<T>`. Esto cambia la forma de pensar respecto a otros lenguajes donde muchas veces se utilizan valores nulos.

Al finalizar esta etapa el DevCenter CLI ya cuenta con una base funcional: puede crear proyectos, persistir información, listar proyectos existentes, obtener proyectos por nombre o id, eliminar proyectos y mantener la información separada en módulos.

## Conceptos de Rust practicados

- Structs.
- Implementación de métodos mediante `impl`.
- Ownership y borrowing.
- Referencias inmutables (`&`) y mutables (`&mut`).
- Uso de `Vec<T>`.
- Iteradores (`iter`).
- Búsqueda con `find()` y `position()`.
- Eliminación de elementos con `remove()`.
- Uso de `Option<T>`.
- Uso de `Result<T, E>`.
- Manejo de errores con `expect`.
- Pattern matching con `match`.
- Conversión de tipos con `parse()`.
- Lectura y escritura de archivos con `std::fs`.
- Módulos con `mod`.
- Importación de código con `use`.
- Separación de responsabilidades entre archivos.
- Manejo de argumentos de consola con `std::env`.
- Conversión manual entre estructuras Rust y JSON.
- Persistencia de información.
- Complejidad algorítmica básica aplicada a estructuras de datos.
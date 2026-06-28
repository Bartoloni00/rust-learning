use serde::{Serialize, Deserialize};

//Los atributos con #[] son atributos del compilador.
//Antes de compilar este struct, generame automáticamente código."
#[derive(Serialize, Deserialize, Debug)]
pub struct Proyect {
    pub id: usize,
    pub name: String,
    pub script_path: Option<String>,
}
pub struct Proyect {
    pub id: usize,
    pub name: String,
    pub script_path: Option<String>,
}


impl Proyect {
    pub fn to_json(&self) -> String {
        format!(
            r#"{{"id": {}, "name": "{}", "script_path": "{}"}}"#,
            self.id,
            self.name,
            self.script_path.as_deref().unwrap_or("")
        )
    }

    pub fn json_to_vec(json: &str) -> Vec<Proyect> {
        let mut projects = Vec::new();
        let json = json.trim_matches(|c| c == '[' || c == ']');
        if json.trim().is_empty() {
            return Vec::new();
        }
        for item in json.split("},") {
            let item = item.trim_matches(|c| c == '{' || c == '}');
            let mut id = 0;
            let mut name = String::new();
            let mut script_path = None;
            
            for pair in item.split(',') {
                let mut kv = pair.split(':');
                // trim() debe ejecutarse antes que trim_matches('"')
                // para que las comillas queden en los extremos y puedan eliminarse.
                let key = kv.next().unwrap().trim().trim_matches('"');
                let value = kv.next().unwrap().trim().trim_matches('"');

                match key {
                    "id" => id = value.parse().unwrap(),
                    "name" => name = value.to_string(),
                    "script_path" => {
                        script_path = Some(value.to_string());
                    }
                    _ => (),
                }
            }
            projects.push(Proyect { id, name, script_path });
        }
        projects
    }
}
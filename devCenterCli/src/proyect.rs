pub struct Proyect {
    pub id: usize,
    pub name: String,
    // Add more fields as needed
}


impl Proyect {
    pub fn to_json(&self) -> String {
        format!(r#"{{"id": {}, "name": "{}"}}"#, self.id, self.name)
    }

    pub fn json_to_vec(json: &str) -> Vec<Proyect> {
        let mut projects = Vec::new();
        let json = json.trim_matches(|c| c == '[' || c == ']');
        for item in json.split("},") {
            let item = item.trim_matches(|c| c == '{' || c == '}');
            let mut id = 0;
            let mut name = String::new();
            for pair in item.split(',') {
                let mut kv = pair.split(':');
                // trim() debe ejecutarse antes que trim_matches('"')
                // para que las comillas queden en los extremos y puedan eliminarse.
                let key = kv.next().unwrap().trim().trim_matches('"');
                let value = kv.next().unwrap().trim().trim_matches('"');
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
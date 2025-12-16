use std::{fs, io};
pub struct EnvFile {
    raw: String
}

impl EnvFile {
    pub fn load_or_new(path: &str) -> io::Result<EnvFile> {
        let res = fs::read_to_string(path);
        match res {
            Ok(raw) => Ok(Self{ raw: raw }),
            Err(e) => match e.kind() {
                io::ErrorKind::NotFound => Ok(Self{ raw: String::new() }),
                _ => Err(e)
            }
        }
    }

    pub fn load(path: &str) -> io::Result<EnvFile> {
        let res = fs::read_to_string(path)?;
        Ok(Self {
            raw: res
        })
    }

    pub fn write(&self, path: &str) -> io::Result<()> {
        fs::write(path, self.raw.clone())
    }

    pub fn get_line(&self, prefix: &str) -> Option<String> {
        match self.raw.split('\n').find(|&v| v.starts_with(prefix)) {
            Some(line) => Some(String::from(line)),
            None => None
        }
    }

    pub fn get_field(&self, name: &str) -> Option<String> {
        match self.get_line(name) {
            Some(line) => {
                let parts: Vec<&str> = line.splitn(2, '=').collect();
                let value = parts[1];
                Some(value.to_string())
            }
            None => None
        }
    }

    pub fn set_field(&mut self, name: &str, value: &str) {
        match self.get_line(name) {
            Some(line) => {
                let parts: Vec<&str> = line.splitn(2, '=').collect();
                let name = parts[0];
                let new_field = format!("{}={}", name, value);
                
                self.raw = self.raw.replace(&line, &new_field);
            }
            None => {
                self.raw.push_str(format!("\n{}={}", name, value).as_str());
            }
        }
    }

    pub fn fields(&self) -> Vec<(String, String)> {
        self.raw.split('\n')
            .filter(|&x| !x.starts_with('#') && x.contains('='))
            .map(|x| {
                let parts: Vec<&str> = x.splitn(2, '=').collect();
                let left = parts[0];
                let right = parts[1];

                (String::from(left), String::from(right))
            }).collect()
    }
}
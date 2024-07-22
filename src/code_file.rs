use std::{
    env,
    fmt,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

pub struct CodeFile {
    data_section: Vec<String>,
    text_section: Vec<String>,
}

impl CodeFile {
    pub fn new() -> Result<CodeFile, io::Error> {
        let lines = Self::get_file_contents()?;
        let (data_section, text_section) = Self::get_data_text_sections(&lines);
        Ok(CodeFile {
            data_section,
            text_section,
        })
    }

    fn get_filename() -> Result<String, &'static str> {
        let args: Vec<String> = env::args().collect();
        if args.len() < 2 {
            Err("No filename provided")
        } else {
            Ok(args[1].trim().to_string())
        }
    }

    fn get_file_contents() -> Result<Vec<String>, io::Error> {
        let file_name = Self::get_filename().map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
        let file_path = Path::new(&file_name);
        let file = File::open(&file_path)?;
        let reader = BufReader::new(file);
        reader.lines().collect()
    }

    fn get_data_text_sections(lines: &[String]) -> (Vec<String>, Vec<String>) {
        let mut data_section = Vec::new();
        let mut text_section = Vec::new();
        let mut in_data = false;
        let mut in_text = false;

        for line in lines {
            match line.as_str() {
                ".data" => {
                    in_data = true;
                    in_text = false;
                }
                ".text" => {
                    in_text = true;
                    in_data = false;
                }
                _ => {
                    if in_data {
                        data_section.push(line.clone());
                    } else if in_text {
                        text_section.push(line.clone());
                    }
                }
            }
        }

        (data_section, text_section)
    }
}

impl fmt::Display for CodeFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Data Section:")?;
        for line in &self.data_section {
            writeln!(f, "{}", line)?;
        }
        
        writeln!(f, "\nText Section:")?;
        for line in &self.text_section {
            writeln!(f, "{}", line)?;
        }

        Ok(())
    }
}
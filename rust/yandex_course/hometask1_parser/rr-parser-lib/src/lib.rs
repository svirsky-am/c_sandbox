use serde::{Deserialize, Serialize};
use serde_json::Map;
use std::fs;
use std::path::Path;

/// Supported input/output formats
#[derive(Debug, Clone, PartialEq)]
pub enum Format {
    Json,
    Yaml,
    Toml,
}

impl std::str::FromStr for Format {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(Format::Json),
            "yaml" | "yml" => Ok(Format::Yaml),
            "toml" => Ok(Format::Toml),
            _ => Err(format!("Unsupported format: {}", s)),
        }
    }
}

// /// Generic data container (simplified for demo)
// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct Data {
//     pub content: HashMap<String, serde_json::Value>,
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Data {
    pub content: Map<String, serde_json::Value>, // ✅ Use Map instead of HashMap
}

/// Parse input data based on format
pub fn parse_input(input: &str, format: &Format) -> Result<Data, Box<dyn std::error::Error>> {
    match format {
        Format::Json => {
            let value: serde_json::Value = serde_json::from_str(input)?;
            let content = match value {
                serde_json::Value::Object(map) => map,
                _ => return Err("JSON input must be an object".into()),
            };
            Ok(Data { content })
            // Ok(Data { })
        }
        // In a real app, you'd implement YAML/TOML parsing here
        _ => Err(format!("Parsing {} not implemented in demo", format_to_str(format)).into()),
    }
}

/// Serialize data to target format
pub fn serialize_output(
    data: &Data,
    format: &Format,
) -> Result<String, Box<dyn std::error::Error>> {
    match format {
        Format::Json => {
            let mut map = serde_json::Map::new();
            for (k, v) in &data.content {
                map.insert(k.clone(), v.clone());
            }
            Ok(serde_json::to_string_pretty(&map)?)
        }
        // In a real app, you'd implement other serializers
        _ => Err(format!(
            "Serialization to {} not implemented in demo",
            format_to_str(format)
        )
        .into()),
    }
}

fn format_to_str(f: &Format) -> &'static str {
    match f {
        Format::Json => "JSON",
        Format::Yaml => "YAML",
        Format::Toml => "TOML",
    }
}

/// Reads a file and returns its contents as a String.
///
/// # Arguments
/// * `path` - Path to the file to read
///
/// # Returns
/// * `Ok(String)` containing file contents on success
/// * `Err` if file doesn't exist or can't be read
pub fn read_file(path: &Path) -> Result<String, std::io::Error> {
    fs::read_to_string(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::ErrorKind;
    use std::path::Path;

    #[test]
    fn test_read_nonexistent_file() {
        let fake_path = Path::new("/definitely/does/not/exist.txt");
        let result = read_file(fake_path);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), ErrorKind::NotFound);
    }

    // Note: Library tests avoid real files (they're implementation-agnostic)
    // Real file tests belong in the binary crate
}

// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }

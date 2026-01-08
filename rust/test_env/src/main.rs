use std::io::{Read, Write, Result};
use std::fs::File;

// In-memory representation of parsed CSV
#[derive(Debug, Default)]
struct ParsedCsv {
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
}

impl ParsedCsv {
    // Very simple CSV parser (no quotes, no escapes)
    fn parse_from_str(&mut self, input: &str) {
        let mut lines = input.lines().peekable();
        if lines.peek().is_none() {
            return;
        }

        self.headers = lines
            .next()
            .unwrap()
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();

        for line in lines {
            if line.trim().is_empty() {
                continue;
            }
            let row = line
                .split(',')
                .map(|s| s.trim().to_string())
                .collect();
            self.rows.push(row);
        }
    }

    // Convert to simple YAML format
    fn to_yaml_string(&self) -> String {
        if self.headers.is_empty() {
            return String::new();
        }

        let mut yaml = String::from("---\n");
        for row in &self.rows {
            yaml.push_str("-\n");
            for (i, value) in row.iter().enumerate() {
                if i < self.headers.len() {
                    let key = &self.headers[i];
                    // Minimal escaping: quote if it looks like a number or has special chars
                    let display_value = if value.chars().all(|c| c.is_ascii_digit()) || 
                                         value.contains([' ', ':', '{', '}', '[', ']', ',', '"']) {
                        format!("\"{}\"", value)
                    } else {
                        value.clone()
                    };
                    yaml.push_str(&format!("  {}: {}\n", key, display_value));
                }
            }
        }
        yaml
    }
}

// Main struct that implements Read and Write
pub struct CsvYamlConverter {
    // Input state
    input_buffer: String,
    parsing_done: bool,
    parsed_data: ParsedCsv,

    // Output state (for Read implementation — see note below)
    output_bytes: Vec<u8>,
    output_pos: usize,
}

impl CsvYamlConverter {
    pub fn new() -> Self {
        Self {
            input_buffer: String::new(),
            parsing_done: false,
            parsed_data: ParsedCsv::default(),
            output_bytes: Vec::new(),
            output_pos: 0,
        }
    }

    // Finalize parsing and prepare output bytes
    fn finalize(&mut self) {
        if !self.parsing_done {
            self.parsed_data.parse_from_str(&self.input_buffer);
            self.output_bytes = self.parsed_data.to_yaml_string().into_bytes();
            self.parsing_done = true;
        }
    }
}

// Implement Write: we accept CSV data via write()
impl Write for CsvYamlConverter {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        // Convert bytes to string (assume UTF-8)
        let s = std::str::from_utf8(buf)
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid UTF-8"))?;
        self.input_buffer.push_str(s);
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<()> {
        // Trigger parsing when flushed
        self.finalize();
        Ok(())
    }
}

// Implement Read: once flushed, we can read the YAML output
impl Read for CsvYamlConverter {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        if !self.parsing_done {
            // Not ready yet — force finalization
            self.finalize();
        }

        if self.output_pos >= self.output_bytes.len() {
            return Ok(0); // EOF
        }

        let len = std::cmp::min(buf.len(), self.output_bytes.len() - self.output_pos);
        buf[..len].copy_from_slice(&self.output_bytes[self.output_pos..self.output_pos + len]);
        self.output_pos += len;
        Ok(len)
    }
}

// Example usage
fn main() -> Result<()> {
    // Example CSV content as bytes
    let csv_content = "name,age,city\nAlice,30,New York\nBob,25,Los Angeles\n";

    // Create converter
    let mut converter = CsvYamlConverter::new();

    // Write CSV data into it (using Write trait)
    converter.write_all(csv_content.as_bytes())?;
    converter.flush()?; // Important: triggers parsing

    // Now read YAML output from it (using Read trait)
    let mut output_buffer = Vec::new();
    converter.read_to_end(&mut output_buffer)?;

    // Convert to string and print
    let yaml_output = String::from_utf8(output_buffer)
        .expect("YAML output should be valid UTF-8");
    println!("Generated YAML:\n{}", yaml_output);

    // Also write to file using Write trait
    let mut file = File::create("output.yaml")?;
    file.write_all(yaml_output.as_bytes())?;

    Ok(())
}
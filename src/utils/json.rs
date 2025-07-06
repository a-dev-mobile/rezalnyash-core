use serde::Serialize;
use serde_json;
use std::fs::File;
use std::io::Write;
use std::path::Path;

/// Универсальный метод для сохранения любого сериализуемого объекта в JSON
pub fn save_to_json<T, P>(
    data: &T,
    filename: P
) -> Result<(), Box<dyn std::error::Error>>
where
    T: Serialize,
    P: AsRef<Path>,
{
    let json_string = serde_json::to_string_pretty(data)?;
    let mut file = File::create(filename)?;
    file.write_all(json_string.as_bytes())?;
    Ok(())
}

/// Универсальный метод для сохранения в компактном формате JSON
pub fn save_to_json_compact<T, P>(
    data: &T,
    filename: P
) -> Result<(), Box<dyn std::error::Error>>
where
    T: Serialize,
    P: AsRef<Path>,
{
    let json_string = serde_json::to_string(data)?;
    let mut file = File::create(filename)?;
    file.write_all(json_string.as_bytes())?;
    Ok(())
}
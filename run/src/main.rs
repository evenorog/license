use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::{self, File};
use std::io::{BufWriter, Write};

#[serde(rename_all = "camelCase")]
#[derive(Debug, Serialize, Deserialize)]
struct License {
    name: String,
    license_id: String,
    license_text: String,
    standard_license_header: Option<String>,
    #[serde(default)]
    see_also: Vec<String>,
    #[serde(default)]
    is_osi_approved: bool,
    #[serde(default)]
    is_fsf_libre: bool,
    is_deprecated_license_id: bool,
}

impl License {
    fn ident(&self) -> String {
        let ident = self
            .license_id
            .replace('-', "_")
            .replace('.', "_")
            .replace('+', "_plus");
        if ident == "0BSD" {
            "BSD_0".to_string()
        } else {
            ident
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut f = BufWriter::new(File::create("../src/licenses.rs")?);
    let mut v = vec![];
    f.write_all(b"// This file is autogenerated, do not edit.\n\n")?;
    f.write_all(b"/// Returns a license based on the provided id.\n")?;
    f.write_all(b"#[inline]\n")?;
    f.write_all(b"pub fn from_id(id: &str) -> Option<&'static dyn crate::License> {\n")?;
    f.write_all(b"    match id {\n")?;
    for entry in fs::read_dir("../license-list-data/json/details")? {
        let entry = entry?;
        let rdr = File::open(entry.path())?;
        let license: License = serde_json::from_reader(rdr)?;
        writeln!(
            f,
            "        {:?} => Some(&{}),",
            license.license_id,
            license.ident()
        )?;
        v.push(license);
    }
    f.write_all(b"        _ => None,\n")?;
    f.write_all(b"    }\n")?;
    f.write_all(b"}\n\n")?;
    for license in v {
        writeln!(
            f,
            include_str!("../TEMPLATE"),
            ident = license.ident(),
            name = license.name,
            id = license.license_id,
            text = license.license_text,
            header = license.standard_license_header,
            osi = license.is_osi_approved,
            fsf = license.is_fsf_libre,
            deprecated = license.is_deprecated_license_id,
            see_also = license.see_also,
        )?;
    }
    Ok(())
}

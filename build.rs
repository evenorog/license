use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use std::fs::{self, File};
use std::io::{BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct License {
    name: String,
    license_id: String,
    license_text: String,
    standard_license_header: Option<String>,
    license_comments: Option<String>,
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
            .replace(['-', '.'], "_")
            .replace('+', "_plus");
        if ident == "0BSD" {
            "Bsd0".to_string()
        } else {
            reword::pascal_case(ident)
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Exception {
    name: String,
    license_exception_id: String,
    license_exception_text: String,
    license_comments: Option<String>,
    is_deprecated_license_id: bool,
    #[serde(default)]
    see_also: Vec<String>,
}

impl Exception {
    fn ident(&self) -> String {
        let ident = self.license_exception_id.replace(['-', '.'], "_");
        if ident == "389_exception" {
            "Exception389".to_string()
        } else {
            reword::pascal_case(ident)
        }
    }
}

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let out_dir = PathBuf::from(out_dir);
    let licenses_output = out_dir.join("licenses.rs");
    let exceptions_output = out_dir.join("exceptions.rs");

    if cfg!(feature = "offline") {
        // If unable to clone the latest version from git we use the offline files.
        build_licenses_from_json(
            Path::new("license-list-data/json/details"),
            &licenses_output,
        )
        .unwrap();
        build_exceptions_from_json(
            Path::new("license-list-data/json/exceptions"),
            &exceptions_output,
        )
        .unwrap();
    } else {
        let json_dir = Path::new(&out_dir).join("license-list-data/json");

        if json_dir.exists() {
            build_licenses_from_json(&json_dir.join("details"), &licenses_output).unwrap();
            build_exceptions_from_json(&json_dir.join("exceptions"), &exceptions_output).unwrap();
        } else {
            let status = Command::new("git")
                .arg("clone")
                .arg("--depth")
                .arg("1")
                .arg("https://github.com/spdx/license-list-data.git")
                .current_dir(&out_dir)
                .status()
                .expect("`git` not found");

            if status.success() {
                build_licenses_from_json(&json_dir.join("details"), &licenses_output).unwrap();
                build_exceptions_from_json(&json_dir.join("exceptions"), &exceptions_output)
                    .unwrap();
            } else {
                match status.code() {
                    Some(code) => panic!("`git clone --depth 1` failed with exit code `{}`", code),
                    None => panic!("`git clone --depth 1` was terminated by signal"),
                }
            }
        }
    }
}

fn build_licenses_from_json(input: &Path, output: &Path) -> Result<(), Box<dyn Error>> {
    let inner = File::create(output)?;
    let mut f = BufWriter::with_capacity(4_194_304, inner);
    let mut licenses = Vec::with_capacity(512);

    // Generate the parsing code.
    f.write_all(b"pub(crate) fn parse_id(id: &str) -> Option<&'static dyn crate::License> {\n")?;
    f.write_all(b"    match id {\n")?;
    for entry in fs::read_dir(input)? {
        let entry = entry?;
        let rdr = File::open(entry.path())?;
        let license: License = serde_json::from_reader(BufReader::with_capacity(131_072, rdr))?;
        writeln!(
            f,
            "        {:?} => Some(&{}),",
            license.license_id,
            license.ident()
        )?;
        licenses.push(license);
    }
    f.write_all(b"        _ => None,\n")?;
    f.write_all(b"    }\n")?;
    f.write_all(b"}\n\n")?;

    // Generate the license code.
    for license in licenses {
        writeln!(
            f,
            include_str!("LICENSE-TEMPLATE"),
            ident = license.ident(),
            id = license.license_id,
            name = license.name,
            text = license.license_text,
            header = license.standard_license_header,
            osi = license.is_osi_approved,
            fsf = license.is_fsf_libre,
            deprecated = license.is_deprecated_license_id,
            comments = license.license_comments,
            see_also = license.see_also,
        )?;
    }
    Ok(())
}

fn build_exceptions_from_json(input: &Path, output: &Path) -> Result<(), Box<dyn Error>> {
    let inner = File::create(output)?;
    let mut f = BufWriter::with_capacity(524_288, inner);
    let mut exceptions = Vec::with_capacity(64);

    // Generate the parsing code.
    f.write_all(b"pub(crate) fn parse_id(id: &str) -> Option<&'static dyn crate::Exception> {\n")?;
    f.write_all(b"    match id {\n")?;
    for entry in fs::read_dir(input)? {
        let entry = entry?;
        let rdr = File::open(entry.path())?;
        let exception: Exception = serde_json::from_reader(BufReader::with_capacity(131_072, rdr))?;
        writeln!(
            f,
            "        {:?} => Some(&{}),",
            exception.license_exception_id,
            exception.ident()
        )?;
        exceptions.push(exception);
    }
    f.write_all(b"        _ => None,\n")?;
    f.write_all(b"    }\n")?;
    f.write_all(b"}\n\n")?;

    // Generate the exception code.
    for exception in exceptions {
        writeln!(
            f,
            include_str!("EXCEPTION-TEMPLATE"),
            ident = exception.ident(),
            id = exception.license_exception_id,
            name = exception.name,
            text = exception.license_exception_text,
            deprecated = exception.is_deprecated_license_id,
            comments = exception.license_comments,
            see_also = exception.see_also,
        )?;
    }
    Ok(())
}

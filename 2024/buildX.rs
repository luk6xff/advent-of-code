// build.rs
use std::env;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

fn main() -> std::io::Result<()> {

    // Instruct Cargo to rerun build script if any file in src/days changes
    println!("cargo:rerun-if-changed=src/days");
    println!("cargo:rerun-if-changed=build.rs");

    let days_dir = Path::new("src/days");

    let day_files = fs::read_dir(days_dir)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.is_file() {
                if let Some(fname) = path.file_name()?.to_str() {
                    if fname.starts_with("day") && fname.ends_with(".rs") && fname != "day.rs" {
                        Some(fname.to_string())
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    let dest_path = Path::new(&out_dir).join("generated_days.rs");
    let mut f = File::create(&dest_path)?;

    // Write module declarations
    for day_file in &day_files {
        let module_name = day_file.trim_end_matches(".rs");
        // writeln!(f, "use crate::days::{}::Day{};", module_name, module_name.trim_start_matches("day"))?;
        writeln!(f, "pub mod {};", module_name)?;
    }

    // Write the registration function
    writeln!(f, "\nuse crate::days::day::Day;")?;
    writeln!(f, "use std::vec::Vec;")?;
    writeln!(f, "use std::boxed::Box;")?;
    writeln!(f, "\npub fn get_all_days() -> Vec<Box<dyn Day>> {{")?;
    writeln!(f, "    let mut days: Vec<Box<dyn Day>> = Vec::new();")?;

    for day_file in &day_files {
        let module_name = day_file.trim_end_matches(".rs");
        let struct_name = format!("Day{}", module_name.trim_start_matches("day"));
        writeln!(
            f,
            "    days.push(Box::new({}::{} {{}}));",
            module_name, struct_name
        )?;
    }

    writeln!(f, "    days")?;
    writeln!(f, "}}")?;

    println!(
        "cargo:rustc-env=GENERATED_DAYS_PATH={}",
        dest_path.display()
    );

    Ok(())
}

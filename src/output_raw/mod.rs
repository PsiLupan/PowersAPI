use crate::structs::config::{OutputStyleConfig, PowersConfig};
use crate::structs::*;
use std::collections::HashSet;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::io::{Error, ErrorKind};
use std::path::Path;

/// Default extension for the .json files.
const JSON_EXT: &'static str = ".json";

/// Begins the process of writing the entire powers dictionary to disk as .json files.
///
/// # Arguments:
///
/// * `powers_dict` - A `PowersDictionary` containing a hierarchy of categories, power sets, and powers.
/// * `config` - Configuration information.
///
/// # Returns:
///
/// Nothing if the operation was successful. Otherwise, an `io::Error` containing the error information.
///
/// # Notes:
///
/// The data is written as a hierarchy of individual .json files stored in folders. The output paths and
/// file names are dependent on the source files indicated in the bins, so they probably won't appear to
/// have any rhyme or reason on disk.
///
/// `http://myserver/powers/tanker-melee/super-strength/`
pub fn write_powers_dictionary(
    powers_dict: PowersDictionary,
    config: &PowersConfig,
) -> io::Result<()> {
    // setup the output directory
    let output_path = Path::new(&config.output_path);
    fs::create_dir_all(&output_path)?;
    if output_path.read_dir()?.count() > 0 {
        print!(
            "WARNING! The output path {} is not empty. Overwrite? (y/n)",
            output_path.display()
        );
        io::stdout().flush()?;
        //TODO: better input handling
        for c in io::stdin().lock().bytes() {
            match c? {
                b'y' | b'Y' => break,
                b'n' | b'N' => return Err(Error::from(ErrorKind::Interrupted)),
                _ => (),
            }
        }
        println!();
    }

    // write powers
    let mut fx_cache = HashSet::new();
    let mut file_count = 0;
    for power_cat in powers_dict.power_categories.iter().map(|p| p.borrow()) {
        if power_cat.include_in_output {
            write_power_category(&*power_cat, config)?;
            file_count += 1;
            for power_set in power_cat.pp_power_sets.iter().map(|p| p.borrow()) {
                if power_set.include_in_output {
                    write_power_set(&*power_set, config)?;
                    file_count += 1;
                    let powers: Vec<_> = power_set
                        .pp_powers
                        .iter()
                        .filter(|p| p.borrow().include_in_output)
                        .collect();
                    if powers.len() > 0 {
                        // write all powers in the power set
                        write_powers(&powers, config)?;
                        file_count += 1;

                        // write all the FX blocks, checking for duplicates
                        for p in powers.iter().map(|p| p.borrow()) {
                            if let Some(fx) = &p.p_fx {
                                if let Some(source) = &fx.pch_source_file {
                                    let source = source.to_lowercase();
                                    if !fx_cache.contains(&source) {
                                        fx_cache.insert(source);
                                        write_fx(fx, config)?;
                                        file_count += 1;
                                    }
                                }
                            }
                            for cfx in &p.pp_custom_fx {
                                if let Some(custom_fx) = &cfx.p_fx {
                                    if let Some(source) = &custom_fx.pch_source_file {
                                        let source = source.to_lowercase();
                                        if !fx_cache.contains(&source) {
                                            fx_cache.insert(source);
                                            write_fx(custom_fx, config)?;
                                            file_count += 1;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // write archetypes -
    // the original has everything in one def file, but that results in a massive unwieldy
    // file because of all the computed tables that end up in the bin
    for archetype in powers_dict.archetypes.values() {
        write_archetype(&*archetype.borrow(), config)?;
        file_count += 1;
    }

    // write attribute names
    write_attrib_names(&powers_dict.attrib_names, config)?;
    file_count += 1;

    println!("{} output files written.", file_count);

    Ok(())
}

fn write_power_category(power_cat: &PowerCategory, config: &PowersConfig) -> io::Result<()> {
    let output_file = config.join_to_output_path(
        format!(
            "{}{}",
            power_cat.pch_source_file.as_ref().unwrap().to_lowercase(),
            JSON_EXT
        )
        .as_str(),
    );
    println!("Writing: {} ...", output_file.display());
    ensure_path_exists(&output_file)?;
    let mut f = fs::File::create(&output_file)?;
    match config.output_style {
        OutputStyleConfig::Pretty => serde_json::to_writer_pretty(&mut f, power_cat)?,
        OutputStyleConfig::Compact => serde_json::to_writer(&mut f, power_cat)?,
    }
    Ok(())
}

fn write_power_set(power_set: &BasePowerSet, config: &PowersConfig) -> io::Result<()> {
    let output_file = config.join_to_output_path(
        format!(
            "{}{}",
            power_set.pch_source_file.as_ref().unwrap().to_lowercase(),
            JSON_EXT
        )
        .as_str(),
    );
    println!("\tWriting: {} ...", output_file.display());
    ensure_path_exists(&output_file)?;
    let mut f = fs::File::create(&output_file)?;
    match config.output_style {
        OutputStyleConfig::Pretty => serde_json::to_writer_pretty(&mut f, power_set)?,
        OutputStyleConfig::Compact => serde_json::to_writer(&mut f, power_set)?,
    }
    Ok(())
}

fn write_powers(powers: &Vec<&ObjRef<BasePower>>, config: &PowersConfig) -> io::Result<()> {
    // NOTE: is it true that all powers in a set share same the source file?
    let source_file = powers
        .first()
        .unwrap()
        .borrow()
        .source_file
        .as_ref()
        .unwrap()
        .to_lowercase();
    let output_file = config.join_to_output_path(format!("{}{}", source_file, JSON_EXT).as_str());
    println!("\tWriting: {} ...", output_file.display());
    ensure_path_exists(&output_file)?;
    let mut f = fs::File::create(&output_file)?;
    match config.output_style {
        OutputStyleConfig::Pretty => serde_json::to_writer_pretty(&mut f, powers)?,
        OutputStyleConfig::Compact => serde_json::to_writer(&mut f, powers)?,
    }
    Ok(())
}

fn write_fx(fx: &PowerFX, config: &PowersConfig) -> io::Result<()> {
    let output_file = config.join_to_output_path(
        format!(
            "{}{}",
            fx.pch_source_file.as_ref().unwrap().to_lowercase(),
            JSON_EXT
        )
        .as_str(),
    );
    println!("\t\tWriting: {} ...", output_file.display());
    ensure_path_exists(&output_file)?;
    let mut f = fs::File::create(&output_file)?;
    match config.output_style {
        OutputStyleConfig::Pretty => serde_json::to_writer_pretty(&mut f, fx)?,
        OutputStyleConfig::Compact => serde_json::to_writer(&mut f, fx)?,
    }
    Ok(())
}

fn write_archetype(archetype: &Archetype, config: &PowersConfig) -> io::Result<()> {
    let output_file = config.join_to_output_path(
        format!(
            "defs/classes/{}{}",
            archetype
                .pch_name
                .as_ref()
                .unwrap()
                .to_lowercase()
                .replace(' ', "_"),
            JSON_EXT
        )
        .as_str(),
    );
    println!("Writing: {} ...", output_file.display());
    ensure_path_exists(&output_file)?;
    let mut f = fs::File::create(&output_file)?;
    match config.output_style {
        OutputStyleConfig::Pretty => serde_json::to_writer_pretty(&mut f, archetype)?,
        OutputStyleConfig::Compact => serde_json::to_writer(&mut f, archetype)?,
    }
    Ok(())
}

fn write_attrib_names(attrib_names: &AttribNames, config: &PowersConfig) -> io::Result<()> {
    let output_file = config.join_to_output_path(format!("defs/attrib_names{}", JSON_EXT).as_str());
    println!("Writing: {} ...", output_file.display());
    ensure_path_exists(&output_file)?;
    let mut f = fs::File::create(&output_file)?;
    match config.output_style {
        OutputStyleConfig::Pretty => serde_json::to_writer_pretty(&mut f, attrib_names)?,
        OutputStyleConfig::Compact => serde_json::to_writer(&mut f, attrib_names)?,
    }
    Ok(())
}

fn ensure_path_exists(path: &Path) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    Ok(())
}

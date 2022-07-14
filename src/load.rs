use crate::bin_parse;
use crate::structs::config::PowersConfig;
use crate::structs::*;
use std::borrow::Cow;
use std::process;
use std::rc::Rc;
use std::time::Instant;

/// Default names for the bin files.
const ATTRIB_NAMES_BIN: &'static str = "attrib_names.bin";
const MESSAGESTORE_BIN: &'static str = "clientmessages-en.bin";
const BOOST_SETS_BIN: &'static str = "boostsets.bin";
const CLASSES_BIN: &'static str = "classes.bin";
const POWER_CATEGORIES_BIN: &'static str = "powercats.bin";
const POWER_SETS_BIN: &'static str = "powersets.bin";
const POWERS_BIN: &'static str = "powers.bin";
const VILLAIN_CLASSES_BIN: &'static str = "villain_classes.bin";
const VILLAIN_DEF_BIN: &'static str = "villaindef.bin";

pub struct ErrContext {
    pub message: Cow<'static, str>,
    pub error: bin_parse::ParseError,
}

macro_rules! ecxt {
    ($msg:literal,$err:ident) => {
        ErrContext {
            message: Cow::Borrowed($msg),
            error: $err,
        }
    };
}

/// Used to find power categories by name referenced from archetypes.
fn find_power_category<'a>(
    power_categories: &'a Keyed<PowerCategory>,
    name: Option<&NameKey>,
) -> Option<std::cell::RefMut<'a, PowerCategory>> {
    if let Some(name) = name {
        if let Some(pcat) = power_categories.get(name) {
            return Some(pcat.borrow_mut());
        }
    }
    None
}

/// Assigns `archetypes` to `power_categories` based on internal criteria defined in those archetypes as
/// well as configuration.
fn match_archetypes_to_power_categories(
    archetypes: &Keyed<Archetype>,
    config: &PowersConfig,
    power_categories: &mut Keyed<PowerCategory>,
) {
    for at in archetypes.values() {
        let a = at.borrow();
        if let Some(mut pcat) =
            find_power_category(power_categories, a.pch_primary_category.as_ref())
        {
            println!(
                "Matched {} to primary {}",
                a.pch_name.as_ref().unwrap(),
                pcat.pch_name.as_ref().unwrap()
            );
            pcat.archetypes.push(Rc::clone(at));
            // theoretically there should only be 1 match per primary/secondary ...
            pcat.pri_sec = PrimarySecondary::Primary;
        }
        if let Some(mut pcat) =
            find_power_category(power_categories, a.pch_secondary_category.as_ref())
        {
            println!(
                "Matched {} to secondary {}",
                a.pch_name.as_ref().unwrap(),
                pcat.pch_name.as_ref().unwrap()
            );
            pcat.archetypes.push(Rc::clone(at));
            pcat.pri_sec = PrimarySecondary::Secondary;
        }
        if let Some(mut pcat) =
            find_power_category(power_categories, a.pch_epic_pool_category.as_ref())
        {
            println!(
                "Matched {} to epic {}",
                a.pch_name.as_ref().unwrap(),
                pcat.pch_name.as_ref().unwrap()
            );
            pcat.archetypes.push(Rc::clone(at));
        }
        if let Some(mut pcat) =
            find_power_category(power_categories, a.pch_power_pool_category.as_ref())
        {
            println!(
                "Matched {} to pool {}",
                a.pch_name.as_ref().unwrap(),
                pcat.pch_name.as_ref().unwrap()
            );
            pcat.archetypes.push(Rc::clone(at));
        }
        for pcat in &config.global_categories {
            if let Some(mut pcat) = find_power_category(power_categories, Some(pcat)) {
                println!(
                    "Matched {} to {}",
                    a.pch_name.as_ref().unwrap(),
                    pcat.pch_name.as_ref().unwrap()
                );
                pcat.archetypes.push(Rc::clone(at));
            }
        }
    }
}

/// Copies references to the `powers` used by `entcreate` into the param itself
/// and marks those powers to be included in the data set.
fn copy_powers_to_entcreate(
    entcreate: &mut AttribModParam_EntCreate,
    villain_archetypes: &Keyed<Archetype>,
    power_cats: &Keyed<PowerCategory>,
    power_sets: &Keyed<BasePowerSet>,
    powers: &Keyed<BasePower>,
) {
    if let Some(villain_def) = &entcreate.villain_def {
        let villain_def = villain_def.borrow();
        // look up the powers specified in the entity def
        for power_ref in &villain_def.powers {
            if matches!(&power_ref.power, Some(s) if s.is_wildcard()) {
                // find all the powers in the specified set
                let power_set_name = format!(
                    "{}.{}",
                    power_ref.power_category.as_ref().unwrap(),
                    power_ref.power_set.as_ref().unwrap()
                );
                if let Some(power_set) = power_sets.get(&power_set_name.into()) {
                    for power_name in &power_set.borrow().pp_power_names {
                        entcreate.power_refs.push(power_name.clone());
                    }
                }
            } else {
                // get a specific power
                let power_name = NameKey::new(format!(
                    "{}.{}.{}",
                    power_ref.power_category.as_ref().unwrap(),
                    power_ref.power_set.as_ref().unwrap(),
                    power_ref.power.as_ref().unwrap()
                ));
                if let Some(power) = powers.get(&power_name) {
                    if let Some(power_name_full) = &power.borrow().pch_full_name {
                        entcreate.power_refs.push(power_name_full.clone());
                    }
                }
            }
        }
        // get the specific archetype for this entity
        let mut archetypes = Vec::new();
        if let Some(class_name) = &villain_def.character_class_name {
            let class_key = NameKey::new(format!("@{}", class_name));
            if let Some(archetype) = villain_archetypes.get(&class_key) {
                archetypes.push(Rc::clone(archetype));
            }
        }
        // now mark all of the powers for inclusion
        for power_name in &entcreate.power_refs {
            mark_power_for_inclusion(power_name, &archetypes, power_cats, power_sets, powers);
        }
    }
}

/// Marks references to the `powers` used by `power_param` to be included in the output.
fn mark_powers_in_power_param(
    power_param: &AttribModParam_Power,
    current_power_name: &NameKey,
    archetypes: &Vec<ObjRef<Archetype>>,
    power_cats: &Keyed<PowerCategory>,
    power_sets: &Keyed<BasePowerSet>,
    powers: &Keyed<BasePower>,
) {
    // the power categories and sets are never used, everything is flattened into the power name
    for power_name in &power_param.ppch_power_names {
        // Some powers reference themselves -- no need to mark (this would also cause a borrow check error)
        if power_name != current_power_name {
            mark_power_for_inclusion(power_name, archetypes, power_cats, power_sets, powers);
        }
    }
}

/// Assigns entity defs in `villains` to `powers` based on the EntCreate and Power attrib mod parameters.
fn resolve_entity_defs_and_power_grants(
    villains: &Keyed<VillainDef>,
    villain_archetypes: &Keyed<Archetype>,
    power_cats: &Keyed<PowerCategory>,
    power_sets: &Keyed<BasePowerSet>,
    powers: &Keyed<BasePower>,
) -> usize {
    let mut count_resolved = 0;
    for power in powers.values().map(|p| p.borrow()) {
        if power.include_in_output {
            // check effect groups for attrib mod params we're interested in
            for mut egroup in power
                .pp_effects
                .iter()
                .map(|e| e.borrow_mut())
            {
                for attrib_mod in &mut egroup.pp_templates {
                    for param in &mut attrib_mod.p_params {
                        match param {
                            AttribModParam::EntCreate(e) if !e.resolved => {
                                if let Some(entity_def_name) = &e.pch_entity_def {
                                    if let Some(entity_def) = villains.get(entity_def_name) {
                                        // copy entity def data into the mod param
                                        e.villain_def = Some(Rc::clone(entity_def));
                                        // copy villain's powers into the mod param
                                        copy_powers_to_entcreate(
                                            e,
                                            &villain_archetypes,
                                            power_cats,
                                            power_sets,
                                            powers,
                                        );
                                    }
                                }
                                e.resolved = true;
                                count_resolved += 1;
                            }
                            AttribModParam::Power(p) if !p.resolved => {
                                // copy powers referred to by this param into it
                                mark_powers_in_power_param(
                                    p,
                                    power.pch_full_name.as_ref().unwrap(),
                                    &power.archetypes,
                                    power_cats,
                                    power_sets,
                                    powers,
                                );
                                p.resolved = true;
                                count_resolved += 1;
                            }
                            _ => (),
                        }
                    }
                }
            }
        }
    }
    count_resolved
}

/// Mark the three parts represented by `power_ref` (category, set, power) to be included
/// in the output set.
fn mark_power_for_inclusion(
    power_ref: &NameKey,
    archetypes: &Vec<ObjRef<Archetype>>,
    power_cats: &Keyed<PowerCategory>,
    power_sets: &Keyed<BasePowerSet>,
    powers: &Keyed<BasePower>,
) {
    // extract the category/set/power names
    let name_parts = power_ref.split();
    debug_assert!(
        name_parts.len() == 3,
        "Unexpected redirect reference {} (needs exactly 3 parts)",
        power_ref,
    );
    // include power category
    if let Some(pcat) = power_cats.get(&NameKey::new(name_parts[0].to_string())) {
        pcat.borrow_mut().include_in_output = true;
    }
    // include power set
    let first_two_parts = format!("{}.{}", name_parts[0], name_parts[1]);
    if let Some(pset) = power_sets.get(&NameKey::new(first_two_parts)) {
        pset.borrow_mut().include_in_output = true;
    }
    // include power
    if let Some(power) = powers.get(power_ref) {
        let mut power = power.borrow_mut();
        power.include_in_output = true;
        // copy archetypes from the power that referenced this one
        for at in archetypes {
            if !power
                .archetypes
                .iter()
                .any(|at2| std::ptr::eq(at.as_ref(), at2.as_ref()))
            {
                power.archetypes.push(Rc::clone(at));
            }
        }
    }
}

/// Mark power categories, sets, and powers to include in the output data based on
/// references to power redirects. Because the default mode is to filter based on archetype
/// categories, redirects wouldn't normally survive since they tend to be in the villain
/// categories.
fn resolve_power_redirects(
    powers: &Keyed<BasePower>,
    power_cats: &Keyed<PowerCategory>,
    power_sets: &Keyed<BasePowerSet>,
) -> usize {
    let mut count_resolved = 0;
    for mut power in powers.values().map(|p| p.borrow_mut()) {
        if power.include_in_output && !power.redirects_resolved {
            // inspect redirects and look at what we need to keep
            for redirect in &power.pp_redirect {
                if let Some(power_name) = &redirect.pch_name {
                    mark_power_for_inclusion(
                        &power_name,
                        &power.archetypes,
                        power_cats,
                        power_sets,
                        powers,
                    );
                }
            }
            power.redirects_resolved = true;
            count_resolved += 1;
        }
    }
    count_resolved
}

/// Iterates through all of the enhancement set categories and tags the powers that can be enhanced
/// by them.
fn match_enh_categories_to_powers(boost_sets: &Keyed<BoostSet>, powers: &mut Keyed<BasePower>) {
    for boost_set in boost_sets.values().map(|b| b.borrow()) {
        if let Some(category_name) = &boost_set.pch_group_name {
            for power_name in &boost_set.ppch_powers {
                if let Some(power) = powers.get(power_name) {
                    power
                        .borrow_mut()
                        .enhancement_set_categories_allowed
                        .insert(category_name.clone());
                }
            }
        }
    }
}

/// Runs a few fix-ups on data contained in power categories, sets, and powers. This comes from
/// some code in Common/entity/powers_load.c. This should always be called last.
fn fix_data_in_power_hierarchy(power_categories: &mut Vec<ObjRef<PowerCategory>>) {
    power_categories
        .iter()
        .map(|p| p.borrow())
        .for_each(|pcat| {
            if pcat.top_level {
                let pcat_name = pcat.pch_name.as_ref().unwrap();
                pcat.pp_power_sets
                    .iter()
                    .map(|p| p.borrow())
                    .for_each(|pset| {
                        pset.pp_powers
                            .iter()
                            .map(|p| p.borrow_mut())
                            .for_each(|mut power| {
                                let power_name = power.pch_name.as_ref().unwrap();
                                // All prestige, inherent, and incarnate powers are free
                                if pcat_name == "Prestige" {
                                    power.b_free = true;
                                    power.i_force_level_bought = 0;
                                } else if pcat_name == "Inherent" || power_name == "Inherent" {
                                    power.b_free = true;
                                    power.b_auto_issue = true;
                                } else if pcat_name == "Incarnate" {
                                    power.b_free = true;
                                }

                                // Set max boosts for temporary powers to zero since you can't slot them.
                                // Disallow all kinds of boosts in them. Temporary powers are also free.
                                if pcat_name == "Temporary_Powers" {
                                    power.b_free = true;
                                    power.i_max_boosts = 0;
                                    match power.e_type {
                                        PowerType::kPowerType_Boost
                                        | PowerType::kPowerType_GlobalBoost => (),
                                        _ => power.pe_boosts_allowed.clear(),
                                    }
                                }
                            });
                    });
            }
        });
}

/// Read all .bin files and merge them into a single powers dictionary.
pub fn load_powers_dictionary(config: &PowersConfig) -> Result<PowersDictionary, ErrContext> {
    let begin_time = Instant::now();

    // load everything
    let messages = read_client_messages(config)?;
    let attrib_names = read_attributes(config, &messages)?;
    let archetypes = read_classes_bin(config, &messages)?;
    let boost_sets = read_boostsets_bin(config, &messages)?;
    let villain_archetypes = read_villain_classes_bin(config, &messages)?;
    let villains = read_villaindef_bin(config, &messages)?;
    let mut power_categories = read_powercats_bin(config, &messages)?;

    // match archetypes to power categories
    println!("Matching archetypes to power categories ...");
    match_archetypes_to_power_categories(&archetypes, &config, &mut power_categories);

    // read in power sets and powers
    let mut power_sets = read_powersets_bin(config, &messages)?;
    let mut powers = read_powers_bin(config, &messages)?;

    // assign enhancement category names to individual powers
    match_enh_categories_to_powers(&boost_sets, &mut powers);

    // filter out power sets
    power_sets.0.retain(|pset_name, _| {
        !config
            .filter_powersets
            .iter()
            .any(|f| pset_name.partial_match(f.get()))
    });

    println!("Merging dictionaries ...");
    // move powers into their power sets
    for mut pset in power_sets.values_mut().map(|p| p.borrow_mut()) {
        let power_names = pset.pp_power_names.clone();
        for power_name in &power_names {
            if let Some(power) = powers.get(power_name) {
                pset.pp_powers.push(Rc::clone(power));
            }
        }
    }

    // move power sets into their power categories
    for mut pcat in power_categories.values_mut().map(|p| p.borrow_mut()) {
        let power_set_names = pcat.ppch_power_set_names.clone();
        for power_set_name in &power_set_names {
            if let Some(pset) = power_sets.get(power_set_name) {
                pcat.pp_power_sets.push(Rc::clone(pset));
            }
        }
    }

    // Reduce the power categories to a vector
    let mut power_categories_returned: Vec<_> = power_categories
        .values()
        .map(|pcat| Rc::clone(pcat))
        .collect();
    // automatically include all power sets and powers linked to the top level
    // also does a sanity check and excludes any that have no powers/power sets
    power_categories_returned
        .iter()
        .map(|p| p.borrow_mut())
        .for_each(|mut pcat| {
            if pcat.top_level {
                pcat.pp_power_sets
                    .iter()
                    .map(|p| p.borrow_mut())
                    .for_each(|mut pset| {
                        pset.pp_powers
                            .iter()
                            .map(|p| p.borrow_mut())
                            .for_each(|mut power| {
                                power.include_in_output = true;
                                power.archetypes = pcat.archetypes.clone();
                            });
                        pset.include_in_output = pset
                            .pp_powers
                            .iter()
                            .any(|pwr| pwr.borrow().include_in_output);
                    });
                pcat.include_in_output = pcat
                    .pp_power_sets
                    .iter()
                    .any(|pset| pset.borrow().include_in_output);
                pcat.top_level = pcat.include_in_output;
            }
        });

    println!("Resolving entity defs, power grants, and redirects ...");
    loop {
        // copy pet entity defs into powers
        let mut count = resolve_entity_defs_and_power_grants(
            &villains,
            &villain_archetypes,
            &mut power_categories,
            &mut power_sets,
            &mut powers,
        );
        // look for redirects and make sure the referenced powers are included in the output data
        count += resolve_power_redirects(&mut powers, &mut power_categories, &mut power_sets);
        if count == 0 {
            break;
        }
    }

    println!("Final clean up ...");
    fix_data_in_power_hierarchy(&mut power_categories_returned);

    let elapsed = Instant::now().duration_since(begin_time);
    println!("Done.");
    println!("Powers dictionary parsed in {} seconds.", elapsed.as_secs());
    Ok(PowersDictionary {
        power_categories: power_categories_returned,
        archetypes,
        attrib_names: Rc::new(attrib_names),
    })
}

/// Read in the clientmessages-en.bin data.
fn read_client_messages(config: &PowersConfig) -> Result<MessageStore, ErrContext> {
    let ms_path = config.join_to_input_path(MESSAGESTORE_BIN);
    println!("Reading {} ...", ms_path.display());
    let mut reader = bin_parse::messagestore::open_message_store(&ms_path)
        .map_err(|e| ecxt!("Unable to open client messages!", e))?;

    let mut messages = MessageStore::new();
    messages.messages = bin_parse::messagestore::read_string_table(&mut reader)
        .map_err(|e| ecxt!("Unable to read message string table!", e))?;
    messages.variables = bin_parse::messagestore::read_string_table(&mut reader)
        .map_err(|e| ecxt!("Unable to read variable string table!", e))?;
    bin_parse::messagestore::read_message_ids(&mut reader, &mut messages)
        .map_err(|e| ecxt!("Unable to read message IDs!", e))?;
    println!("Message store contains {} entries.", messages.len_ids());
    Ok(messages)
}

/// Read in the attrib_names.bin data.
fn read_attributes(
    config: &PowersConfig,
    messages: &MessageStore,
) -> Result<AttribNames, ErrContext> {
    let attr_path = config.join_to_input_path(ATTRIB_NAMES_BIN);
    println!("Reading {} ...", attr_path.display());
    let mut reader = bin_parse::open_serialized(&attr_path)
        .map_err(|e| ecxt!("Unable to open attributes!", e))?;
    let strings = bin_parse::serialized_read_string_pool(&mut reader)
        .map_err(|e| ecxt!("Unable to parse string pool!", e))?;
    let attribs = bin_parse::serialized_read_attribs(&mut reader, &strings, messages)
        .map_err(|e| ecxt!("Unable to read attribute names!", e))?;
    Ok(attribs)
}

/// Read in the classes.bin data.
fn read_classes_bin(
    config: &PowersConfig,
    messages: &MessageStore,
) -> Result<Keyed<Archetype>, ErrContext> {
    let classes_path = config.join_to_input_path(CLASSES_BIN);
    println!("Reading {} ...", classes_path.display());
    let mut reader = bin_parse::open_serialized(&classes_path)
        .map_err(|e| ecxt!("Unable to open classes!", e))?;
    let strings = bin_parse::serialized_read_string_pool(&mut reader)
        .map_err(|e| ecxt!("Unable to parse string pool!", e))?;
    let archetypes = bin_parse::serialized_read_archetypes(&mut reader, &strings, messages, false)
        .map_err(|e| ecxt!("Unable to parse classes table.", e))?;
    println!("Read {} archetypes.", archetypes.len());
    Ok(archetypes)
}

/// Read in the powercats.bin data.
fn read_powercats_bin(
    config: &PowersConfig,
    messages: &MessageStore,
) -> Result<Keyed<PowerCategory>, ErrContext> {
    let pc_path = config.join_to_input_path(POWER_CATEGORIES_BIN);
    println!("Reading {} ...", pc_path.display());
    let mut reader = bin_parse::open_serialized(&pc_path)
        .map_err(|e| ecxt!("Unable to open power categories!", e))?;
    let strings = bin_parse::serialized_read_string_pool(&mut reader)
        .map_err(|e| ecxt!("Unable to parse string pool!", e))?;
    let powercats = bin_parse::serialized_read_power_categories(&mut reader, &strings, messages)
        .map_err(|e| ecxt!("Unable to parse power categories table.", e))?;
    println!("Read {} power categories.", powercats.len());
    if config.power_categories.len() > 0 {
        powercats
            .values()
            .map(|p| p.borrow_mut())
            .for_each(|mut pcat| {
                if config
                    .power_categories
                    .iter()
                    .any(|f| f == pcat.pch_name.as_ref().unwrap())
                {
                    pcat.top_level = true;
                }
            });
        let top_level_count = powercats
            .values()
            .filter(|pcat| pcat.borrow().top_level)
            .count();
        if top_level_count == 0 {
            println!("No power categories to work on. Did you filter them all?");
            process::exit(1);
        }
        println!("Filtered to {} top level categories", top_level_count);
    } else {
        powercats.values().for_each(|pcat| {
            pcat.borrow_mut().top_level = true;
        });
    }
    Ok(powercats)
}

/// Read in the powersets.bin data.
fn read_powersets_bin(
    config: &PowersConfig,
    messages: &MessageStore,
) -> Result<Keyed<BasePowerSet>, ErrContext> {
    let ps_path = config.join_to_input_path(POWER_SETS_BIN);
    println!("Reading {} ...", ps_path.display());
    let mut reader =
        bin_parse::open_serialized(&ps_path).map_err(|e| ecxt!("Unable to open power sets!", e))?;
    let strings = bin_parse::serialized_read_string_pool(&mut reader)
        .map_err(|e| ecxt!("Unable to parse string pool!", e))?;
    let powersets = bin_parse::serialized_read_powersets(&mut reader, &strings, messages)
        .map_err(|e| ecxt!("Unable to parse power sets table.", e))?;
    println!("Read {} power sets.", powersets.len());
    Ok(powersets)
}

/// Read in the powers.bin data.
fn read_powers_bin(
    config: &PowersConfig,
    messages: &MessageStore,
) -> Result<Keyed<BasePower>, ErrContext> {
    let pwr_path = config.join_to_input_path(POWERS_BIN);
    println!("Reading {} ...", pwr_path.display());
    let mut reader =
        bin_parse::open_serialized(&pwr_path).map_err(|e| ecxt!("Unable to open powers!", e))?;
    let strings = bin_parse::serialized_read_string_pool(&mut reader)
        .map_err(|e| ecxt!("Unable to parse string pool!", e))?;
    let powers = bin_parse::serialized_read_powers(&mut reader, &strings, messages)
        .map_err(|e| ecxt!("Unable to parse powers table.", e))?;
    println!("Read {} powers.", powers.len());
    Ok(powers)
}

/// Read in the villain_classes.bin data.
fn read_villain_classes_bin(
    config: &PowersConfig,
    messages: &MessageStore,
) -> Result<Keyed<Archetype>, ErrContext> {
    let classes_path = config.join_to_input_path(VILLAIN_CLASSES_BIN);
    println!("Reading {} ...", classes_path.display());
    let mut reader = bin_parse::open_serialized(&classes_path)
        .map_err(|e| ecxt!("Unable to open classes!", e))?;
    let strings = bin_parse::serialized_read_string_pool(&mut reader)
        .map_err(|e| ecxt!("Unable to parse string pool!", e))?;
    let archetypes = bin_parse::serialized_read_archetypes(&mut reader, &strings, messages, true)
        .map_err(|e| ecxt!("Unable to parse classes table.", e))?;
    println!("Read {} villain archetypes.", archetypes.len());
    Ok(archetypes)
}

/// Read in the VillainDef.bin data.
fn read_villaindef_bin(
    config: &PowersConfig,
    messages: &MessageStore,
) -> Result<Keyed<VillainDef>, ErrContext> {
    let villain_path = config.join_to_input_path(VILLAIN_DEF_BIN);
    println!("Reading {} ...", villain_path.display());
    let mut reader = bin_parse::open_serialized(&villain_path)
        .map_err(|e| ecxt!("Unable to open villains!", e))?;
    let strings = bin_parse::serialized_read_string_pool(&mut reader)
        .map_err(|e| ecxt!("Unable to parse string pool!", e))?;
    let villains = bin_parse::serialized_read_villains(&mut reader, &strings, messages)
        .map_err(|e| ecxt!("Unable to parse villains table.", e))?;
    println!("Read {} villain definitions.", villains.len());
    Ok(villains)
}

/// Read in the boostsets.bin data.
fn read_boostsets_bin(
    config: &PowersConfig,
    messages: &MessageStore,
) -> Result<Keyed<BoostSet>, ErrContext> {
    let boostsets_path = config.join_to_input_path(BOOST_SETS_BIN);
    println!("Reading {} ...", boostsets_path.display());
    let mut reader = bin_parse::open_serialized(&boostsets_path)
        .map_err(|e| ecxt!("Unable to open boost sets!", e))?;
    let strings = bin_parse::serialized_read_string_pool(&mut reader)
        .map_err(|e| ecxt!("Unable to parse string pool!", e))?;
    let boost_sets = bin_parse::serialized_read_boost_sets(&mut reader, &strings, messages)
        .map_err(|e| ecxt!("Unable to parse boost sets table.", e))?;
    println!("Read {} boost sets.", boost_sets.len());
    Ok(boost_sets)
}

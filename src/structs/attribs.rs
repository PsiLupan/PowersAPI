use super::AttribNames;
use serde::{Serialize, Serializer};
use std::borrow::Cow;
use std::rc::Rc;

/// Global cache for the current `AttribNames` data. Some background on this... this is absolutely
/// not the best way to do this, but a compromise. I didn't want to use `serde_state` as a dependency
/// and a global variable seemed the easiest way to accomplish serializing with state without
/// complicated dependencies.
pub static mut GLOBAL_ATTRIB_NAMES: Option<Rc<AttribNames>> = None;

/// Used in attribute name tables.
pub const ORIGINS_SIZE: usize = 5;

/// Matches the width of pointers in the game structs (32 bits).
pub const PTR_SIZE: usize = 4;

/// Defines the attributes which can be modified by effects.
#[derive(Debug, Default, Serialize)]
pub struct CharacterAttributes {
    /// Mod: The number of points to add or remove from current hit points.
    /// ModBase: 0.0, Add, TimesMax, Absolute, HitPoints, DumpAttribs: NO_CUR
    pub f_damage_type: [f32; Self::DAMAGE_TYPE_SIZE],
    /// Cur: Number of hitpoints the player currently has. Running tally.
    /// Mod: How many hitpoints to add or remove from the tally.
    /// ModBase: 0.0, Add, TimesMax, Absolute, HitPoints, DumpAttribs: ALWAYS
    pub f_hit_points: f32,
    /// Max: Number of absorb points the player currently has. Running tally.
    /// ModBase: 0.0, Add, TimesMax, Absolute, HitPoints, DumpAttribs: ALWAYS
    pub f_absorb: f32,
    /// Cur: Measure of endurance the player currently has. Running tally.
    /// Mod: How many points to add or remove from the tally.
    /// ModBase: 0.0, Add, TimesMax, Absolute, DumpAttribs: ALWAYS
    pub f_endurance: f32,
    /// Cur: Measure of Insight the player currently has. Running tally.
    /// Mod: How many points to add or remove from the tally.
    /// ModBase: 0.0, Add, TimesMax, Absolute, DumpAttribs: ALWAYS, Synonym: Idea
    pub f_insight: f32,
    /// Cur: Measure of Rage the player currently has. Running tally.
    /// Mod: How many points to add or remove from the tally.
    /// ModBase: 0.0, Add, TimesMax, Absolute, DumpAttribs: ALWAYS
    pub f_rage: f32,
    /// Cur: The change to hit a target. .75==75%, min 5%, max 95%
    /// Mod: This is a percentage to be added to the base percentage value.
    /// ModBase: 0.0, Add, CLAMP_CUR: No
    pub f_to_hit: f32,
    /// Cur: The chance to avoid being hit by a certain kind of attack. Opposes ToHit.
    /// Mod: This is a percentage added to the base percentage value.
    /// ModBase: 0.0, Add
    pub f_defense_type: [f32; Self::DEFENSE_TYPE_SIZE],
    /// Cur: The chance of avoiding being hit by a direct attack.
    /// Mod: This is a percentage to be added to the base percentage value.
    /// ModBase: 0.0, Add
    pub f_defense: f32,
    /// Cur: How fast the character travels as a percentage of basic character speed. Defaults to 1.0 (100%) (30ft/s).
    /// Mod: A percentage to be multiplied with the base speed value.
    /// ModBase: 1.0, Multiply
    pub f_speed_running: f32,
    /// Cur: How fast the character travels as a percentage of basic character speed. Defaults to 1.0 (100%) (30ft/s).
    /// Mod: A percentage to be multiplied with the base speed value.
    /// ModBase: 1.0, Multiply
    pub f_speed_flying: f32,
    /// Cur: How fast the character travels as a percentage of basic character speed. Defaults to 1.0 (100%) (30ft/s).
    /// Mod: A percentage to be multiplied with the base speed value.
    /// ModBase: 1.0, Multiply
    pub f_speed_swimming: f32,
    /// Cur: How fast the character travels as a percentage of basic character speed. Defaults to 1.0 (100%) (30ft/s).
    /// Mod: A percentage to be multiplied with the base speed value.
    /// ModBase: 1.0, Multiply
    pub f_speed_jumping: f32,
    /// Cur: How well the character jumps as a percentage of basic character jump velocity. Defaults to 1.0 (100%) (12ft).
    /// Mod: A percentage to be multiplied with the base value.
    /// ModBase: 1.0, Multiply
    pub f_jump_height: f32,
    /// Cur: Controls the character's ability to move. Default is 0.0 (use built-ins), running is 1.0, jumping is 0.03.
    /// Mod: This is a percentage to be multiplied with the base value.
    /// ModBase: 1.0, Multiply
    pub f_movement_control: f32,
    /// Cur: Controls the character's ability to move. Default is 0.0 (use built-ins), running is 0.3, jumping is 0.
    /// Mod: This is a percentage to be multiplied with the base value.
    /// ModBase: 1.0, Multiply
    pub f_movement_friction: f32,
    /// Cur: The chance of avoiding being seen when in eyeshot of an enemy.
    /// Mod: This is a percentage to be added to the base percentage value.
    /// ModBase: 0.0, Add
    pub f_stealth: f32,
    /// Cur: This is the distance subtracted from an enemy's perception distance.
    /// Mod: This is a distance to be added to the base distance value.
    /// ModBase: 0.0, Add
    pub f_stealth_radius: f32,
    /// Cur: This is the distance subtracted from an enemy player's perception distance.
    /// Mod: This is a distance to be added to the base distance value.
    /// ModBase: 0.0, Add
    pub f_stealth_radius_player: f32,
    /// Cur: This is the distance the character can see.
    /// Mod: This is a percentage improvement over the base.
    /// ModBase: 1.0, Mutliply, PlusAbsolute
    pub f_perception_radius: f32,
    /// Cur: This is the rate at which hit points are regenerated. (1.0 = 100% max HP per minute.)
    /// Mod: This is a rate which will be multiplied by the base rate.
    /// ModBase: 1.0, Multiply
    pub f_regeneration: f32,
    /// Cur: This is the rate at which endurance is recovered. (1.0 = 100% max endurance per minute.)
    /// Mod: This is a rate which will be multiplied by the base rate.
    /// ModBase: 1.0, Multiply
    pub f_recovery: f32,
    /// Cur: This is the rate at which insight will recover. (1.0 = 100% max insight per minute.)
    /// Mod: This is a rate which will be multiplied by the base rate.
    /// ModBase: 1.0, Multiply
    pub f_insight_recovery: f32,
    /// Cur: The general threat level of the character, used by AI.
    /// Mod: N/A
    /// ModBase: 0.0, Add
    pub f_threat_level: f32,
    /// Cur: This is how much the character is taunting a target. (Not really useful, modifying makes the AI more belligerent to you.)
    /// Mod: N/A
    /// ModBase: 1.0, Add
    pub f_taunt: f32,
    /// Cur: This is how much the character is being placated. (Not really useful, modifying makes the AI less belligerent to you.)
    /// Mod: N/A
    /// ModBase: 1.0, Add
    pub f_placate: f32,
    /// Cur: Wanders around. Boolean.
    /// ModBase: 0.0, Add
    pub f_confused: f32,
    /// Cur: Wants to run away. Boolean.
    /// ModBase: 0.0, Add
    pub f_afraid: f32,
    /// Cur: Cowers. Boolean.
    /// ModBase: 0.0, Add
    pub f_terrorized: f32,
    /// Cur: Cannot move or execute powers. Boolean.
    /// ModBase: 0.0, Add
    pub f_held: f32,
    /// Cur: Cannot move. Boolean.
    /// ModBase: 0.0, Add
    pub f_immobilized: f32,
    /// Cur: Cannot execute powers. Boolean.
    /// ModBase: 0.0, Add
    pub f_stunned: f32,
    /// Cur: Immobilize + stun unless awoken. Boolean.
    /// ModBase: 0.0, Add
    pub f_sleep: f32,
    /// Cur: Can fly. Boolean.
    /// ModBase: 0.0, Add
    pub f_fly: f32,
    /// Cur: Can use jump pack. Boolean.
    /// ModBase: 0.0, Add
    pub f_jump_pack: f32,
    /// Cur: Initiates a teleport. Boolean.
    /// ModBase: 0.0, Add
    pub f_teleport: f32,
    /// Cur: Only caster can hit themself. Boolean.
    /// ModBase: 0.0, Add
    pub f_untouchable: f32,
    /// Cur: Doesn't collide with others. Boolean.
    /// ModBase: 0.0, Add
    pub f_intangible: f32,
    /// Cur: Powers only affect self. Boolean.
    /// ModBase: 0.0, Add
    pub f_only_affects_self: f32,
    /// Cur: XP gain factor.
    /// ModBase: 0.0, Add
    pub f_experience_gain: f32,
    /// Cur: Influence gain factor.
    /// ModBase: 0.0, Add
    pub f_influence_gain: f32,
    /// Cur: Prestige gain factor.
    /// ModBase: 0.0, Add
    pub f_prestige_gain: f32,
    /// Cur: Doesn't do anything.
    /// ModBase: 0.0, Add
    pub f_null_bool: f32,
    /// Cur: How hard the character knocks enemies up as a percentage of base. Default to 1.0 (100%).
    /// Mod: A percentage to be multiplied with the base value.
    /// ModBase: 0.0, Multiply
    pub f_knock_up: f32,
    /// Cur: How hard the character knocks enemies back as a percentage of base. Default to 1.0 (100%).
    /// Mod: A percentage to be multiplied with the base value.
    /// ModBase: 0.0, Multiply
    pub f_knock_back: f32,
    /// Cur: How hard the character repels enemies as a percentage of base. Default to 1.0 (100%).
    /// Mod: A percentage to be multiplied with the base value.
    /// ModBase: 0.0, Multiply
    pub f_repel: f32,
    /// Cur: Unused.
    /// Mod: Unused.
    /// Str: A percentage which is multiplied with a power's facets.
    /// ModBase: 1.0, Multiply, DumpAttribs: STR_RES
    pub f_accuracy: f32,
    /// Cur: Unused.
    /// Mod: Unused.
    /// Str: A percentage which is multiplied with a power's facets.
    /// ModBase: 1.0, Multiply, DumpAttribs: STR_RES
    pub f_radius: f32,
    /// Cur: Unused.
    /// Mod: Unused.
    /// Str: A percentage which is multiplied with a power's facets.
    /// ModBase: 1.0, Multiply, DumpAttribs: STR_RES
    pub f_arc: f32,
    /// Cur: Unused.
    /// Mod: Unused.
    /// Str: A percentage which is multiplied with a power's facets.
    /// ModBase: 1.0, Multiply, DumpAttribs: STR_RES
    pub f_range: f32,
    /// Cur: Unused.
    /// Mod: Unused.
    /// Str: A rate which will be multiplied by the base (hard-coded) rate.
    /// ModBase: 1.0, Multiply, DumpAttribs: STR_RES
    pub f_time_to_activate: f32,
    /// Cur: Unused.
    /// Mod: Unused.
    /// Str: A rate which will be multiplied by the base (hard-coded) rate.
    /// ModBase: 1.0, Multiply, DumpAttribs: STR_RES
    pub f_recharge_time: f32,
    /// Cur: Unused.
    /// Mod: Unused.
    /// Str: A rate which will be multiplied by the base (hard-coded) rate.
    /// ModBase: 1.0, Multiply, DumpAttribs: STR_RES
    pub f_interrupt_time: f32,
    /// Cur: Unused.
    /// Mod: Unused.
    /// Str: This is a magnitude which will divide into the cost.
    /// ModBase: 1.0, Multiply, DumpAttribs: STR_RES
    pub f_endurance_discount: f32,
    /// Cur: Unused.
    /// Mod: Unused.
    /// Str: This is a magnitude which will divide into the cost.
    /// ModBase: 1.0, Multiply, DumpAttribs: STR_RES, NoDump
    pub f_insight_discount: f32,
    /// Cur: A "fake" attribute which shows up as a meter in the UI.
    /// Mod: Amount to increase or decrease the meter.
    /// ModBase: 0.0, Add, PlusAbsolute
    pub f_meter: f32,
    /// Cur: The chance to avoid being hit by a certain kind of attack. Opposes Accuracy. PvP only.
    /// Mod: This is a percentage added to the base percentage value.
    /// Str: Anti-accuracy.
    /// ModBase: 0.0, Add
    pub f_elusivity: [f32; Self::ELUSIVITY_SIZE],
    pub f_elusivity_base: f32,
}

macro_rules! offsets {
	($($name:ident, $offset:literal),+ $(,)?) => {
		$( pub const $name: usize = $offset; )+
	}
}

#[allow(dead_code)]
impl CharacterAttributes {
    pub const DAMAGE_TYPE_SIZE: usize = 20;
    pub const DEFENSE_TYPE_SIZE: usize = 20;
    pub const ELUSIVITY_SIZE: usize = 20;

    // This is pretty annoying but the bin files refer to the
    // various fields in CharacterAttributes by their struct offset.
    // Hopefully no one modifies the source struct...
    // If they do, make sure to fix ranges in effect.rs:get_scaled_effect()
	#[rustfmt::skip]
    offsets!(
		OFFSET_DMG_0, 0,
		OFFSET_DMG_1, 4,
		OFFSET_DMG_2, 8,
		OFFSET_DMG_3, 12,
		OFFSET_DMG_4, 16,
		OFFSET_DMG_5, 20,
		OFFSET_DMG_6, 24,
		OFFSET_DMG_7, 28,
		OFFSET_DMG_8, 32,
		OFFSET_DMG_9, 36,
		OFFSET_DMG_10, 40,
		OFFSET_DMG_11, 44,
		OFFSET_DMG_12, 48,
		OFFSET_DMG_13, 52,
		OFFSET_DMG_14, 56,
		OFFSET_DMG_15, 60,
		OFFSET_DMG_16, 64,
		OFFSET_DMG_17, 68,
		OFFSET_DMG_18, 72,
		OFFSET_DMG_19, 76,
		OFFSET_HIT_POINTS, 80,
		OFFSET_ABSORB, 84,
		OFFSET_ENDURANCE, 88,
		OFFSET_INSIGHT, 92,
		OFFSET_RAGE, 96,
		OFFSET_TOHIT, 100,
		OFFSET_DEF_0, 104,
		OFFSET_DEF_1, 108,
		OFFSET_DEF_2, 112,
		OFFSET_DEF_3, 116,
		OFFSET_DEF_4, 120,
		OFFSET_DEF_5, 124,
		OFFSET_DEF_6, 128,
		OFFSET_DEF_7, 132,
		OFFSET_DEF_8, 136,
		OFFSET_DEF_9, 140,
		OFFSET_DEF_10, 144,
		OFFSET_DEF_11, 148,
		OFFSET_DEF_12, 152,
		OFFSET_DEF_13, 156,
		OFFSET_DEF_14, 160,
		OFFSET_DEF_15, 164,
		OFFSET_DEF_16, 168,
		OFFSET_DEF_17, 172,
		OFFSET_DEF_18, 176,
		OFFSET_DEF_19, 180,
		OFFSET_DEFENSE, 184,
		OFFSET_RUNNING_SPEED, 188,
		OFFSET_FLYING_SPEED, 192,
		OFFSET_SWIMMING_SPEED, 196,
		OFFSET_JUMPING_SPEED, 200,
		OFFSET_JUMP_HEIGHT, 204,
		OFFSET_MOVEMENT_CONTROL, 208,
		OFFSET_MOVEMENT_FRICTION, 212,
		OFFSET_STEALTH, 216,
		OFFSET_STEALTH_RADIUS_PVE, 220,
		OFFSET_STEALTH_RADIUS_PVP, 224,
		OFFSET_PERCEPTION_RADIUS, 228,
		OFFSET_REGENERATION, 232,
		OFFSET_RECOVERY, 236,
		OFFSET_INSIGHT_RECOVERY, 240,
		OFFSET_THREAT_LEVEL, 244,
		OFFSET_TAUNT, 248,
		OFFSET_PLACATE, 252,
		OFFSET_CONFUSED, 256,
		OFFSET_AFRAID, 260,
		OFFSET_TERRORIZED, 264,
		OFFSET_HELD, 268,
		OFFSET_IMMOBILIZED, 272,
		OFFSET_STUNNED, 276,
		OFFSET_SLEEP, 280,
		OFFSET_FLY, 284,
		OFFSET_JUMP_PACK, 288,
		OFFSET_TELEPORT, 292,
		OFFSET_UNTOUCHABLE, 296,
		OFFSET_INTANGIBLE, 300,
		OFFSET_ONLY_AFFECTS_SELF, 304,
		OFFSET_EXPERIENCE_GAIN, 308,
		OFFSET_INFLUENCE_GAIN, 312,
		OFFSET_PRESTIGE_GAIN, 316,
		OFFSET_EVADE, 320,
		OFFSET_KNOCKUP, 324,
		OFFSET_KNOCKBACK, 328,
		OFFSET_REPEL, 332,
		OFFSET_ACCURACY, 336,
		OFFSET_RADIUS, 340,
		OFFSET_ARC, 344,
		OFFSET_RANGE, 348,
		OFFSET_TIME_TO_ACTIVATE, 352,
		OFFSET_RECHARGE_TIME, 356,
		OFFSET_INTERRUPT_TIME, 360,
		OFFSET_ENDURANCE_DISCOUNT, 364,
		OFFSET_INSIGHT_DISCOUNT, 368,
		OFFSET_METER, 372,
		OFFSET_ELUSIVITY_0, 376,
		OFFSET_ELUSIVITY_1, 380,
		OFFSET_ELUSIVITY_2, 384,
		OFFSET_ELUSIVITY_3, 388,
		OFFSET_ELUSIVITY_4, 392,
		OFFSET_ELUSIVITY_5, 396,
		OFFSET_ELUSIVITY_6, 400,
		OFFSET_ELUSIVITY_7, 404,
		OFFSET_ELUSIVITY_8, 408,
		OFFSET_ELUSIVITY_9, 412,
		OFFSET_ELUSIVITY_10, 416,
		OFFSET_ELUSIVITY_11, 420,
		OFFSET_ELUSIVITY_12, 424,
		OFFSET_ELUSIVITY_13, 428,
		OFFSET_ELUSIVITY_14, 432,
		OFFSET_ELUSIVITY_15, 436,
		OFFSET_ELUSIVITY_16, 440,
		OFFSET_ELUSIVITY_17, 444,
		OFFSET_ELUSIVITY_18, 448,
		OFFSET_ELUSIVITY_19, 452,
		OFFSET_ELUSIVITY_BASE, 456,
	);

    pub fn new() -> Self {
        Default::default()
    }
}

/// Defines the attributes which can be modified by effects.
/// This is essentially a version of `CharacterAttributes` where each entry is
/// an array rather than a single value. The arrays are typically 50 entries
/// long, representing values for levels 1-50.
#[derive(Debug, Default, Serialize)]
pub struct CharacterAttributesTable {
    pub pf_damage_type: [Vec<f32>; CharacterAttributes::DAMAGE_TYPE_SIZE],
    pub pf_hit_points: Vec<f32>,
    pub pf_endurance: Vec<f32>,
    pub pf_insight: Vec<f32>,
    pub pf_rage: Vec<f32>,
    pub pf_to_hit: Vec<f32>,
    pub pf_defense_type: [Vec<f32>; CharacterAttributes::DEFENSE_TYPE_SIZE],
    pub pf_defense: Vec<f32>,
    pub pf_speed_running: Vec<f32>,
    pub pf_speed_flying: Vec<f32>,
    pub pf_speed_swimming: Vec<f32>,
    pub pf_speed_jumping: Vec<f32>,
    pub pf_jump_height: Vec<f32>,
    pub pf_movement_control: Vec<f32>,
    pub pf_movement_friction: Vec<f32>,
    pub pf_stealth: Vec<f32>,
    pub pf_stealth_radius: Vec<f32>,
    pub pf_stealth_radius_player: Vec<f32>,
    pub pf_perception_radius: Vec<f32>,
    pub pf_regeneration: Vec<f32>,
    pub pf_recovery: Vec<f32>,
    pub pf_insight_recovery: Vec<f32>,
    pub pf_threat_level: Vec<f32>,
    pub pf_taunt: Vec<f32>,
    pub pf_placate: Vec<f32>,
    pub pf_confused: Vec<f32>,
    pub pf_afraid: Vec<f32>,
    pub pf_terrorized: Vec<f32>,
    pub pf_held: Vec<f32>,
    pub pf_immobilized: Vec<f32>,
    pub pf_stunned: Vec<f32>,
    pub pf_sleep: Vec<f32>,
    pub pf_fly: Vec<f32>,
    pub pf_jump_pack: Vec<f32>,
    pub pf_teleport: Vec<f32>,
    pub pf_untouchable: Vec<f32>,
    pub pf_intangible: Vec<f32>,
    pub pf_only_affects_self: Vec<f32>,
    pub pf_experience_gain: Vec<f32>,
    pub pf_influence_gain: Vec<f32>,
    pub pf_prestige_gain: Vec<f32>,
    pub pf_null_bool: Vec<f32>,
    pub pf_knock_up: Vec<f32>,
    pub pf_knock_back: Vec<f32>,
    pub pf_repel: Vec<f32>,
    pub pf_accuracy: Vec<f32>,
    pub pf_radius: Vec<f32>,
    pub pf_arc: Vec<f32>,
    pub pf_range: Vec<f32>,
    pub pf_time_to_activate: Vec<f32>,
    pub pf_recharge_time: Vec<f32>,
    pub pf_interrupt_time: Vec<f32>,
    pub pf_endurance_discount: Vec<f32>,
    pub pf_insight_discount: Vec<f32>,
    pub pf_meter: Vec<f32>,
    pub pf_elusivity: [Vec<f32>; CharacterAttributes::ELUSIVITY_SIZE],
    pub pf_elusivity_base: Vec<f32>,
    pub pf_absorb: Vec<f32>,
}

impl CharacterAttributesTable {
    pub fn new() -> Self {
        Default::default()
    }
}

/// An offset-based attribute reference from the character. See also `CharacterAttributes` struct.
#[derive(Debug, Default)]
pub struct CharacterAttrib(pub i32);

impl CharacterAttrib {
    /// Shorthand function to convert this `CharacterAttrib` to a `usize` value.
    pub fn usize(&self) -> usize {
        self.0 as usize
    }

    /// Attempts to convert this `CharacterAttrib` into a `SpecialAttrib`.
    pub fn as_special_attrib(&self) -> Option<SpecialAttrib> {
        let attr = SpecialAttrib::from_i32(self.0);
        if !matches!(attr, SpecialAttrib::kSpecialAttrib_Character(_) | SpecialAttrib::kSpecialAttrib_UNSET)
        {
            Some(attr)
        } else {
            None
        }
    }

    /// Converts a character attribute to a human readable string.
    ///
    /// # Arguments:
    /// * `attrib_names` - The attribute name table.
    ///
    /// # Returns:
    /// A String with a human readable name for the attribute.
    pub fn get_string(&self, attrib_names: &AttribNames) -> Option<Cow<'static, str>> {
        macro_rules! retopt {
            ($string:literal) => {
                return Some(Cow::Borrowed($string));
            };
        }
        match self.usize() {
            // The below entries are divided by PTR_SIZE to get the name because they originally refer to
            // memory offsets into the C structs.
            // ppDamage starts at offset OFFSET_DMG_0
            i @ CharacterAttributes::OFFSET_DMG_0..=CharacterAttributes::OFFSET_DMG_19 => {
                if let Some(name) = attrib_names.pp_damage.get(i / PTR_SIZE) {
                    Some(Cow::Owned(format!(
                        "{}_Dmg",
                        name.pch_display_name.as_ref().unwrap()
                    )))
                } else {
                    debug_assert!(false, "Unmapped damage: {}", self.0);
                    None
                }
            }
            // There are a few different versions of these strings stored for use in the UI
            // but I prefer to use my own.
            CharacterAttributes::OFFSET_HIT_POINTS => retopt!("HitPoints"),
            CharacterAttributes::OFFSET_ABSORB => retopt!("Absorb"),
            CharacterAttributes::OFFSET_ENDURANCE => retopt!("Endurance"),
            CharacterAttributes::OFFSET_INSIGHT => retopt!("Insight"),
            CharacterAttributes::OFFSET_RAGE => retopt!("Rage"),
            CharacterAttributes::OFFSET_TOHIT => retopt!("ToHit"),
            // ppDefense starts at offset OFFSET_DEF_0
            i @ CharacterAttributes::OFFSET_DEF_0..=CharacterAttributes::OFFSET_DEF_19 => {
                if let Some(name) = attrib_names
                    .pp_defense
                    .get((i - CharacterAttributes::OFFSET_DEF_0) / PTR_SIZE)
                {
                    Some(Cow::Owned(format!(
                        "{}_Def",
                        name.pch_display_name.as_ref().unwrap()
                    )))
                } else {
                    debug_assert!(false, "Unmapped defense: {}", self.0);
                    None
                }
            }
            CharacterAttributes::OFFSET_DEFENSE => retopt!("Defense"),
            CharacterAttributes::OFFSET_RUNNING_SPEED => retopt!("RunningSpeed"),
            CharacterAttributes::OFFSET_FLYING_SPEED => retopt!("FlyingSpeed"),
            CharacterAttributes::OFFSET_SWIMMING_SPEED => retopt!("SwimmingSpeed"),
            CharacterAttributes::OFFSET_JUMPING_SPEED => retopt!("JumpingSpeed"),
            CharacterAttributes::OFFSET_JUMP_HEIGHT => retopt!("JumpHeight"),
            CharacterAttributes::OFFSET_MOVEMENT_CONTROL => retopt!("MovementControl"),
            CharacterAttributes::OFFSET_MOVEMENT_FRICTION => retopt!("MovementFriction"),
            CharacterAttributes::OFFSET_STEALTH => retopt!("Stealth"),
            CharacterAttributes::OFFSET_STEALTH_RADIUS_PVE => retopt!("StealthRadius_PVE"),
            CharacterAttributes::OFFSET_STEALTH_RADIUS_PVP => retopt!("StealthRadius_PVP"),
            CharacterAttributes::OFFSET_PERCEPTION_RADIUS => retopt!("PerceptionRadius"),
            CharacterAttributes::OFFSET_REGENERATION => retopt!("Regeneration"),
            CharacterAttributes::OFFSET_RECOVERY => retopt!("Recovery"),
            CharacterAttributes::OFFSET_INSIGHT_RECOVERY => retopt!("InsightRecovery"),
            CharacterAttributes::OFFSET_THREAT_LEVEL => retopt!("ThreatLevel"),
            CharacterAttributes::OFFSET_TAUNT => retopt!("Taunt"),
            CharacterAttributes::OFFSET_PLACATE => retopt!("Placate"),
            CharacterAttributes::OFFSET_CONFUSED => retopt!("Confused"),
            CharacterAttributes::OFFSET_AFRAID => retopt!("Afraid"),
            CharacterAttributes::OFFSET_TERRORIZED => retopt!("Terrorized"),
            CharacterAttributes::OFFSET_HELD => retopt!("Held"),
            CharacterAttributes::OFFSET_IMMOBILIZED => retopt!("Immobilized"),
            CharacterAttributes::OFFSET_STUNNED => retopt!("Stunned"),
            CharacterAttributes::OFFSET_SLEEP => retopt!("Sleep"),
            CharacterAttributes::OFFSET_FLY => retopt!("Fly"),
            CharacterAttributes::OFFSET_JUMP_PACK => retopt!("Jump Pack"),
            CharacterAttributes::OFFSET_TELEPORT => retopt!("Teleport"),
            CharacterAttributes::OFFSET_UNTOUCHABLE => retopt!("Untouchable"),
            CharacterAttributes::OFFSET_INTANGIBLE => retopt!("Intangible"),
            CharacterAttributes::OFFSET_ONLY_AFFECTS_SELF => retopt!("OnlyAffectsSelf"),
            CharacterAttributes::OFFSET_EXPERIENCE_GAIN => retopt!("ExperienceGain"),
            CharacterAttributes::OFFSET_INFLUENCE_GAIN => retopt!("InfluenceGain"),
            CharacterAttributes::OFFSET_PRESTIGE_GAIN => retopt!("PrestigeGain"),
            CharacterAttributes::OFFSET_EVADE => retopt!("Evade"),
            CharacterAttributes::OFFSET_KNOCKUP => retopt!("Knockup"),
            CharacterAttributes::OFFSET_KNOCKBACK => retopt!("Knockback"),
            CharacterAttributes::OFFSET_REPEL => retopt!("Repel"),
            CharacterAttributes::OFFSET_ACCURACY => retopt!("Accuracy"),
            CharacterAttributes::OFFSET_RADIUS => retopt!("Radius"),
            CharacterAttributes::OFFSET_ARC => retopt!("Arc"),
            CharacterAttributes::OFFSET_RANGE => retopt!("Range"),
            CharacterAttributes::OFFSET_TIME_TO_ACTIVATE => retopt!("TimeToActivate"),
            CharacterAttributes::OFFSET_RECHARGE_TIME => retopt!("RechargeTime"),
            CharacterAttributes::OFFSET_INTERRUPT_TIME => retopt!("InterruptTime"),
            CharacterAttributes::OFFSET_ENDURANCE_DISCOUNT => retopt!("EnduranceDiscount"),
            CharacterAttributes::OFFSET_INSIGHT_DISCOUNT => retopt!("InsightDiscount"),
            CharacterAttributes::OFFSET_METER => retopt!("Meter"),
            // ppElusivity starts at offset OFFSET_ELUSIVITY_0
            i
            @
            CharacterAttributes::OFFSET_ELUSIVITY_0
                ..=CharacterAttributes::OFFSET_ELUSIVITY_19 => {
                if let Some(name) = attrib_names
                    .pp_elusivity
                    .get((i - CharacterAttributes::OFFSET_ELUSIVITY_0) / PTR_SIZE)
                {
                    Some(Cow::Owned(format!(
                        "{}_Elusivity",
                        name.pch_display_name.as_ref().unwrap()
                    )))
                } else {
                    debug_assert!(false, "Unmapped elusivity: {}", self.0);
                    None
                }
            }
            CharacterAttributes::OFFSET_ELUSIVITY_BASE => retopt!("ElusivityBase"),
            _ => {
                // Special attributes and character attributes share the same offset space,
                // so falling through here to the SpeicalAttrib implementation is expected.
                let attrib = SpecialAttrib::from_i32(self.0);
                Some(Cow::Borrowed(attrib.get_string()))
            }
        }
    }
}

impl Serialize for CharacterAttrib {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let attrib_names = unsafe {
            GLOBAL_ATTRIB_NAMES
                .as_ref()
                .expect("GLOBAL_ATTRIB_NAMES was not initialized")
        };
        if let Some(s) = self.get_string(attrib_names) {
            serializer.serialize_str(&s)
        } else {
            serializer.serialize_none()
        }
    }
}

#[derive(Debug, Default)]
pub struct ModeAttrib(pub i32);

impl ModeAttrib {
    pub fn usize(&self) -> usize {
        self.0 as usize
    }

    /// Converts a mode attribute to a human readable string.
    ///
    /// # Arguments:
    /// * `attrib_names` - The attribute name table.
    ///
    /// # Returns:
    /// A String with a human readable name for the attribute.
    pub fn get_string(&self, attrib_names: &AttribNames) -> Option<String> {
        if self.0 == 0 {
            Some(String::from("ServerTrayOverride"))
        } else if let Some(name) = attrib_names.pp_mode.get(self.usize()) {
            name.pch_name.clone()
        } else {
            None
        }
    }
}

impl Serialize for ModeAttrib {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let attrib_names = unsafe {
            GLOBAL_ATTRIB_NAMES
                .as_ref()
                .expect("GLOBAL_ATTRIB_NAMES was not initialized")
        };
        if let Some(s) = self.get_string(attrib_names) {
            serializer.serialize_str(&s)
        } else {
            serializer.serialize_none()
        }
    }
}

#[derive(Debug, Default)]
pub struct BoostAttrib(pub i32);

impl BoostAttrib {
    pub fn usize(&self) -> usize {
        self.0 as usize
    }

    /// Converts a boost attribute to a human readable string.
    ///
    /// # Arguments:
    /// * `attrib_names` - The attribute name table.
    ///
    /// # Returns:
    /// A String with a human readable name for the attribute.
    pub fn get_string(&self, attrib_names: &AttribNames) -> Option<String> {
        match self.usize() {
            i @ ORIGINS_SIZE..=99 => {
                // Why do we subtract ORIGINS_SIZE? Good question! Check this lovely note I found in the code:
                //
                // > mw 3.10.06 added guard here because it's everywhere else this calc is done,
                // > and there's reported crash here that I can't repro, so I'm doing this and hoping for the best
                // > (subtracting off the number of origins seems insane and neither Jered nor CW can remember why its needed)
                //
                // Coding is weird, folks :)
                //
                // Follow up: It's possible the weird 4..3..2..1..0 sequence seen in several powers (such as incarnates) is a
                // reference to those origins that's been trimmed out here.
                if let Some(name) = attrib_names.pp_boost.get(i - ORIGINS_SIZE) {
                    name.pch_display_name.clone()
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

impl Serialize for BoostAttrib {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let attrib_names = unsafe {
            GLOBAL_ATTRIB_NAMES
                .as_ref()
                .expect("GLOBAL_ATTRIB_NAMES was not initialized")
        };
        if let Some(s) = self.get_string(attrib_names) {
            serializer.serialize_str(&s)
        } else {
            serializer.serialize_none()
        }
    }
}

// see ESpecialAttrib in Common/entity/character_attribs.h
#[derive(Debug, Serialize)]
#[allow(non_camel_case_types)]
pub enum SpecialAttrib {
    kSpecialAttrib_Character(i32),
    kSpecialAttrib_Translucency,
    kSpecialAttrib_EntCreate,
    kSpecialAttrib_ClearDamagers,
    kSpecialAttrib_SilentKill,
    kSpecialAttrib_XPDebtProtection,
    kSpecialAttrib_SetMode,
    kSpecialAttrib_SetCostume,
    kSpecialAttrib_Glide,
    kSpecialAttrib_Null,
    kSpecialAttrib_Avoid,
    kSpecialAttrib_Reward,
    kSpecialAttrib_XPDebt,
    kSpecialAttrib_DropToggles,
    kSpecialAttrib_GrantPower,
    kSpecialAttrib_RevokePower,
    kSpecialAttrib_UnsetMode,
    kSpecialAttrib_GlobalChanceMod,
    kSpecialAttrib_PowerChanceMod,
    kSpecialAttrib_GrantBoostedPower,
    kSpecialAttrib_ViewAttrib,
    kSpecialAttrib_RewardSource,
    kSpecialAttrib_RewardSourceTeam,
    kSpecialAttrib_ClearFog,
    kSpecialAttrib_CombatPhase,
    kSpecialAttrib_CombatModShift,
    kSpecialAttrib_RechargePower,
    kSpecialAttrib_VisionPhase,
    kSpecialAttrib_NinjaRun,
    kSpecialAttrib_Walk,
    kSpecialAttrib_BeastRun,
    kSpecialAttrib_SteamJump,
    kSpecialAttrib_DesignerStatus,
    kSpecialAttrib_ExclusiveVisionPhase,
    kSpecialAttrib_HoverBoard,
    kSpecialAttrib_SetSZEValue,
    kSpecialAttrib_AddBehavior,
    kSpecialAttrib_MagicCarpet,
    kSpecialAttrib_TokenAdd,
    kSpecialAttrib_TokenSet,
    kSpecialAttrib_TokenClear,
    kSpecialAttrib_LuaExec,
    kSpecialAttrib_ForceMove,
    kSpecialAttrib_ParkourRun,
    kSpecialAttrib_CancelMods,
    kSpecialAttrib_ExecutePower,
    kSpecialAttrib_PowerRedirect,
    kSpecialAttrib_UNSET,
}

impl Default for SpecialAttrib {
    fn default() -> Self {
        SpecialAttrib::kSpecialAttrib_UNSET
    }
}

impl SpecialAttrib {
    /// Special attributes start after the end of the character attributes.
    pub const SIZE_OF_CHARACTER_ATTRIBUTES: i32 = 460;

    /// Converts an `i32` value to a `SpecialAttrib` value.
    pub fn from_i32(val: i32) -> Self {
        match val {
            460 => SpecialAttrib::kSpecialAttrib_Translucency,
            461 => SpecialAttrib::kSpecialAttrib_EntCreate,
            462 => SpecialAttrib::kSpecialAttrib_ClearDamagers,
            463 => SpecialAttrib::kSpecialAttrib_SilentKill,
            464 => SpecialAttrib::kSpecialAttrib_XPDebtProtection,
            465 => SpecialAttrib::kSpecialAttrib_SetMode,
            466 => SpecialAttrib::kSpecialAttrib_SetCostume,
            467 => SpecialAttrib::kSpecialAttrib_Glide,
            468 => SpecialAttrib::kSpecialAttrib_Null,
            469 => SpecialAttrib::kSpecialAttrib_Avoid,
            470 => SpecialAttrib::kSpecialAttrib_Reward,
            471 => SpecialAttrib::kSpecialAttrib_XPDebt,
            472 => SpecialAttrib::kSpecialAttrib_DropToggles,
            473 => SpecialAttrib::kSpecialAttrib_GrantPower,
            474 => SpecialAttrib::kSpecialAttrib_RevokePower,
            475 => SpecialAttrib::kSpecialAttrib_UnsetMode,
            476 => SpecialAttrib::kSpecialAttrib_GlobalChanceMod,
            477 => SpecialAttrib::kSpecialAttrib_PowerChanceMod,
            478 => SpecialAttrib::kSpecialAttrib_GrantBoostedPower,
            479 => SpecialAttrib::kSpecialAttrib_ViewAttrib,
            480 => SpecialAttrib::kSpecialAttrib_RewardSource,
            481 => SpecialAttrib::kSpecialAttrib_RewardSourceTeam,
            482 => SpecialAttrib::kSpecialAttrib_ClearFog,
            483 => SpecialAttrib::kSpecialAttrib_CombatPhase,
            484 => SpecialAttrib::kSpecialAttrib_CombatModShift,
            485 => SpecialAttrib::kSpecialAttrib_RechargePower,
            486 => SpecialAttrib::kSpecialAttrib_VisionPhase,
            487 => SpecialAttrib::kSpecialAttrib_NinjaRun,
            488 => SpecialAttrib::kSpecialAttrib_Walk,
            489 => SpecialAttrib::kSpecialAttrib_BeastRun,
            490 => SpecialAttrib::kSpecialAttrib_SteamJump,
            491 => SpecialAttrib::kSpecialAttrib_DesignerStatus,
            492 => SpecialAttrib::kSpecialAttrib_ExclusiveVisionPhase,
            493 => SpecialAttrib::kSpecialAttrib_HoverBoard,
            494 => SpecialAttrib::kSpecialAttrib_SetSZEValue,
            495 => SpecialAttrib::kSpecialAttrib_AddBehavior,
            496 => SpecialAttrib::kSpecialAttrib_MagicCarpet,
            497 => SpecialAttrib::kSpecialAttrib_TokenAdd,
            498 => SpecialAttrib::kSpecialAttrib_TokenSet,
            499 => SpecialAttrib::kSpecialAttrib_TokenClear,
            500 => SpecialAttrib::kSpecialAttrib_LuaExec,
            501 => SpecialAttrib::kSpecialAttrib_ForceMove,
            502 => SpecialAttrib::kSpecialAttrib_ParkourRun,
            503 => SpecialAttrib::kSpecialAttrib_CancelMods,
            504 => SpecialAttrib::kSpecialAttrib_ExecutePower,
            1460 => SpecialAttrib::kSpecialAttrib_PowerRedirect,
            _ => SpecialAttrib::kSpecialAttrib_Character(val),
        }
    }

    /// Gets a human readable string representing this attribute.
    ///
    /// # Notes:
    /// This only applies to general attributes. For boost, mode, and character
    /// attributes, see the other methods in this implementation.
    pub fn get_string(&self) -> &'static str {
        match self {
            SpecialAttrib::kSpecialAttrib_UNSET => "",
            SpecialAttrib::kSpecialAttrib_Character(_) => "Character Attribute",
            SpecialAttrib::kSpecialAttrib_Translucency => "Translucency",
            SpecialAttrib::kSpecialAttrib_EntCreate => "Create Entity",
            SpecialAttrib::kSpecialAttrib_ClearDamagers => "Clear Damagers",
            SpecialAttrib::kSpecialAttrib_SilentKill => "Silent Kill",
            SpecialAttrib::kSpecialAttrib_XPDebtProtection => "Debt Protection",
            SpecialAttrib::kSpecialAttrib_SetMode => "Set Mode",
            SpecialAttrib::kSpecialAttrib_SetCostume => "Set Costume",
            SpecialAttrib::kSpecialAttrib_Glide => "Glide",
            SpecialAttrib::kSpecialAttrib_Null => "Null",
            SpecialAttrib::kSpecialAttrib_Avoid => "Avoid",
            SpecialAttrib::kSpecialAttrib_Reward => "Reward",
            SpecialAttrib::kSpecialAttrib_XPDebt => "Debt",
            SpecialAttrib::kSpecialAttrib_DropToggles => "Drop Toggles",
            SpecialAttrib::kSpecialAttrib_GrantPower => "Grant Power",
            SpecialAttrib::kSpecialAttrib_RevokePower => "Revoke Power",
            SpecialAttrib::kSpecialAttrib_UnsetMode => "Unset Mode",
            SpecialAttrib::kSpecialAttrib_GlobalChanceMod => "Global Chance Mod",
            SpecialAttrib::kSpecialAttrib_PowerChanceMod => "Power Chance Mod",
            SpecialAttrib::kSpecialAttrib_GrantBoostedPower => "Grant Boosted Power",
            SpecialAttrib::kSpecialAttrib_ViewAttrib => "View Attributes",
            SpecialAttrib::kSpecialAttrib_RewardSource => "Reward Source",
            SpecialAttrib::kSpecialAttrib_RewardSourceTeam => "Reward Source Team",
            SpecialAttrib::kSpecialAttrib_ClearFog => "Clear Fog",
            SpecialAttrib::kSpecialAttrib_CombatPhase => "Combat Phase",
            SpecialAttrib::kSpecialAttrib_CombatModShift => "Level Shift",
            SpecialAttrib::kSpecialAttrib_RechargePower => "Recharge Power",
            SpecialAttrib::kSpecialAttrib_VisionPhase => "Vision Phase",
            SpecialAttrib::kSpecialAttrib_NinjaRun => "Ninja Run",
            SpecialAttrib::kSpecialAttrib_Walk => "Walk",
            SpecialAttrib::kSpecialAttrib_BeastRun => "Beast Run",
            SpecialAttrib::kSpecialAttrib_SteamJump => "Steam Jump",
            SpecialAttrib::kSpecialAttrib_DesignerStatus => "Designer Status",
            SpecialAttrib::kSpecialAttrib_ExclusiveVisionPhase => "Exclusive Vision Phase",
            SpecialAttrib::kSpecialAttrib_HoverBoard => "Hover Board",
            SpecialAttrib::kSpecialAttrib_SetSZEValue => "Set Script Value",
            SpecialAttrib::kSpecialAttrib_AddBehavior => "Add Behavior",
            SpecialAttrib::kSpecialAttrib_MagicCarpet => "Magic Carpet",
            SpecialAttrib::kSpecialAttrib_TokenAdd => "Add Token",
            SpecialAttrib::kSpecialAttrib_TokenSet => "Set Token",
            SpecialAttrib::kSpecialAttrib_TokenClear => "Clear Token",
            SpecialAttrib::kSpecialAttrib_LuaExec => "Execute Script",
            SpecialAttrib::kSpecialAttrib_ForceMove => "Force Move",
            SpecialAttrib::kSpecialAttrib_ParkourRun => "Parkour Run",
            SpecialAttrib::kSpecialAttrib_CancelMods => "Cancel Effects",
            SpecialAttrib::kSpecialAttrib_ExecutePower => "Execute Power",
            SpecialAttrib::kSpecialAttrib_PowerRedirect => "Redirect Power",
        }
    }
}

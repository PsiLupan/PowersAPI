#![allow(non_upper_case_globals)]

use super::attribs::SpecialAttrib;
use serde::{Serialize, Serializer};

bitflags! {
    #[derive(Default)]
    pub struct EffectGroupFlag: u32 {
        /// If true, this effect group is ignored while on PVP maps.
        const PVEOnly = 1;
        /// If true, this effect group is ignored while on PVE maps.
        const PVPOnly = 1 << 1;
        /// Fallback effect groups are normally ignored. If no non-fallback effect
        /// groups within the same collection are eligible to be applied
        const Fallback = 1 << 2;
        /// Effect only the main target. Added i26p6 (replaced the deprecated LinkedChance).
        /// Might've actually been p5 but I missed it.
        const MainTargetOnly = 1 << 3;
        /// Effect only the secondary targets. Added i26p6 or p5.
        const SecondaryTargetsOnly = 1 << 4;
        /// Added i27
        const HideFromInfo = 1 << 5;
        /// Added i27
        const HitRollSuccess = 1 << 6;
        /// Added i27
        const HitRollFail = 1 << 7;    }
}

/// Used below to map values of attrib mod flags back to their human-readable names.
#[rustfmt::skip]
const EFFECT_GROUP_FLAGS_TO_STRINGS: &'static [(EffectGroupFlag, &'static str)] = &[
    (EffectGroupFlag::PVEOnly, "PVEOnly"),
    (EffectGroupFlag::PVPOnly, "PVPOnly"),
    (EffectGroupFlag::Fallback, "Fallback"),
    (EffectGroupFlag::MainTargetOnly, "MainTargetOnly"),
    (EffectGroupFlag::SecondaryTargetsOnly, "SecondaryTargetsOnly"),
    (EffectGroupFlag::HideFromInfo, "HideFromInfo"),
    (EffectGroupFlag::HitRollSuccess, "HitRollSuccess"),
    (EffectGroupFlag::HitRollFail, "HitRollFail"),
];

impl EffectGroupFlag {
    /// Converts an `EffectGroupFlag` value to human-readable strings for each bit.
    ///
    /// # Returns
    /// A `Vec<String>` containing zero or more values based on the current `AttribModFlag`.
    pub fn get_strings(&self) -> Vec<&'static str> {
        let mut strings = Vec::new();
        for (a, s) in EFFECT_GROUP_FLAGS_TO_STRINGS {
            if self.contains(*a) {
                strings.push(*s);
            }
        }
        strings
    }
}

impl Serialize for EffectGroupFlag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_seq(self.get_strings())
    }
}

bitflags! {
    #[derive(Default)]
    pub struct AttribModFlag: u32 {
        /// If set, hides floaters (damage and healing numbers, for example) over the affected's head.
        /// If specified, `pch_display_float` is always shown, even if this is set.
        const NoFloaters = 1;
        /// Determines whether this attribmod ignores diminishing returns on boosts
        /// (aka Enhancement Diversification). Only used if aspect is Strength.
        const BoostIgnoreDiminishing = 1 << 1;
        /// If set and the test governed by `f_tick_chance` fails, the attrib mod will be removed.
        const CancelOnMiss = 1 << 2;
        /// If set, only applies the attrib modifier if the target is on the ground.
        const NearGround = 1 << 3;
        /// If set, the attacker's strength is not used to modify the scale of the effects.
        const IgnoreStrength = 1 << 4;
        /// If set, the target's resistance is not used to modify the scale of the applied effects.
        const IgnoreResistance = 1 << 5;
        /// If set, the level difference between the source and the target
        /// is not used to modify the effect's magnitude and/or duration.
        const IgnoreCombatMods = 1 << 6;
        /// If set, forces resistance to apply to something other than the default (based on the
        /// attrib mod type).
        const ResistMagnitude = 1 << 7;
        /// If set, forces resistance to apply to something other than the default (based on the
        /// attrib mod type).
        const ResistDuration = 1 << 8;
        /// If set, forces combat mod to apply to something other than the default (based on the
        /// attrib mod type).
        const CombatModMagnitude = 1 << 9;
        /// If set, forces combat mod to apply to something other than the default (based on the
        /// attrib mod type).
        const CombatModDuration = 1 << 10;
        /// If true, this is supposed to be a boost template.  This is used
        /// in boosts and powers used as set bonuses, which can include both
        /// boost templates and additional normal templates.
        const Boost = 1 << 11;
        /// If true and `pch_display_float`, `pch_display_attacker_hit` or `pch_display_victim_hit`
        /// is specified, it will only display if the attribute evaluates to a non-zero value.
        const HideZero = 1 << 12;
        /// If true, do not clean out this attrib mod when the entity dies.
        const KeepThroughDeath = 1 << 13;
        /// If true, delay any evaluations associated with this attrib mod until the last possible moment in mod_process.
        /// This means you will have to store all the Eval stashes until that time comes. Note that this can cause
        /// desynchronization with other members of the same effect group.
        const DelayEval = 1 << 14;
        /// Do not add the FramesBeforeHit delay from the power to this attrib mod.
        const NoHitDelay = 1 << 15;
        /// Do not add the projectile distance delay from the power to this attribmod.
        const NoProjectileDelay = 1 << 16;
        /// When using StackKey stacking, also compare the Aspect/Attribute in addition to the key.
        const StackByAttribAndKey = 1 << 17;
        /// Apply stacking rules per power instance rather than by the template. Implies individual caster stacking.
        const StackExactPower = 1 << 18;
        /// Designer laziness flag.
        const IgnoreSuppressErrors = 1 << 19;
    }
}

#[derive(Debug)]
pub enum EffectSpecificAttribModFlag {
    // BIT 0
    /// Valid for: EntCreate
    /// If true, if the pet times out or is otherise destroyed by the server (as opposed to being defeated) then the entity is
    /// vanished as opposed to going through the usual DieNow code. (Only for powers which spawn entities.)
    VanishEntOnTimeout,
    // Valid for: CombatModShift
    // Causes this mod shift not to be added to the total reported to the client.
    DoNotDisplayShift,
    /// Valid for: TokenAdd, TokenSet
    /// Don't update the token timer.
    NoTokenTime,
    /// Valid for: RevokePower
    /// Revokes all copies of the power, ignoring Count.
    RevokeAll,
    // Valid for: Knock
    // Added i26p5. If the target is flying, knock effects normally ignore the special
    // height calculation. This flag forces the use of the height parameters even for flying targets.
    #[allow(dead_code)]
    AlwaysUseHeight,
    /// Valid for: RechargePower
    /// Added i27. Instead of recharging the power to ready, sets the recharge timer to
    /// a specific value. The magnitude of the attribmod is taken as a time in seconds.
    SetTimer,

    // BIT 1
    /// Valid for: EntCreate
    /// If true, do not apply custom tinting to the spawned pet's costume.
    DoNotTintCostume,
    // Valid for: ExecutePower
    // Added i27. Performs an additional line-of-sight check between caster and target
    // when the exectued power is activated.
    CheckLoS,
    /// Valid for: RechargePower
    /// Added i27. With aspect kAbs, adds the raw value of magnitude to the power's recharge timer.
    /// With aspect kCur, multiplies the power's current recharge timer by the magnitude.
    AdjustTimer,

    // BIT 2
    /// Valid for: ExecutePower, EntCreate
    /// Copy enhancements to the resulting power(s) if they are accepted by its allowed types.
    CopyBoosts,
    /// Valid for: RechargePower
    /// Added i27. Places the power on cooldown, using the recharge time as defined by the power definition.
    Cooldown,

    // BIT 3+
    /// Valid for: EntCreate
    /// Copy strength buff mods from the creator of this entity.
    CopyCreatorMods,
    /// Valid for: EntCreate
    /// Suppresses FX on mods copied from creator. Only has an effect if CopyCreatorMods is also set.
    NoCreatorModFX,
    /// Valid for: EntCreate
    /// Ignores `pch_villain_def` and `pch_class`, creates a generic entity the same class as its creator.
    /// Implies NoCreatorModFX.
    PseudoPet,
    /// Valid for: EntCreate
    /// Forces the summoned entity to show up in a player's pet window.
    PetVisible,
    /// Valid for: EntCreate
    /// Forces the summoned entity to be commandable like a mastermind pet.
    PetCommandable,
    // Valid for: EntCreate
    // Added i26p5. Copies the costume of the creator, as if it were a doppelganger.
    CopyCreatorCostume,
}

/// Used below to map values of attrib mod flags back to their human-readable names.
#[rustfmt::skip]
const ATTRIB_MOD_FLAGS_TO_STRINGS: &'static [(AttribModFlag, &'static str)] = &[
    (AttribModFlag::NoFloaters, "NoFloaters"),
    (AttribModFlag::BoostIgnoreDiminishing, "BoostIgnoreDiminishing"),
    (AttribModFlag::CancelOnMiss, "CancelOnMiss"),
    (AttribModFlag::NearGround, "NearGround"),
    (AttribModFlag::IgnoreStrength, "IgnoreStrength"),
    (AttribModFlag::IgnoreResistance, "IgnoreResistance"),
    (AttribModFlag::IgnoreCombatMods, "IgnoreLevelDifference"),
    (AttribModFlag::ResistMagnitude, "ResistMagnitude"),
    (AttribModFlag::ResistDuration, "ResistDuration"),
    (AttribModFlag::CombatModMagnitude, "CombatModMagnitude"),
    (AttribModFlag::CombatModDuration, "CombatModDuration"),
    (AttribModFlag::Boost, "Boost"),
    (AttribModFlag::HideZero, "HideZero"),
    (AttribModFlag::KeepThroughDeath, "KeepThroughDeath"),
    (AttribModFlag::DelayEval, "DelayEval"),
    (AttribModFlag::NoHitDelay, "NoHitDelay"),
    (AttribModFlag::NoProjectileDelay, "NoProjectileDelay"),
    (AttribModFlag::StackByAttribAndKey, "StackByAttribAndKey"),
    (AttribModFlag::StackExactPower, "StackExactPower"),
    (AttribModFlag::IgnoreSuppressErrors, "IgnoreSupressErrors"),
];

impl AttribModFlag {
    /// Converts an `AttribModFlag` value to human-readable strings for each bit.
    ///
    /// # Returns
    /// A `Vec<String>` containing zero or more values based on the current `AttribModFlag`.
    pub fn get_strings(&self) -> Vec<&'static str> {
        let mut strings = Vec::new();
        for (a, s) in ATTRIB_MOD_FLAGS_TO_STRINGS {
            if self.contains(*a) {
                strings.push(*s);
            }
        }
        strings
    }
}

impl Serialize for AttribModFlag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_seq(self.get_strings())
    }
}

impl EffectSpecificAttribModFlag {
    /// Converts an `EffectSpecificAttribModFlag` value to a human-readable string.
    ///
    /// # Returns
    /// A string based on the current `EffectSpecificAttribModFlag`.
    pub fn get_string(&self) -> &'static str {
        match self {
            EffectSpecificAttribModFlag::VanishEntOnTimeout => "VanishEntOnTimeout",
            EffectSpecificAttribModFlag::DoNotDisplayShift => "DoNotDisplayShift",
            EffectSpecificAttribModFlag::NoTokenTime => "NoTokenTime",
            EffectSpecificAttribModFlag::RevokeAll => "RevokeAll",
            EffectSpecificAttribModFlag::AlwaysUseHeight => "AlwaysUseHeight",
            EffectSpecificAttribModFlag::SetTimer => "SetTimer",
            EffectSpecificAttribModFlag::DoNotTintCostume => "DoNotTintCostume",
            EffectSpecificAttribModFlag::CheckLoS => "CheckLineOfSight",
            EffectSpecificAttribModFlag::AdjustTimer => "AdjustTimer",
            EffectSpecificAttribModFlag::CopyBoosts => "CopyBoosts",
            EffectSpecificAttribModFlag::Cooldown => "Cooldown",
            EffectSpecificAttribModFlag::CopyCreatorMods => "CopyCreatorMods",
            EffectSpecificAttribModFlag::NoCreatorModFX => "NoCreatorModFX",
            EffectSpecificAttribModFlag::PseudoPet => "PseudoPet",
            EffectSpecificAttribModFlag::PetVisible => "PetVisible",
            EffectSpecificAttribModFlag::PetCommandable => "PetCommandable",
            EffectSpecificAttribModFlag::CopyCreatorCostume => "CopyCreatorCostume",
        }
    }

    /// Converts a `u32` value to an `EffectSpecificAttribModFlag`. Because the value depends on other characteristcs
    /// of the power, a simple conversion is not possible like `AttribModFlag`.
    ///
    /// # Parameters
    /// * `value` - The raw flags from the bin.
    /// * `special` - The corresponding `SpecialAttrib` for the current attrib mod, if any.
    ///
    /// # Returns
    /// A `Vec<EffectSpecificAttribModFlag>` containing zero or more values.
    pub fn from_bits(value: u32, special: &SpecialAttrib) -> Vec<Self> {
        let bad = |bit| {
            debug_assert!(
                false,
                "Unknown EffectSpecificAttribModFlag bit {} for {:?}",
                bit, special
            );
        };
        let mut flags = Vec::new();
        if value == 0 || matches!(special, SpecialAttrib::kSpecialAttrib_UNSET) {
            return flags;
        }
        // bit 0
        if value & 1 != 0 {
            match special {
                SpecialAttrib::kSpecialAttrib_EntCreate => {
                    flags.push(EffectSpecificAttribModFlag::VanishEntOnTimeout)
                }
                SpecialAttrib::kSpecialAttrib_CombatModShift => {
                    flags.push(EffectSpecificAttribModFlag::DoNotDisplayShift)
                }
                SpecialAttrib::kSpecialAttrib_TokenAdd | SpecialAttrib::kSpecialAttrib_TokenSet => {
                    flags.push(EffectSpecificAttribModFlag::NoTokenTime)
                }
                SpecialAttrib::kSpecialAttrib_RevokePower => {
                    flags.push(EffectSpecificAttribModFlag::RevokeAll)
                }
                //SpecialAttrib::kSpecialAttrib_Knock => flags.push(EffectSpecificAttribModFlag::AlwaysUseHeight),
                SpecialAttrib::kSpecialAttrib_RechargePower => {
                    flags.push(EffectSpecificAttribModFlag::SetTimer)
                }
                SpecialAttrib::kSpecialAttrib_SetMode => (), // ???
                _ => bad(0),
            }
        }
        // bit 1
        if value & (1 << 1) != 0 {
            match special {
                SpecialAttrib::kSpecialAttrib_EntCreate => {
                    flags.push(EffectSpecificAttribModFlag::DoNotTintCostume)
                }
                SpecialAttrib::kSpecialAttrib_ExecutePower => {
                    flags.push(EffectSpecificAttribModFlag::CheckLoS)
                }
                SpecialAttrib::kSpecialAttrib_RechargePower => {
                    flags.push(EffectSpecificAttribModFlag::AdjustTimer)
                }
                _ => bad(1),
            }
        }
        // bit 2
        if value & (1 << 2) != 0 {
            match special {
                SpecialAttrib::kSpecialAttrib_EntCreate
                | SpecialAttrib::kSpecialAttrib_ExecutePower => {
                    flags.push(EffectSpecificAttribModFlag::CopyBoosts)
                }
                SpecialAttrib::kSpecialAttrib_RechargePower => {
                    flags.push(EffectSpecificAttribModFlag::Cooldown)
                }
                _ => bad(2),
            }
        }
        // the rest of the bits are only valid for EntCreate, so the test is inverted
        if matches!(special, SpecialAttrib::kSpecialAttrib_EntCreate) {
            // bit 3
            if value & (1 << 3) != 0 {
                flags.push(EffectSpecificAttribModFlag::CopyCreatorMods);
            }
            // bit 4
            if value & (1 << 4) != 0 {
                flags.push(EffectSpecificAttribModFlag::NoCreatorModFX);
            }
            // bit 5
            if value & (1 << 5) != 0 {
                flags.push(EffectSpecificAttribModFlag::PseudoPet);
            }
            // bit 6
            if value & (1 << 6) != 0 {
                flags.push(EffectSpecificAttribModFlag::PetVisible);
            }
            // bit 7
            if value & (1 << 7) != 0 {
                flags.push(EffectSpecificAttribModFlag::PetCommandable);
            }
            // bit 8
            if value & (1 << 8) != 0 {
                flags.push(EffectSpecificAttribModFlag::CopyCreatorCostume);
            }
        } else if value > 0b111 {
            debug_assert!(
                false,
                "Unknown EffectSpecificAttribModFlag bit > 2 ({:b}) for {:?}",
                value, special
            );
        }
        flags
    }
}

impl Serialize for EffectSpecificAttribModFlag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.get_string())
    }
}

bitflags! {
    #[derive(Default)]
    pub struct VillainExclusion: u32
    {
        /// Allow in all games.
        const VE_NONE = 0;
        /// Allow in _CoH_ only.
        const VE_COH = 1;
        /// Allow in _CoV_ only.
        const VE_COV = 1 << 1;
        /// ???
        const VE_MA = 1 << 2;
    }
}

bitflags! {
    #[derive(Default)]
    pub struct VillainDefFlags: u32 {
        /// Don't count a badge stat for the villain group when defeated.
        const VILLAINDEF_NOGROUPBADGESTAT = 1;
        /// Don't count a badge stat for the villain rank when defeated.
        const VILLAINDEF_NORANKBADGESTAT = 1 << 2;
        /// Don't count a badge stat for the villain name when defeated.
        const VILLAINDEF_NONAMEBADGESTAT = 1 << 3;
        const VILLAINDEF_NOGENERICBADGESTAT = Self::VILLAINDEF_NOGROUPBADGESTAT.bits | Self::VILLAINDEF_NORANKBADGESTAT.bits | Self::VILLAINDEF_NONAMEBADGESTAT.bits;
    }
}

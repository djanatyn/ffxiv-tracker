use std::collections::BTreeMap;
use strum::{Display, EnumIter, EnumString, IntoEnumIterator};

/// Player jobs. When encountering an initial class (such as Marauder), we parse
/// it as the appropriate matching job (such as Warrior).
///
/// Because Arcanist becomes both Scholar and Summoner, we parse Arcanist as
/// Summoner.
///
/// Limited jobs such as Blue Mage are included.
#[derive(
    Debug, EnumString, EnumIter, Eq, Hash, PartialEq, Clone, Copy, PartialOrd, Ord, Display,
)]
#[strum(serialize_all = "title_case")]
pub enum Job {
    // tanks
    #[strum(to_string = "Paladin", serialize = "Gladiator")]
    Paladin,
    #[strum(to_string = "Warrior", serialize = "Marauder")]
    Warrior,
    DarkKnight,
    Gunbreaker,
    // healers
    #[strum(to_string = "White Mage", serialize = "Conjurer")]
    WhiteMage,
    Scholar,
    Astrologian,
    Sage,
    // melee dps
    #[strum(to_string = "Monk", serialize = "Pugilist")]
    Monk,
    #[strum(to_string = "Dragoon", serialize = "Lancer")]
    Dragoon,
    #[strum(to_string = "Ninja", serialize = "Rogue")]
    Ninja,
    Samurai,
    Reaper,
    Viper,
    // phys ranged
    #[strum(to_string = "Bard", serialize = "Archer")]
    Bard,
    Machinist,
    Dancer,
    // casters
    #[strum(to_string = "Black Mage", serialize = "Thaumaturge")]
    BlackMage,
    #[strum(to_string = "Summoner", serialize = "Arcanist")]
    Summoner,
    RedMage,
    Pictomancer,
    BlueMage,
    // disciples of hand
    Carpenter,
    Armorer,
    Blacksmith,
    Goldsmith,
    Leatherworker,
    Weaver,
    Alchemist,
    Culinarian,
    // disciples of land
    Miner,
    Botanist,
    Fisher,
}

#[derive(Debug)]
/// A snapshot of a job's experience level taken from a lodestone profile.
pub struct JobSnapshot {
    pub job: Job,
    pub level: Option<u64>,
    pub exp: Option<(u64, u64)>,
}

/// Snapshot of all jobs.
#[derive(Debug)]
pub struct PlayerJobSnapshot(pub BTreeMap<Job, JobSnapshot>);

impl TryFrom<Vec<JobSnapshot>> for PlayerJobSnapshot {
    type Error = String;

    fn try_from(snapshots: Vec<JobSnapshot>) -> Result<Self, Self::Error> {
        let mut jobs: BTreeMap<Job, JobSnapshot> = BTreeMap::new();
        for snapshot in snapshots {
            jobs.insert(snapshot.job, snapshot);
        }

        // check for each job before constructing
        for job in Job::iter() {
            if !jobs.contains_key(&job) {
                Err(format!("missing {job:?}"))?;
            }
        }

        Ok(PlayerJobSnapshot(jobs))
    }
}

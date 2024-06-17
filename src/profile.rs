use std::{collections::HashMap, str::FromStr};

use scraper::{ElementRef, Html, Selector};

#[derive(Debug)]
pub struct JobSnapshot {
    job: Job,
    level: String, // TODO
    exp: String,   // TODO
}

#[derive(Debug)]
pub struct PlayerSnapshot {
    // tanks
    paladin: JobSnapshot,
    warrior: JobSnapshot,
    dark_knight: JobSnapshot,
    gunbreaker: JobSnapshot,
    // healers
    white_mage: JobSnapshot,
    scholar: JobSnapshot,
    astrologian: JobSnapshot,
    sage: JobSnapshot,
    // melee dps
    monk: JobSnapshot,
    dragoon: JobSnapshot,
    ninja: JobSnapshot,
    samurai: JobSnapshot,
    reaper: JobSnapshot,
    // phys ranged
    bard: JobSnapshot,
    machinist: JobSnapshot,
    dancer: JobSnapshot,
    // casters
    black_mage: JobSnapshot,
    summoner: JobSnapshot,
    red_mage: JobSnapshot,
    blue_mage: JobSnapshot,
    // disciples of hand
    carpenter: JobSnapshot,
    blacksmith: JobSnapshot,
    armorer: JobSnapshot,
    goldsmith: JobSnapshot,
    leatherworker: JobSnapshot,
    weaver: JobSnapshot,
    alchemist: JobSnapshot,
    culinarian: JobSnapshot,
    // disciples of land
    miner: JobSnapshot,
    botanist: JobSnapshot,
    fisher: JobSnapshot,
}

impl TryFrom<Vec<JobSnapshot>> for PlayerSnapshot {
    type Error = String;

    fn try_from(snapshots: Vec<JobSnapshot>) -> Result<Self, Self::Error> {
        let mut jobs: HashMap<Job, JobSnapshot> = HashMap::new();
        for snapshot in snapshots {
            jobs.insert(snapshot.job.clone(), snapshot);
        }
        Ok(PlayerSnapshot {
            paladin: jobs.remove(&Job::Paladin).ok_or("missing paladin")?,
            warrior: jobs.remove(&Job::Warrior).ok_or("missing warrior")?,
            dark_knight: jobs.remove(&Job::DarkKnight).ok_or("missing dark knight")?,
            gunbreaker: jobs.remove(&Job::Gunbreaker).ok_or("missing gunbreaker")?,
            white_mage: jobs.remove(&Job::WhiteMage).ok_or("missing white mage")?,
            scholar: jobs.remove(&Job::Scholar).ok_or("missing scholar")?,
            astrologian: jobs
                .remove(&Job::Astrologian)
                .ok_or("missing astrologian")?,
            sage: jobs.remove(&Job::Sage).ok_or("missing sage")?,
            monk: jobs.remove(&Job::Monk).ok_or("missing monk")?,
            dragoon: jobs.remove(&Job::Dragoon).ok_or("missing dragoon")?,
            ninja: jobs.remove(&Job::Ninja).ok_or("missing ninja")?,
            samurai: jobs.remove(&Job::Samurai).ok_or("missing samurai")?,
            reaper: jobs.remove(&Job::Reaper).ok_or("missing reaper")?,
            bard: jobs.remove(&Job::Bard).ok_or("missing bard")?,
            machinist: jobs.remove(&Job::Machinist).ok_or("missing machinist")?,
            dancer: jobs.remove(&Job::Dancer).ok_or("missing dancer")?,
            black_mage: jobs.remove(&Job::BlackMage).ok_or("missing black mage")?,
            summoner: jobs.remove(&Job::Summoner).ok_or("missing summoner")?,
            red_mage: jobs.remove(&Job::RedMage).ok_or("missing red mage")?,
            blue_mage: jobs.remove(&Job::BlueMage).ok_or("missing blue mage")?,
            carpenter: jobs.remove(&Job::Carpenter).ok_or("missing carpenter")?,
            blacksmith: jobs.remove(&Job::Blacksmith).ok_or("missing blacksmith")?,
            armorer: jobs.remove(&Job::Armorer).ok_or("missing armorer")?,
            goldsmith: jobs.remove(&Job::Goldsmith).ok_or("missing goldsmith")?,
            leatherworker: jobs
                .remove(&Job::Leatherworker)
                .ok_or("missing leatherworker")?,
            weaver: jobs.remove(&Job::Weaver).ok_or("missing weaver")?,
            alchemist: jobs.remove(&Job::Alchemist).ok_or("missing alchemist")?,
            culinarian: jobs.remove(&Job::Culinarian).ok_or("missing culinarian")?,
            miner: jobs.remove(&Job::Miner).ok_or("missing miner")?,
            botanist: jobs.remove(&Job::Botanist).ok_or("missing botanist")?,
            fisher: jobs.remove(&Job::Fisher).ok_or("missing fisher")?,
        })
    }
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub enum Job {
    // tanks
    Paladin,
    Warrior,
    DarkKnight,
    Gunbreaker,
    // healers
    WhiteMage,
    Scholar,
    Astrologian,
    Sage,
    // melee dps
    Monk,
    Dragoon,
    Ninja,
    Samurai,
    Reaper,
    // phys ranged
    Bard,
    Machinist,
    Dancer,
    // casters
    BlackMage,
    Summoner,
    RedMage,
    BlueMage,
    // disciples of hand
    Carpenter,
    Blacksmith,
    Armorer,
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

impl FromStr for Job {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "paladin" => Ok(Self::Paladin),
            "warrior" => Ok(Self::Warrior),
            "dark knight" => Ok(Self::DarkKnight),
            "gunbreaker" => Ok(Self::Gunbreaker),
            // healers
            "white mage" => Ok(Self::WhiteMage),
            "scholar" => Ok(Self::Scholar),
            "astrologian" => Ok(Self::Astrologian),
            "sage" => Ok(Self::Sage),
            // melee dps
            "monk" => Ok(Self::Monk),
            "dragoon" => Ok(Self::Dragoon),
            "ninja" => Ok(Self::Ninja),
            "samurai" => Ok(Self::Samurai),
            "reaper" => Ok(Self::Reaper),
            // phys ranged
            "bard" => Ok(Self::Bard),
            "machinist" => Ok(Self::Machinist),
            "dancer" => Ok(Self::Dancer),
            // casters
            "black mage" => Ok(Self::BlackMage),
            "summoner" => Ok(Self::Summoner),
            "red mage" => Ok(Self::RedMage),
            "blue mage" => Ok(Self::BlueMage),
            // disciples of hand
            "carpenter" => Ok(Self::Carpenter),
            "blacksmith" => Ok(Self::Blacksmith),
            "armorer" => Ok(Self::Armorer),
            "goldsmith" => Ok(Self::Goldsmith),
            "leatherworker" => Ok(Self::Leatherworker),
            "weaver" => Ok(Self::Weaver),
            "alchemist" => Ok(Self::Alchemist),
            "culinarian" => Ok(Self::Culinarian),
            // disciples of land
            "miner" => Ok(Self::Miner),
            "botanist" => Ok(Self::Botanist),
            "fisher" => Ok(Self::Fisher),
            _ => Err(format!("failed to parse job name: {}", s)),
        }
    }
}

#[derive(Debug)]
pub struct Profile {}
impl Profile {
    pub const BASE_URL: &'static str = "https://na.finalfantasyxiv.com/lodestone/character";

    fn get(id: u64) -> Result<Profile, String> {
        let url = format!("{0}/{id}/class_job/", Profile::BASE_URL);
        let html = ureq::get(url.as_str())
            .call()
            .map_err(|e| e.to_string())?
            .into_string()
            .map_err(|e| e.to_string())?;
        let document = Html::parse_document(&html);

        let select_jobs = Selector::parse("ul.character__job li").map_err(|e| e.to_string())?;
        let select_level =
            Selector::parse("div.character__job__level").map_err(|e| e.to_string())?;
        let select_job_name =
            Selector::parse("div.character__job__name").map_err(|e| e.to_string())?;
        let select_exp = Selector::parse("div.character__job__exp").map_err(|e| e.to_string())?;

        let mut jobs: Vec<JobSnapshot> = vec![];
        for job_details in document.select(&select_jobs) {
            let level = job_details
                .select(&select_level)
                .next()
                .ok_or("couldn't find level")?
                .text()
                .next()
                .ok_or("no level")?;
            let job_name = job_details
                .select(&select_job_name)
                .next()
                .ok_or("couldn't find job name")?
                .text()
                .next()
                .ok_or("no job name")?;
            let job = Job::from_str(&job_name)?;
            let exp = job_details
                .select(&select_exp)
                .next()
                .ok_or("couldn't find exp")?
                .text()
                .next()
                .ok_or("no exp")?;
            jobs.push(JobSnapshot {
                job,
                level: level.to_string(),
                exp: exp.to_string(),
            });
        }
        dbg!(PlayerSnapshot::try_from(jobs));
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    /// https://na.finalfantasyxiv.com/lodestone/character/38598907/class_job/
    fn fetch_profile() -> Result<(), String> {
        let profile = Profile::get(38598907)?;
        Ok(())
    }
}

#![allow(unused)]

use scraper::{Html, Selector};
use std::{collections::HashMap, str::FromStr};

#[derive(Debug, parse_display::Display, parse_display::FromStr, Eq, Hash, PartialEq, Clone)]
#[display(style = "Title Case")]
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

#[derive(Debug)]
pub struct JobSnapshot {
    job: Job,
    level: String, // TODO
    exp: String,   // TODO
}

#[derive(Debug)]
pub struct JobSnapshots {
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

impl TryFrom<Vec<JobSnapshot>> for JobSnapshots {
    type Error = String;

    fn try_from(snapshots: Vec<JobSnapshot>) -> Result<Self, Self::Error> {
        let mut jobs: HashMap<Job, JobSnapshot> = HashMap::new();
        for snapshot in snapshots {
            jobs.insert(snapshot.job.clone(), snapshot);
        }
        Ok(JobSnapshots {
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

#[derive(Debug)]
pub struct Profile {
    user_id: u64,
    free_company: String,
    name: String,
    nameday: String,
    guardian: String,
    city_state: String,
    server: String,
    race_clan_gender: String, // TODO
    hp: u64,
    mp: u64,
    jobs: JobSnapshots,
}

impl Profile {
    pub const BASE_URL: &'static str = "https://na.finalfantasyxiv.com/lodestone/character";

    fn get(user_id: u64) -> Result<Profile, String> {
        // extract profile page info
        let profile_url = format!("{0}/{user_id}", Profile::BASE_URL);
        let profile_html = ureq::get(profile_url.as_str())
            .call()
            .map_err(|e| e.to_string())?
            .into_string()
            .map_err(|e| e.to_string())?;
        let profile_document = Html::parse_document(&profile_html);

        let select_free_company =
            Selector::parse("div.character__freecompany__name h4").map_err(|e| e.to_string())?;
        let select_name = Selector::parse("p.frame__chara__name").map_err(|e| e.to_string())?;
        let select_nameday =
            Selector::parse("p.character-block__birth").map_err(|e| e.to_string())?;
        let select_guardian =
            Selector::parse("p.character-block__name").map_err(|e| e.to_string())?;
        let select_city_state =
            Selector::parse("p.character-block__name").map_err(|e| e.to_string())?;
        let select_server = Selector::parse("p.frame__chara__world").map_err(|e| e.to_string())?;
        let select_race_clan_gender =
            Selector::parse("p.character-block__name").map_err(|e| e.to_string())?;
        let select_hp = Selector::parse("p.character__param__text__hp--en-us + span")
            .map_err(|e| e.to_string())?;
        let select_mp = Selector::parse("p.character__param__text__mp--en-us + span")
            .map_err(|e| e.to_string())?;

        let free_company = profile_document
            .select(&select_free_company)
            .next()
            .ok_or("couldn't find free_company")?
            .text()
            .next()
            .ok_or("no free_company")?
            .to_string();
        let name = profile_document
            .select(&select_name)
            .next()
            .ok_or("couldn't find name")?
            .text()
            .next()
            .ok_or("no name")?
            .to_string();
        let nameday = profile_document
            .select(&select_nameday)
            .next()
            .ok_or("couldn't find nameday")?
            .text()
            .next()
            .ok_or("no nameday")?
            .to_string();
        let guardian = profile_document
            .select(&select_guardian)
            .next()
            .ok_or("couldn't find guardian")?
            .text()
            .next()
            .ok_or("no guardian")?
            .to_string();
        let city_state = profile_document
            .select(&select_city_state)
            .next()
            .ok_or("couldn't find city_state")?
            .text()
            .next()
            .ok_or("no city_state")?
            .to_string();
        let server = profile_document
            .select(&select_server)
            .next()
            .ok_or("couldn't find server")?
            .text()
            .next()
            .ok_or("no server")?
            .to_string();
        let race_clan_gender = profile_document
            .select(&select_race_clan_gender)
            .next()
            .ok_or("couldn't find race_clan_gender")?
            .text()
            .next()
            .ok_or("no race_clan_gender")?
            .to_string();
        let hp = profile_document
            .select(&select_hp)
            .next()
            .ok_or("couldn't find hp")?
            .text()
            .next()
            .ok_or("no hp")?
            .to_string()
            .parse::<u64>()
            .map_err(|e| e.to_string())?;
        let mp = profile_document
            .select(&select_mp)
            .next()
            .ok_or("couldn't find mp")?
            .text()
            .next()
            .ok_or("no mp")?
            .to_string()
            .parse::<u64>()
            .map_err(|e| e.to_string())?;

        // extract job page info
        let job_url = format!("{0}/{user_id}/class_job/", Profile::BASE_URL);
        let job_html = ureq::get(job_url.as_str())
            .call()
            .map_err(|e| e.to_string())?
            .into_string()
            .map_err(|e| e.to_string())?;
        let job_document = Html::parse_document(&job_html);

        let select_jobs = Selector::parse("ul.character__job li").map_err(|e| e.to_string())?;
        let select_level =
            Selector::parse("div.character__job__level").map_err(|e| e.to_string())?;
        let select_job_name =
            Selector::parse("div.character__job__name").map_err(|e| e.to_string())?;
        let select_exp = Selector::parse("div.character__job__exp").map_err(|e| e.to_string())?;

        let mut snapshots: Vec<JobSnapshot> = vec![];
        for job_details in job_document.select(&select_jobs) {
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
            let job = Job::from_str(&job_name).map_err(|e| e.to_string())?;
            let exp = job_details
                .select(&select_exp)
                .next()
                .ok_or("couldn't find exp")?
                .text()
                .next()
                .ok_or("no exp")?;
            snapshots.push(JobSnapshot {
                job,
                level: level.to_string(),
                exp: exp.to_string(),
            });
        }
        let jobs = JobSnapshots::try_from(snapshots)?;
        Ok(Profile {
            jobs,
            user_id,
            free_company,
            name,
            nameday,
            guardian,
            city_state,
            server,
            race_clan_gender,
            hp,
            mp,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    /// https://na.finalfantasyxiv.com/lodestone/character/38598907/class_job/
    fn fetch_profile() -> Result<(), String> {
        let profile = dbg!(Profile::get(38598907)?);
        Ok(())
    }
}

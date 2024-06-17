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
pub struct PlayerJobSnapshot(HashMap<Job, JobSnapshot>);

impl TryFrom<Vec<JobSnapshot>> for PlayerJobSnapshot {
    type Error = String;

    fn try_from(snapshots: Vec<JobSnapshot>) -> Result<Self, Self::Error> {
        let mut jobs: HashMap<Job, JobSnapshot> = HashMap::new();
        for snapshot in snapshots {
            jobs.insert(snapshot.job.clone(), snapshot);
        }
        // check for each job before constructing
        jobs.contains_key(&Job::Paladin) || Err("missing paladin")?;
        jobs.contains_key(&Job::Warrior) || Err("missing warrior")?;
        jobs.contains_key(&Job::DarkKnight) || Err("missing dark knight")?;
        jobs.contains_key(&Job::Gunbreaker) || Err("missing gunbreaker")?;
        jobs.contains_key(&Job::WhiteMage) || Err("missing white mage")?;
        jobs.contains_key(&Job::Scholar) || Err("missing scholar")?;
        jobs.contains_key(&Job::Astrologian) || Err("missing astrologian")?;
        jobs.contains_key(&Job::Sage) || Err("missing sage")?;
        jobs.contains_key(&Job::Monk) || Err("missing monk")?;
        jobs.contains_key(&Job::Dragoon) || Err("missing dragoon")?;
        jobs.contains_key(&Job::Ninja) || Err("missing ninja")?;
        jobs.contains_key(&Job::Samurai) || Err("missing samurai")?;
        jobs.contains_key(&Job::Reaper) || Err("missing reaper")?;
        jobs.contains_key(&Job::Bard) || Err("missing bard")?;
        jobs.contains_key(&Job::Machinist) || Err("missing machinist")?;
        jobs.contains_key(&Job::Dancer) || Err("missing dancer")?;
        jobs.contains_key(&Job::BlackMage) || Err("missing black mage")?;
        jobs.contains_key(&Job::Summoner) || Err("missing summoner")?;
        jobs.contains_key(&Job::RedMage) || Err("missing red mage")?;
        jobs.contains_key(&Job::BlueMage) || Err("missing blue mage")?;
        jobs.contains_key(&Job::Carpenter) || Err("missing carpenter")?;
        jobs.contains_key(&Job::Blacksmith) || Err("missing blacksmith")?;
        jobs.contains_key(&Job::Armorer) || Err("missing armorer")?;
        jobs.contains_key(&Job::Goldsmith) || Err("missing goldsmith")?;
        jobs.contains_key(&Job::Leatherworker) || Err("missing leatherworker")?;
        jobs.contains_key(&Job::Weaver) || Err("missing weaver")?;
        jobs.contains_key(&Job::Alchemist) || Err("missing alchemist")?;
        jobs.contains_key(&Job::Culinarian) || Err("missing culinarian")?;
        jobs.contains_key(&Job::Miner) || Err("missing miner")?;
        jobs.contains_key(&Job::Botanist) || Err("missing botanist")?;
        jobs.contains_key(&Job::Fisher) || Err("missing fisher")?;

        Ok(PlayerJobSnapshot(jobs))
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
    jobs: PlayerJobSnapshot,
}

impl Profile {
    pub const BASE_URL: &'static str = "https://na.finalfantasyxiv.com/lodestone/character";

    fn get(user_id: u64) -> Result<Profile, String> {
        let profile_url = format!("{0}/{user_id}", Profile::BASE_URL);
        let profile_html = ureq::get(profile_url.as_str())
            .call()
            .map_err(|e| e.to_string())?
            .into_string()
            .map_err(|e| e.to_string())?;
        let profile_html = Html::parse_document(&profile_html);

        let job_url = format!("{0}/{user_id}/class_job/", Profile::BASE_URL);
        let job_html = ureq::get(job_url.as_str())
            .call()
            .map_err(|e| e.to_string())?
            .into_string()
            .map_err(|e| e.to_string())?;
        let job_html = Html::parse_document(&job_html);

        Self::parse(user_id, profile_html, job_html)
    }

    fn parse(user_id: u64, profile_html: Html, jobs_html: Html) -> Result<Profile, String> {
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

        let free_company = profile_html
            .select(&select_free_company)
            .next()
            .ok_or("couldn't find free_company")?
            .text()
            .next()
            .ok_or("no free_company")?
            .to_string();
        let name = profile_html
            .select(&select_name)
            .next()
            .ok_or("couldn't find name")?
            .text()
            .next()
            .ok_or("no name")?
            .to_string();
        let nameday = profile_html
            .select(&select_nameday)
            .next()
            .ok_or("couldn't find nameday")?
            .text()
            .next()
            .ok_or("no nameday")?
            .to_string();
        let guardian = profile_html
            .select(&select_guardian)
            .next()
            .ok_or("couldn't find guardian")?
            .text()
            .next()
            .ok_or("no guardian")?
            .to_string();
        let city_state = profile_html
            .select(&select_city_state)
            .next()
            .ok_or("couldn't find city_state")?
            .text()
            .next()
            .ok_or("no city_state")?
            .to_string();
        let server = profile_html
            .select(&select_server)
            .next()
            .ok_or("couldn't find server")?
            .text()
            .next()
            .ok_or("no server")?
            .to_string();
        let race_clan_gender = profile_html
            .select(&select_race_clan_gender)
            .next()
            .ok_or("couldn't find race_clan_gender")?
            .text()
            .next()
            .ok_or("no race_clan_gender")?
            .to_string();
        let hp = profile_html
            .select(&select_hp)
            .next()
            .ok_or("couldn't find hp")?
            .text()
            .next()
            .ok_or("no hp")?
            .to_string()
            .parse::<u64>()
            .map_err(|e| e.to_string())?;
        let mp = profile_html
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
        let select_jobs = Selector::parse("ul.character__job li").map_err(|e| e.to_string())?;
        let select_level =
            Selector::parse("div.character__job__level").map_err(|e| e.to_string())?;
        let select_job_name =
            Selector::parse("div.character__job__name").map_err(|e| e.to_string())?;
        let select_exp = Selector::parse("div.character__job__exp").map_err(|e| e.to_string())?;

        let mut snapshots: Vec<JobSnapshot> = vec![];
        for job_details in jobs_html.select(&select_jobs) {
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
        let jobs = PlayerJobSnapshot::try_from(snapshots)?;
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
        let user_id: u64 = 38598907;
        let profile_html = Html::parse_document(include_str!("tests/yov_ziv_profile.html"));
        let jobs_html = Html::parse_document(include_str!("tests/yov_ziv_jobs.html"));

        dbg!(Profile::parse(user_id, profile_html, jobs_html)?);
        Ok(())
    }
}

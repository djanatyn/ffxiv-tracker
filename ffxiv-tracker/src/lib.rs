#![allow(unused)]

use scraper::{Html, Selector};
use std::{collections::BTreeMap, str::FromStr};
use strum::{EnumIter, EnumString, IntoEnumIterator};

#[derive(Debug, EnumString, EnumIter, Eq, Hash, PartialEq, Clone, Copy, PartialOrd, Ord)]
#[strum(serialize_all = "title_case")]
pub enum Job {
    // tanks
    #[strum(serialize = "Paladin", serialize = "Gladiator")]
    Paladin,
    #[strum(serialize = "Marauder", serialize = "Warrior")]
    Warrior,
    DarkKnight,
    Gunbreaker,
    // healers
    #[strum(serialize = "Conjurer", serialize = "White Mage")]
    WhiteMage,
    Scholar,
    Astrologian,
    Sage,
    // melee dps
    #[strum(serialize = "Pugilist", serialize = "Monk")]
    Monk,
    #[strum(serialize = "Lancer", serialize = "Dragoon")]
    Dragoon,
    #[strum(serialize = "Rogue", serialize = "Ninja")]
    Ninja,
    Samurai,
    Reaper,
    // phys ranged
    #[strum(serialize = "Archer", serialize = "Bard")]
    Bard,
    Machinist,
    Dancer,
    // casters
    #[strum(serialize = "Thaumaturge", serialize = "Black Mage")]
    BlackMage,
    #[strum(serialize = "Arcanist", serialize = "Summoner")]
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
    level: Option<u64>,
    exp: Option<(u64, u64)>,
}

#[derive(Debug)]
/// Snapshot of all jobs. Containers either Arcanist, or Summoner, but not both.
pub struct PlayerJobSnapshot(BTreeMap<Job, JobSnapshot>);

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

#[derive(Debug)]
pub struct Profile {
    user_id: u64,
    free_company: Option<String>,
    name: String,
    nameday: String,
    guardian: String,
    city_state: String,
    server: String,
    race: String,
    clan: String,
    gender: String,
    grand_company: String,
    grand_company_rank: String,
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
        let select_profile_info =
            Selector::parse("p.character-block__name").map_err(|e| e.to_string())?;
        let select_server = Selector::parse("p.frame__chara__world").map_err(|e| e.to_string())?;
        let select_hp = Selector::parse("p.character__param__text__hp--en-us + span")
            .map_err(|e| e.to_string())?;
        let select_mp = Selector::parse("p.character__param__text__mp--en-us + span")
            .map_err(|e| e.to_string())?;
        let free_company = match profile_html.select(&select_free_company).next() {
            Some(element) => element.text().next().map(|txt| txt.to_string()),
            None => None,
        };
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
        let server = profile_html
            .select(&select_server)
            .next()
            .ok_or("couldn't find server")?
            .text()
            .next()
            .ok_or("no server")?
            .to_string();
        let info: Vec<Vec<&str>> = profile_html
            .select(&select_profile_info)
            .map(|e| e.text().collect::<Vec<&str>>())
            .collect();
        let (race, clan_gender) = match info[0][..] {
            [race, clan_gender] => (race.to_string(), clan_gender),
            _ => Err("failed to find race/clan/gender")?,
        };
        let (clan, gender) = match clan_gender
            .split('/')
            .map(|t| t.trim())
            .collect::<Vec<&str>>()[..]
        {
            [clan, gender] => (clan.to_string(), gender.to_string()),
            _ => Err("failed to parse clan/gender")?,
        };
        let guardian = info[1]
            .first()
            .ok_or("failed to find guardian")?
            .to_string();
        let city_state = info[2]
            .first()
            .ok_or("failed to find city state")?
            .to_string();
        let (grand_company, grand_company_rank) = match info[3]
            .first()
            .ok_or("failed to find grand company info")?
            .split('/')
            .map(|t| t.trim())
            .collect::<Vec<&str>>()[..]
        {
            [grand_company, grand_company_rank] => {
                (grand_company.to_string(), grand_company_rank.to_string())
            }
            _ => Err("failed to parse grand company + grand_company rank")?,
        };
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

        // extract job info page
        let select_jobs = Selector::parse("ul.character__job li").map_err(|e| e.to_string())?;
        let select_level =
            Selector::parse("div.character__job__level").map_err(|e| e.to_string())?;
        let select_job_name =
            Selector::parse("div.character__job__name").map_err(|e| e.to_string())?;
        let select_exp = Selector::parse("div.character__job__exp").map_err(|e| e.to_string())?;

        let mut snapshots: Vec<JobSnapshot> = vec![];
        for job_details in jobs_html.select(&select_jobs) {
            let level_string = job_details
                .select(&select_level)
                .next()
                .ok_or("couldn't find level")?
                .text()
                .next()
                .ok_or("no level")?;
            let level = match level_string {
                "-" => None,
                some => some.replace(",", "").trim().parse::<u64>().ok(),
            };
            let job_name = job_details
                .select(&select_job_name)
                .next()
                .ok_or("couldn't find job name")?
                .text()
                .next()
                .ok_or("no job name")?;
            let job = Job::from_str(job_name).map_err(|e| e.to_string())?;
            let exp_string = job_details
                .select(&select_exp)
                .next()
                .ok_or("couldn't find exp")?
                .text()
                .next()
                .ok_or("no exp")?;
            let exp_parts: Vec<Option<u64>> = exp_string
                .split('/')
                .map(|part| match part {
                    "--" => None,
                    some => some.replace(",", "").trim().parse::<u64>().ok(),
                })
                .collect();
            let exp = match &exp_parts[..] {
                &[Some(current), Some(next)] => Some((current, next)),
                _ => None,
            };
            snapshots.push(JobSnapshot { job, level, exp });
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
            race,
            clan,
            gender,
            grand_company,
            grand_company_rank,
            hp,
            mp,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use include_dir::{include_dir, Dir};
    use std::path::Path;

    const PROFILES: &'static str = include_str!("tests/test-profiles.kdl");
    static TEST_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/src/tests");

    #[derive(Debug, knuffel::Decode)]
    struct TestProfile {
        #[knuffel(argument)]
        name: String,
        #[knuffel(property)]
        id: u64,
    }

    #[test]
    /// https://na.finalfantasyxiv.com/lodestone/character/38598907/class_job/
    fn fetch_profile() -> Result<(), String> {
        let profiles = knuffel::parse::<Vec<TestProfile>>("test-profiles.kdl", PROFILES).unwrap();

        for profile in profiles {
            let text_profile = TEST_DIR
                .get_file(format!("{}_profile.html", profile.name))
                .unwrap()
                .contents_utf8()
                .unwrap();
            let text_jobs = TEST_DIR
                .get_file(format!("{}_jobs.html", profile.name))
                .unwrap()
                .contents_utf8()
                .unwrap();
            let profile_html = Html::parse_document(&text_profile);
            let jobs_html = Html::parse_document(&text_jobs);
            insta::assert_debug_snapshot!(
                profile.name,
                Profile::parse(profile.id, profile_html, jobs_html)?
            );
        }

        Ok(())
    }
}

use crate::job::{Job, JobSnapshot, PlayerJobSnapshot};
use scraper::{Element, ElementRef, Html, Selector};
use std::collections::HashMap;
use std::str::FromStr;

/// A player profile snapshot, collected from their lodestone pages.
#[derive(Debug)]
pub struct Profile {
    pub user_id: u64,
    pub free_company: Option<String>,
    pub name: String,
    pub nameday: String,
    pub guardian: String,
    pub city_state: String,
    pub server: String,
    pub race: String,
    pub clan: String,
    pub gender: String,
    pub grand_company: Option<String>,
    pub grand_company_rank: Option<String>,
    pub hp: u64,
    pub mp: u64,
    pub jobs: PlayerJobSnapshot,
}

impl Profile {
    pub const BASE_URL: &'static str = "https://na.finalfantasyxiv.com/lodestone/character";

    pub fn get(user_id: u64) -> Result<Profile, String> {
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
            .collect::<String>();
        let nameday = profile_html
            .select(&select_nameday)
            .next()
            .ok_or("couldn't find nameday")?
            .text()
            .collect::<String>();
        let server = profile_html
            .select(&select_server)
            .next()
            .ok_or("couldn't find server")?
            .text()
            .collect::<String>();
        // some of these elements are optional:
        // - free company
        // - grand company
        // - grand company rank
        // they all have the same CSS class, so we iterate over all matching
        // elements, and then see if we're missing anything important
        let info: Vec<ElementRef> = profile_html.select(&select_profile_info).collect();
        let mut info_blocks: HashMap<&str, String> = HashMap::new();
        for block in info {
            let block_name = block
                .prev_sibling_element()
                .ok_or("failed to find block")?
                .text()
                .collect::<String>();
            match block_name.as_str() {
                "Race/Clan/Gender" => {
                    let (race, clan_gender) = match block.text().collect::<Vec<&str>>()[..] {
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
                    info_blocks.insert("race", race);
                    info_blocks.insert("clan", clan);
                    info_blocks.insert("gender", gender);
                }
                "Guardian" => {
                    let guardian = block.text().collect::<String>();
                    info_blocks.insert("guardian", guardian);
                }
                "City-state" => {
                    let city_state = block.text().collect::<String>();
                    info_blocks.insert("city_state", city_state);
                }
                "Grand Company" => {
                    let (grand_company, grand_company_rank) = match block
                        .text()
                        .collect::<String>()
                        .split('/')
                        .map(|t| t.trim())
                        .collect::<Vec<&str>>()[..]
                    {
                        [grand_company, grand_company_rank] => {
                            (grand_company.to_string(), grand_company_rank.to_string())
                        }
                        _ => Err("failed to parse grand company + grand_company rank")?,
                    };
                    info_blocks.insert("grand_company", grand_company);
                    info_blocks.insert("grand_company_rank", grand_company_rank);
                }
                _ => Err("encountered unknown profile element")?,
            }
        }
        let race = info_blocks
            .get("race")
            .ok_or("failed to find race")?
            .to_string();
        let clan = info_blocks
            .get("clan")
            .ok_or("failed to find clan")?
            .to_string();
        let gender = info_blocks
            .get("gender")
            .ok_or("failed to find gender")?
            .to_string();
        let guardian = info_blocks
            .get("guardian")
            .ok_or("failed to find guardian")?
            .to_string();
        let city_state = info_blocks
            .get("city_state")
            .ok_or("failed to find city_state")?
            .to_string();
        let grand_company = info_blocks.remove("grand_company");
        let grand_company_rank = info_blocks.remove("grand_company_rank");
        let hp = profile_html
            .select(&select_hp)
            .next()
            .ok_or("couldn't find hp")?
            .text()
            .collect::<String>()
            .parse::<u64>()
            .map_err(|e| e.to_string())?;
        let mp = profile_html
            .select(&select_mp)
            .next()
            .ok_or("couldn't find mp")?
            .text()
            .collect::<String>()
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
                .collect::<String>();
            let level = match level_string.as_str() {
                "-" => None,
                some => some.replace(',', "").trim().parse::<u64>().ok(),
            };
            let job_name = job_details
                .select(&select_job_name)
                .next()
                .ok_or("couldn't find job name")?
                .text()
                .collect::<String>();
            let job = Job::from_str(&job_name).map_err(|e| e.to_string())?;
            let exp_string = job_details
                .select(&select_exp)
                .next()
                .ok_or("couldn't find exp")?
                .text()
                .collect::<String>();
            let exp_parts: Vec<Option<u64>> = exp_string
                .split('/')
                .map(|part| match part {
                    "--" => None,
                    some => some.replace(',', "").trim().parse::<u64>().ok(),
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

    /// Lodestone profile references included in test automation.
    ///
    /// To fetch, run:
    ///
    /// ```
    /// $ cargo run -p tasks setup-tests ./src/tests
    /// ```
    #[derive(Debug, knuffel::Decode)]
    struct TestProfile {
        #[knuffel(argument)]
        name: String,
        #[knuffel(property)]
        id: u64,
    }

    /// Inspect cached profiles using snapshot testing.
    ///
    /// Requires all profiles mentioned in `test-profiles.kdl` to be accessible.
    /// If this test fails to find a profile, run:
    ///
    /// ```
    /// $ cargo run -p tasks setup-tests ./src/tests
    /// ```
    #[test]
    fn parse_profiles() -> Result<(), String> {
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

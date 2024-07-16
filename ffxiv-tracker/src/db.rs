use std::path::PathBuf;

use crate::profile::Profile;
use rusqlite::{named_params, Connection};

#[derive(Debug)]
pub struct TrackerDatabase {
    pub path: PathBuf,
}

impl TrackerDatabase {
    pub fn init(&self) -> Result<(), String> {
        let db: Connection =
            Connection::open(&self.path).map_err(|e| format!("failed to open path: {}", e))?;
        db.execute_batch(include_str!("init.sql"))
            .map_err(|e| format!("failed to run init script: {}", e))
    }

    pub fn snapshot(&self, profile: Profile) -> Result<(), String> {
        let db: Connection =
            Connection::open(&self.path).map_err(|e| format!("failed to open path: {}", e))?;
        db.execute(
            "INSERT OR IGNORE INTO profile_snapshots (
                user_id, free_company, name, nameday, guardian, city_state, server,
                race, clan, gender, grand_company, grand_company_rank, hp, mp
            ) VALUES (
                :user_id, :free_company, :name, :nameday, :guardian, :city_state, :server,
                :race, :clan, :gender, :grand_company, :grand_company_rank, :hp, :mp
            )",
            named_params! {
                ":user_id": profile.user_id,
                ":free_company": profile.free_company,
                ":name": profile.name,
                ":nameday": profile.nameday,
                ":guardian": profile.guardian,
                ":city_state": profile.city_state,
                ":server": profile.server,
                ":clan": profile.clan,
                ":race": profile.race,
                ":gender": profile.gender,
                ":grand_company": profile.grand_company,
                ":grand_company_rank": profile.grand_company_rank,
                ":hp": profile.hp,
                ":mp": profile.mp
            },
        )
        .map_err(|e| format!("failed to insert: {}", e))?;
        let snapshot_id = db.last_insert_rowid();
        println!("created snapshot ID: {}", snapshot_id);

        for (job, job_snapshot) in profile.jobs.0 {
            let (current_xp, max_xp) = match job_snapshot.exp {
                Some((current_xp, max_xp)) => (Some(current_xp), Some(max_xp)),
                _ => (None, None),
            };

            db.execute(
                "INSERT INTO experience_snapshots (
                snapshot_id, class_name, xp_level, current_xp, max_xp
            ) VALUES (
                :snapshot_id, :class_name, :xp_level, :current_xp, :max_xp
            )",
                named_params! {
                    ":snapshot_id": snapshot_id,
                    ":class_name": job.to_string(),
                    ":xp_level": job_snapshot.level,
                    ":current_xp": current_xp,
                    ":max_xp": max_xp,
                },
            )
            .map_err(|e| format!("failed to insert experience: {}", e))?;
        }
        Ok(())
    }
}

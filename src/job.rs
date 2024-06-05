use crate::platforms::Platform;
use gitlab::api::{self, projects, projects::jobs::JobScope, Query};
use gitlab::Gitlab;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct Job {
    id: u64,
    status: String,
    tag_list: Vec<String>,
}

impl Job {
    fn get_runner_platform(&self) -> Platform {
        if self.tag_list.iter().any(|tag| tag.contains("linux")) {
            Platform::Linux
        } else if self.tag_list.iter().any(|tag| tag.contains("macos-arm64")) {
            Platform::MacOSArm64
        } else if self.tag_list.iter().any(|tag| tag.contains("macos-x86_64")) {
            Platform::MacOSx86_64
        } else if self.tag_list.iter().any(|tag| tag.contains("windows")) {
            Platform::Windows
        } else {
            Platform::Unknown
        }
    }
}

pub(crate) struct JobRunnerStats {
    job_scopes: Vec<JobScope>,
    counts: Vec<Vec<u64>>,
}

impl JobRunnerStats {
    pub fn new(job_scopes: Vec<JobScope>) -> JobRunnerStats {
        let mut counts: Vec<Vec<u64>> = Vec::new();
        for _i in 0..job_scopes.len() {
            let mut count: Vec<u64> = Vec::new();
            count.resize(Platform::NumberOfRunnerPlatforms as usize, 0);
            counts.push(count);
        }
        return JobRunnerStats {
            job_scopes: job_scopes.clone(),
            counts: counts.clone(),
        };
    }
    pub fn get_jobs(client: &Gitlab, project_path: &String, job_scope: JobScope) -> Vec<Job> {
        let endpoint = projects::jobs::Jobs::builder()
            .project(project_path)
            .scopes([job_scope].iter().cloned())
            .build()
            .unwrap();

        return api::paged(endpoint, api::Pagination::All)
            .query(client)
            .unwrap();
    }

    pub fn compute(&mut self, client: &Gitlab, project_path: &String) {
        for (scope_id, scope) in self.job_scopes.iter().enumerate() {
            for job in JobRunnerStats::get_jobs(&client, project_path, *scope) {
                let runner_platform: Platform = job.get_runner_platform();
                self.counts[scope_id][runner_platform as usize] += 1;
                println!(
                    "id: {}, status: {}, tag_list: {:?}",
                    job.id, job.status, job.tag_list
                );
            }
        }
    }

    pub fn get_number_of_jobs_with_status(&self, job_scope: JobScope, platform: Platform) -> u64 {
        let idx = self.job_scopes.iter().position(|&j| j == job_scope);
        if idx == None {
            return 0;
        } else {
            return self.counts[idx.unwrap()][platform as usize];
        }
    }
}

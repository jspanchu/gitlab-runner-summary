pub mod job;
pub mod platforms;

use gitlab::{api::projects::jobs::JobScope, Gitlab};
use job::JobRunnerStats;
use platforms::Platform;
use std::env;

fn get_argument(short_opt: &str, long_opt: &str) -> Option<String> {
    for (i, arg) in env::args().enumerate() {
        if arg == short_opt || arg == long_opt {
            return env::args().nth(i + 1);
        }
    }
    return None;
}

fn main() {
    let token: String = get_argument("-t", "--token")
        .expect("Please provide personal access token with -t/--token!");
    let project_path: String = get_argument("-p", "--project-path")
        .expect("Please provide project path with -p/--project-path! Ex: vtk/vtk");
    let client = Gitlab::new("gitlab.kitware.com", token).unwrap();
    let job_scopes = [JobScope::Running, JobScope::Pending];
    let mut job_stats = JobRunnerStats::new(job_scopes.to_vec());
    job_stats.compute(&client, &project_path);

    for platform in [
        Platform::Linux,
        Platform::MacOSArm64,
        Platform::MacOSx86_64,
        Platform::Windows,
    ] {
        let pending = job_stats.get_number_of_jobs_with_status(JobScope::Pending, platform);
        let running = job_stats.get_number_of_jobs_with_status(JobScope::Running, platform);

        let mut total_no_of_jobs = 0;
        for job_scope in job_scopes {
            total_no_of_jobs += job_stats.get_number_of_jobs_with_status(job_scope, platform);
        }
        let demand = (pending as f64 / total_no_of_jobs as f64) * 100.00;
        let supply = (running as f64 / total_no_of_jobs as f64) * 100.00;
        println!(
            "platform: {:?} | demand: {:.3} | supply: {:.3} | pending: {} | running: {} | total: {}",
            platform, demand, supply, pending, running, total_no_of_jobs
        );
    }
}

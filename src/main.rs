use rusoto_batch::{Batch, BatchClient, SubmitJobRequest};
use rusoto_core::region::Region;

fn main() {
    let region = Region::ApNortheast1;
    let client = BatchClient::new(region);
    let ret = client.describe_job_definitions(Default::default()).sync();
    // println!("{:?}", ret);
    let ret = client
        .submit_job(SubmitJobRequest {
            job_definition: "first-run-job-definition:1".to_string(),
            job_name: "job-from-rust".to_string(),
            job_queue: "first-run-job-queue".to_string(),
            ..Default::default()
        })
        .sync();
    println!("{:?}", ret);
}

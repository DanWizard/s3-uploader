use aws_config;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::model::ObjectCannedAcl;
use aws_sdk_s3::presigning::config::PresigningConfig;
use aws_sdk_s3::Client;
use dotenv::dotenv;
use std::time::Duration;
use tokio;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    dotenv().ok();
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);

    // SEE WHAT BUCKETS EXIST
    // let resp = client.list_buckets().send().await.unwrap();
    // let buckets = resp.buckets().unwrap_or_default();
    // let num_buckets = buckets.len();
    // for bucket in buckets {
    //     println!("{}", bucket.name().unwrap_or_default());
    // }
    // println!();
    // println!("Found {} buckets.", num_buckets);

    let resp = client
        .list_objects_v2()
        .bucket("freeway-local")
        .send()
        .await
        .unwrap();

    // CREATE URLS FOR PUBLIC OBJECTS IN THE BUCKET
    for object in resp.contents().unwrap_or_default() {
        // GET PRESIGNED OBJECTS PER BUCKET
        // println!("{}", object.key().unwrap_or_default());
        // let expires_in = Duration::from_secs(10);
        // let presigned_request = client
        //     .get_object()
        //     .bucket("freeway-local")
        //     .key(object.key().unwrap())
        //     .presigned(PresigningConfig::expires_in(expires_in).unwrap())
        //     .await
        //     .unwrap();
        // println!("Object URI: {:?}", presigned_request);

        // UPDATE ACL FOR OBJECTS PER BUCKET
        // client
        //     .put_object_acl()
        //     .set_acl(Some(ObjectCannedAcl::PublicRead))
        //     .bucket("freeway-local")
        //     .key(object.key().unwrap())
        //     .send()
        //     .await;

        let endpoint_url = "https://s3.amazonaws.com";
        let bucket = "freeway-local";
        let key = object.key().unwrap();
        let url = format!("{}/{}/{}", endpoint_url, bucket, key);
        println!("Object URL: {}", url);
    }
}

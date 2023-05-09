use aws_lambda_events::event::s3::S3Event;
use aws_lambda_events::s3::S3EventRecord;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use image::ImageError;
use s3::bucket::Bucket;
use s3::creds::Credentials;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb as dynamodb;
    

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
/// - https://github.com/aws-samples/serverless-rust-demo/
async fn function_handler(event: LambdaEvent<S3Event>) -> Result<(), Error> {
    // Extract some useful information from the request
    for record in event.payload.records {
        let _ = process_record(record).await;
    }
    Ok(())
}

async fn process_record(record: S3EventRecord) -> Result<(), Error> {
        //extract fields from event record
        let bucket_name = record.s3.bucket.name.unwrap();
        //string is urlencoded, need to convert
        let object_key = record.s3.object.key.unwrap();
        let object_key = urlencoding::decode(&object_key).expect("UTF-8");

        let region = record.aws_region.unwrap().parse()?;

        //initialize bucket
        let credentials = Credentials::default()?;
        let bucket = Bucket::new(&bucket_name, region, credentials).expect("Unable to connect to bucket");
        
        //Get object
        let mut object_data_stream = bucket.get_object_stream(&object_key).await?;

		let original_filename = format!("/tmp/{}", &object_key);

		let mut async_output_file = tokio::fs::File::create(&original_filename).await.expect("Unable to create file");

		while let Some(chunk) = object_data_stream.bytes().next().await {
			async_output_file.write_all(&chunk).await?;
		}	

		let object = image::io::Reader::open(&original_filename)?.decode()?;

        //Scale image
		let scale_ratio = 0.5;
        let resized = resize_image(&object, &scale_ratio).unwrap();

        // Create new S3 key name from source without the prefix
        let removed_root_folder = get_route_without_root(&object_key);
        let target = format!("resized-rust{}", removed_root_folder);
		println!("Uploading resized image to {}", target);

        //write to fs
		let tmp_target = format!("/tmp/{}", &target);
		resized.save(&tmp_target)?;

        // Upload new image to s3
        let mut file = tokio::fs::File::open(&tmp_target).await?;
        let status_code = bucket.put_object_stream(&mut file, &target).await?;
        println!("Uploaded resized image with status code {}", &status_code);

        put_on_dynamo(&object_key, &target).await?;

        Ok(())
}

async fn put_on_dynamo(original_path: &str, thumbnail_path: &str) -> Result<(), Error> {
	let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
	let config = aws_config::from_env().region(region_provider).load().await;
	let client = dynamodb::Client::new(&config);

	let request = client
		.put_item()
        .table_name("image_metadata")
        .item("id", dynamodb::types::AttributeValue::S(String::from(original_path)))
        .item("fullsize-path", dynamodb::types::AttributeValue::S(String::from(original_path)))
        .item("thumbnail-path", dynamodb::types::AttributeValue::S(String::from(thumbnail_path)));

    let response = request.send().await?;

    println!("Wrote item {:?}", response);

    Ok(())
}

fn get_route_without_root(path: &str) -> &str {
 	let bytes = path.as_bytes();
 	for (i, &item) in bytes.iter().enumerate() {
 		if item == b'/' {
 			return &path[i..];
 		}
 	}
    &path
}


fn resize_image(img: &image::DynamicImage, ratio: &f32) -> Result<image::DynamicImage, ImageError> {
    let old_w = img.width() as f32;
    let old_h = img.height() as f32;
    let new_w = (old_w * ratio).floor();
    let new_h = (old_h * ratio).floor();

    let scaled = img.resize(new_w as u32, new_h as u32, image::imageops::FilterType::Nearest);

    Ok(scaled)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}

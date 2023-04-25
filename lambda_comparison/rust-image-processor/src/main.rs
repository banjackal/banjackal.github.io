use aws_lambda_events::event::s3::S3Event;
use aws_lambda_events::s3::S3EventRecord;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use image::ImageError;
use s3::bucket::Bucket;
use s3::creds::Credentials;
use std::io::Cursor;

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
        let object = bucket.get_object(&object_key).await?;
        // let object = image::load_from_memory(object.bytes()).unwrap();
        let reader = image::io::Reader::new(Cursor::new(object.bytes())).with_guessed_format()?;

        //gets the source format so we can use it in our write
        let object_format = reader.format().unwrap();
        let object = reader.decode()?;

        //Scale image
		let scale_ratio = 0.5;
        let resized = resize_image(&object, &scale_ratio).unwrap();

        // Create new S3 key name from source without the prefix
        let removed_root_folder = get_route_without_root(&object_key);
        let target = format!("resized-rust{}", removed_root_folder);
		println!("Uploading resized image to {}", target);

        //write to bytes
        let mut bytes: Vec<u8> = Vec::new();
		resized.write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::from(object_format))?;

        // Upload new image to s3
        let _uploaded = bucket.put_object(&target, &bytes).await?;
        println!("Uploaded resized image");

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

    let scaled = img.resize(new_w as u32, new_h as u32, image::imageops::FilterType::Lanczos3);

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

## Faster resizing!
[Rust Docs](https://docs.rs/image/latest/image/imageops/enum.FilterType.html)
to the rescue! In our previous example, we were using one of the most
expensive FilterType enum values as our sampling filter for our resize operation.

For the utmost speed (and worst image quality) let's instead use the `Nearest` sampling
filter.

##### __`rust-image-processor/src/main.rs`__
```rust
// --snipped--
fn resize_image(img: &image::DynamicImage, ratio: &f32) -> Result<image::DynamicImage, ImageError> {
    let old_w = img.width() as f32;
    let old_h = img.height() as f32;
    let new_w = (old_w * ratio).floor();
    let new_h = (old_h * ratio).floor();

    let scaled = img.resize(new_w as u32, new_h as u32, image::imageops::FilterType::Nearest);

    Ok(scaled)
}
// --snipped--
```

Now, this should have helped, yet our Lambda function still takes over 5 seconds to cold
start, and is running at around 4 seconds on warm starts with our 2MB cat picture.

One other difference I'm noticing is that the Python lambda is using the underlying filesystem as temporary storage for the downloaded and resized file instead of keeping the bytes in memory. Let's try following the same pattern with our Rust lambda.

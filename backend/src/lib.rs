use std::path::Path;

use lcs_image_diff::compare;

#[derive(Debug)]
pub enum ImageCompareError {
    ImageReadError(image::ImageError),
    ImageCompareError(lcs_image_diff::DecodeError),
    IoError(std::io::Error),
}

impl From<std::io::Error> for ImageCompareError {
    fn from(error: std::io::Error) -> Self {
        ImageCompareError::IoError(error)
    }
}

impl From<lcs_image_diff::DecodeError> for ImageCompareError {
    fn from(error: lcs_image_diff::DecodeError) -> Self {
        ImageCompareError::ImageCompareError(error)
    }
}

impl From<image::ImageError> for ImageCompareError {
    fn from(error: image::ImageError) -> Self {
        ImageCompareError::ImageReadError(error)
    }
}

pub struct ImageCompare {
    before: image::DynamicImage,
    after: image::DynamicImage,
    diff: image::DynamicImage,
}

impl ImageCompare {
    pub fn compare_images<P>(before: P, after: P) -> Result<Self, ImageCompareError>
    where
        P: AsRef<Path>,
    {
        let mut _before = image::open(before)?;
        let mut _after = image::open(after)?;

        let diff = compare(&mut _before, &mut _after, 100.0 / 256.0)?;

        return Ok(ImageCompare {
            before: _before,
            after: _after,
            diff: diff,
        });
    }

    pub fn save<P>(&self, dir: P) -> std::io::Result<()>
    where
        P: AsRef<Path>,
    {
        let _dir = Path::new(dir.as_ref());

        // check if path exist
        // create if it doesn't
        if (!_dir.exists()) {
            std::fs::create_dir_all(_dir)?;
        }

        self.before.save(_dir.join("marked-before.png"))?;
        self.after.save(_dir.join("marked-after.png"))?;
        self.diff.save(_dir.join("diff.png"))?;

        return Ok(());
    }
}

#[cfg(test)]
mod tests {}

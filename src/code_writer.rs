use std::path::Path;
pub struct CodeWriter<P>
where
    P: AsRef<Path>,
{
    filename: P,
}

impl<P: AsRef<Path>> CodeWriter<P> {
    pub fn init(filename: P) -> Self {
        Self { filename }
    }
}

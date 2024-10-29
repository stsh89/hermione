use std::path::Path;
use tracing_appender::{
    non_blocking::WorkerGuard,
    rolling::{RollingFileAppender, Rotation},
};

const MAX_LOG_FILES: usize = 3;
const DEFAULT_ROTATION_POLICY: Rotation = Rotation::HOURLY;
const FILE_NAME_PREFIX: &str = "hermione-logs";

type Result<T> = anyhow::Result<T>;

pub struct Tracer<'a> {
    directory: &'a Path,
}

pub struct NewTracerParameters<'a> {
    pub directory: &'a Path,
}

impl<'a> Tracer<'a> {
    pub fn init_non_blocking(&self) -> Result<WorkerGuard> {
        let file_appender = RollingFileAppender::builder()
            .max_log_files(MAX_LOG_FILES)
            .filename_prefix(FILE_NAME_PREFIX)
            .rotation(DEFAULT_ROTATION_POLICY)
            .build(self.directory)?;

        let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

        tracing_subscriber::fmt()
            .json()
            .with_writer(non_blocking)
            .init();

        Ok(guard)
    }

    pub fn new(parameters: NewTracerParameters<'a>) -> Self {
        let NewTracerParameters { directory } = parameters;

        Self { directory }
    }
}

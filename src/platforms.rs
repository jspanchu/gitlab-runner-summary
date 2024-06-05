#[derive(Copy, Clone, Debug)]
pub(crate) enum Platform {
    Linux,
    MacOSArm64,
    MacOSx86_64,
    Windows,
    Unknown,
    NumberOfRunnerPlatforms,
}

use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum ValidationError {
    #[error("Input URL string is empty")]
    EmptyInput,

    #[error("Input contains null bytes or disallowed control characters")]
    IllegalCharacters,

    #[error("Input begins with a CLI flag prefix ('-' or '--'), rejected to prevent argument injection")]
    ArgumentInjectionAttempt,

    #[error("URL parsing failure: {0}")]
    ParseFailure(String),

    #[error("Unsupported or dangerous URL scheme: '{0}'")]
    DisallowedScheme(String),

    #[error("Missing network host in address: '{0}'")]
    MissingHost(String),
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum RoutingError {
    #[error("Rule pattern '{0}' exceeds maximum permitted length")]
    PatternTooLong(String),

    #[error("Regular expression compilation failed for pattern: {0}")]
    InvalidRegex(String),

    #[error("No target browser found matching ID: {0}")]
    TargetNotFound(String),
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum LaunchError {
    #[error("Target executable path does not exist: {0}")]
    ExecutableNotFound(String),

    #[error("Dangerous or disallowed executable path: {0}")]
    DisallowedExecutable(String),

    #[error("Process spawning failed: {0}")]
    SpawnFailed(String),

    #[error("Alternate launch profile not found: {0}")]
    AlternateProfileNotFound(String),
}

#[derive(Error, Debug)]
pub enum SettingsError {
    #[error("Unable to locate standard configuration directory for current user")]
    NoConfigDirectory,

    #[error("Underlying I/O error: {0}")]
    Io(String),

    #[error("Serialization / Deserialization error: {0}")]
    Serialization(String),
}

use crate::domain::errors::LaunchError;
use crate::infrastructure::traits::ProcessRunner;
use std::path::Path;
use std::process::Command;

#[derive(Default)]
pub struct NativeProcessRunner;

impl NativeProcessRunner {
    const DISALLOWED_BINARIES: &'static [&'static str] = &[
        "sh",
        "bash",
        "zsh",
        "dash",
        "csh",
        "tcsh",
        "cmd.exe",
        "powershell.exe",
        "pwsh.exe",
        "wscript.exe",
        "cscript.exe",
    ];

    pub fn new() -> Self {
        Self
    }
}

impl ProcessRunner for NativeProcessRunner {
    fn run(&self, executable: &str, arguments: &[String]) -> Result<(), LaunchError> {
        let file_name = Path::new(executable)
            .file_name()
            .and_then(|f| f.to_str())
            .unwrap_or("")
            .to_lowercase();

        if Self::DISALLOWED_BINARIES.contains(&file_name.as_str()) {
            return Err(LaunchError::DisallowedExecutable(executable.to_string()));
        }

        if Path::new(executable).is_absolute() && !Path::new(executable).exists() {
            return Err(LaunchError::ExecutableNotFound(executable.to_string()));
        }

        let mut cmd = Command::new(executable);
        cmd.args(arguments);

        cmd.spawn()
            .map_err(|e| LaunchError::SpawnFailed(e.to_string()))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reject_shell_binaries() {
        let runner = NativeProcessRunner::new();
        assert_eq!(
            runner.run("/bin/bash", &[]),
            Err(LaunchError::DisallowedExecutable("/bin/bash".into()))
        );
        assert_eq!(
            runner.run("cmd.exe", &[]),
            Err(LaunchError::DisallowedExecutable("cmd.exe".into()))
        );
    }
}

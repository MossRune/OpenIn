use crate::domain::errors::LaunchError;
use crate::domain::models::BrowserTarget;
use crate::infrastructure::native_process_runner::NativeProcessRunner;
use crate::infrastructure::traits::ProcessRunner;
use std::sync::Arc;

pub struct LaunchService {
    runner: Arc<dyn ProcessRunner>,
}

impl LaunchService {
    pub fn new(runner: Arc<dyn ProcessRunner>) -> Self {
        Self { runner }
    }

    pub fn prepare_arguments(template: Option<&str>, target_url: &str) -> Vec<String> {
        let mut final_args = Vec::new();
        let mut replaced = false;

        if let Some(template_str) = template {
            let tokens = Self::tokenize_arguments(template_str);
            for token in tokens {
                if token.contains("%URL%") {
                    final_args.push(token.replace("%URL%", target_url));
                    replaced = true;
                } else if token.contains("%u") {
                    final_args.push(token.replace("%u", target_url));
                    replaced = true;
                } else if token.contains("%U") {
                    final_args.push(token.replace("%U", target_url));
                    replaced = true;
                } else {
                    final_args.push(token);
                }
            }
        }

        if !replaced {
            final_args.push(target_url.to_string());
        }

        final_args
    }

    pub fn prepare_args(template: Option<&str>, target_url: &str) -> Vec<String> {
        Self::prepare_arguments(template, target_url)
    }

    fn tokenize_arguments(input: &str) -> Vec<String> {
        let mut tokens = Vec::new();
        let mut current = String::new();
        let mut in_quotes = false;
        let mut quote_char = ' ';

        for ch in input.chars() {
            match ch {
                '"' | '\'' if !in_quotes => {
                    in_quotes = true;
                    quote_char = ch;
                }
                c if in_quotes && c == quote_char => {
                    in_quotes = false;
                }
                c if c.is_whitespace() && !in_quotes => {
                    if !current.is_empty() {
                        tokens.push(std::mem::take(&mut current));
                    }
                }
                c => {
                    current.push(c);
                }
            }
        }

        if !current.is_empty() {
            tokens.push(current);
        }

        tokens
    }

    pub fn launch(
        &self,
        target: &BrowserTarget,
        url: &str,
        alternate_profile_id: Option<&str>,
    ) -> Result<(), LaunchError> {
        let base_args = if let Some(alt_id) = alternate_profile_id {
            let alt = target
                .alternate_profiles
                .iter()
                .find(|p| p.id == alt_id)
                .ok_or_else(|| LaunchError::AlternateProfileNotFound(alt_id.to_string()))?;
            Some(alt.arguments.as_str())
        } else {
            target.arguments.as_deref()
        };

        let args = Self::prepare_arguments(base_args, url);
        self.runner.run(&target.executable_path, &args)
    }

    /// Static launch convenience method using the default native runner
    pub fn launch_target(
        target: &BrowserTarget,
        url: &str,
        alternate_profile_id: Option<&str>,
    ) -> Result<(), LaunchError> {
        let runner = Arc::new(NativeProcessRunner::new());
        let service = LaunchService::new(runner);
        service.launch(target, url, alternate_profile_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prepare_arguments() {
        let args =
            LaunchService::prepare_arguments(Some("--private-window %URL%"), "https://example.com");
        assert_eq!(args, vec!["--private-window", "https://example.com"]);

        let args_default = LaunchService::prepare_arguments(None, "https://example.com");
        assert_eq!(args_default, vec!["https://example.com"]);
    }
}

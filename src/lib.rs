pub struct TempEnvVar {
    pub key: String,
    pub initial_value: Option<String>,
}

impl TempEnvVar {
    pub fn new(key: &str) -> Self {
        let initial_value = std::env::var(key).ok();
        std::env::remove_var(key);
        Self {
            key: key.into(),
            initial_value,
        }
    }

    pub fn with(self, value: &str) -> Self {
        std::env::set_var(self.key.as_str(), value);
        self
    }
}

impl Drop for TempEnvVar {
    fn drop(&mut self) {
        match self.initial_value.as_ref() {
            Some(value) => std::env::set_var(self.key.as_str(), value),
            None => std::env::remove_var(self.key.as_str()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn with_non_existing_variable() {
        let name = "MISSINGVAR";
        std::env::remove_var(name);
        let variable = TempEnvVar::new(name);
        assert_eq!(variable.initial_value, None);
        assert_eq!(std::env::var(name).ok(), None);
        let variable = variable.with("SOMETHING");
        assert_eq!(variable.initial_value, None);
        assert_eq!(std::env::var(name).ok(), Some("SOMETHING".into()));
        drop(variable);
        assert_eq!(std::env::var(name).ok(), None);
    }

    #[test]
    fn with_existing_variable() {
        let name = "EXISTINGVAR";
        std::env::set_var(name, "INITIAL");
        let variable = TempEnvVar::new(name);
        assert_eq!(variable.initial_value, Some("INITIAL".into()));
        assert_eq!(std::env::var(name).ok(), None);
        let variable = variable.with("SOMETHING");
        assert_eq!(variable.initial_value, Some("INITIAL".into()));
        assert_eq!(std::env::var(name).ok(), Some("SOMETHING".into()));
        drop(variable);
        assert_eq!(std::env::var(name).ok(), Some("INITIAL".into()));
    }
}

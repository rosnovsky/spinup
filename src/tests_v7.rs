#[cfg(test)]
mod tests {
    use crate::helpers::get_all_configured_apps;
    use crate::structs::{CommonConfig, Config, ManagerConfig, OsConfig, TaskConfig};
    use std::collections::HashMap;

    #[test]
    fn test_v7_aggregation() {
        let mut manager = HashMap::new();
        manager.insert(
            "dnf".to_string(),
            ManagerConfig {
                packages: vec!["git".to_string(), "vim".to_string()],
                flags: vec![],
                depends_on: None,
            },
        );

        let mut tasks = HashMap::new();
        tasks.insert(
            "custom_script".to_string(),
            TaskConfig {
                script: "echo hello".to_string(),
                description: None,
                depends_on: None,
            },
        );

        let os_config = OsConfig {
            description: Some("Test OS".to_string()),
            manager,
            tasks,
            dotfiles: None,
        };

        let common = CommonConfig {
            packages: vec!["common_pkg".to_string()],
        };

        let config = Config {
            version: 7,
            common: Some(common),
            os_entries: HashMap::new(),
        };

        let apps = get_all_configured_apps(&config, &os_config);

        assert!(apps.contains(&"git".to_string()));
        assert!(apps.contains(&"vim".to_string()));
        assert!(apps.contains(&"custom_script".to_string()));
        assert!(apps.contains(&"common_pkg".to_string()));
        assert_eq!(apps.len(), 4);
    }
}

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub beta_mode: bool,
}

impl AppConfig {
    pub fn parse_args() -> Self {
        let args: Vec<String> = std::env::args().collect();
        let beta_mode = args.contains(&"--beta".to_string()) || args.contains(&"beta".to_string());
        
        Self {
            beta_mode
        }
    }
}

pub trait PasswordHash {
    fn generate(&self, password: &str) -> String;
    fn validate(&self, password: &str, haseh: &str) -> bool;
}

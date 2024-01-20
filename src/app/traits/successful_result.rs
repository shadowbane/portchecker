use crate::app::dto::ping_result::SuccessField;

// Common trait for types that can have successful results
pub trait HasSuccessfulResults<T> {
    fn results(&self) -> &Vec<T>;

    fn successful_results(&self) -> Vec<&T> where T: SuccessField {
        self.results().iter().filter(|r| r.success()).collect()
    }
}

use crate::app::dto::ping_result::SuccessField;

// Common trait for types that can have failed results
pub trait HasFailedResults<T> {
    fn results(&self) -> &Vec<T>;

    fn failed_results(&self) -> Vec<&T>
        where
            T: SuccessField,
    {
        self.results().iter().filter(|r| !r.success()).collect()
    }
}

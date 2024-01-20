use crate::app::traits::failed_result::HasFailedResults;
use crate::app::traits::successful_result::HasSuccessfulResults;

pub trait CheckResult<T>: HasSuccessfulResults<T> + HasFailedResults<T> {
    fn average(&self) -> f64;
    fn min(&self) -> f64;
    fn max(&self) -> f64;
    fn failed(&self) -> u8;
    fn received(&self) -> u8;
    fn transmitted(&self) -> u8;
    // fn results(&self) -> &Vec<T>;
}

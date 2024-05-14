use std::time::Duration;
use chrono::{DateTime, Local};

pub trait TimeSensitiveTrait {
    fn get_duration(&self) -> Duration;
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct TimeSensitiveData<T:TimeSensitiveTrait> {
    data: T,

    /// The time when the data was created.
    time: DateTime<Local>,
}

impl<T> TimeSensitiveData<T> where T: TimeSensitiveTrait
{
    /// Create a new TimeSensitiveData instance.
    ///
    /// # Arguments
    ///
    /// * `data`: the time sensitive data.
    ///
    /// returns: TimeSensitiveData<T>
    ///
    /// # Examples
    ///
    /// ```
    /// let data = TimeSensitiveData::new("Hello, world!");
    /// ```
    fn new(data: T) -> Self {
        Self {
            data,
            time: Local::now(),
        }
    }
    
    fn is_expired(&self) -> bool {
        let duration = (Local::now() - self.time).to_std().expect("Failed to convert chrono::Duration to std::Duration");
        return duration > self.data.get_duration();
    }

}

#[derive(Clone, serde::Serialize)]
pub struct Payload {
    pub(crate) message: String,
}
/*
 * Copyright (c) 2021. Aberic - All Rights Reserved.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 * http://www.apache.org/licenses/LICENSE-2.0
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use crate::Time;
use chrono::format::{DelayedFormat, StrftimeItems};
use chrono::{Duration, Local, NaiveDateTime};

impl Time {
    pub fn now() -> Self {
        let now: NaiveDateTime = Local::now().naive_local();
        Time {
            duration: Duration::nanoseconds(now.timestamp_nanos()),
        }
    }

    pub fn from(duration: Duration) -> Self {
        Time { duration }
    }

    /// Formats the combined date and time with the specified format string.
    /// See the [`format::strftime` module](../format/strftime/index.html)
    /// on the supported escape sequences.
    ///
    /// This returns a `DelayedFormat`,
    /// which gets converted to a string only when actual formatting happens.
    /// You may use the `to_string` method to get a `String`,
    /// or just feed it into `print!` and other formatting macros.
    /// (In this way it avoids the redundant memory allocation.)
    ///
    /// A wrong format string does *not* issue an error immediately.
    /// Rather, converting or formatting the `DelayedFormat` fails.
    /// You are recommended to immediately use `DelayedFormat` for this reason.
    ///
    /// # Example
    ///
    /// ```
    /// use comm::Time;
    ///
    /// let time = Time::now();
    /// assert_eq!(time.format("%Y-%m-%d %H:%M:%S").to_string(), "2015-09-05 23:56:04");
    /// assert_eq!(time.format("around %l %p on %b %-d").to_string(), "around 11 PM on Sep 5");
    /// ```
    ///
    /// The resulting `DelayedFormat` can be formatted directly via the `Display` trait.
    ///
    /// ```
    /// # use comm::Time;
    /// # let time = Time::now();
    /// assert_eq!(format!("{}", time.format("%Y-%m-%d %H:%M:%S")), "2015-09-05 23:56:04");
    /// assert_eq!(format!("{}", time.format("around %l %p on %b %-d")), "around 11 PM on Sep 5");
    /// ```
    pub fn format<'a>(&self, fmt: &'a str) -> DelayedFormat<StrftimeItems<'a>> {
        let time_from_stamp = NaiveDateTime::from_timestamp(self.duration.num_seconds(), 0);
        time_from_stamp.format(fmt)
    }

    /// # Example
    /// * `%Y-%m-%d %H:%M:%S`
    /// * `around %l %p on %b %-d`
    pub fn to_string(&self, fmt: &str) -> String {
        self.format(fmt).to_string()
    }

    pub fn duration(&self) -> Duration {
        self.duration.clone()
    }

    pub fn num_nanoseconds(&self) -> Option<i64> {
        self.duration.num_nanoseconds()
    }

    pub fn num_nanoseconds_string(&self) -> Option<String> {
        match self.duration.num_nanoseconds() {
            Some(nano_i64) => Some(nano_i64.to_string()),
            None => None,
        }
    }
}

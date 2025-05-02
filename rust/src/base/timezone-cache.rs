pub mod base {
    /// Time zone redetection indicator for Clear function.
    ///
    /// `Skip` indicates host time zone doesn't have to be redetected.
    /// `Redetect` indicates host time zone should be redetected, and used to set
    /// the default time zone.
    ///
    /// The host time zone detection may require file system access or similar
    /// operations unlikely to be available inside a sandbox. If v8 is run inside a
    /// sandbox, the host time zone has to be detected outside the sandbox
    /// separately.
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum TimeZoneDetection {
        Skip,
        Redetect,
    }

    /// A trait for caching timezone information.
    pub trait TimezoneCache {
        /// Returns the short name of the local timezone (e.g., EST).
        fn local_timezone(&self, time_ms: f64) -> &str;

        /// ES #sec-daylight-saving-time-adjustment
        /// Daylight Saving Time Adjustment
        fn daylight_savings_offset(&self, time_ms: f64) -> f64;

        /// ES #sec-local-time-zone-adjustment
        /// Local Time Zone Adjustment
        ///
        /// <https://github.com/tc39/ecma262/pull/778>
        fn local_time_offset(&self, time_ms: f64, is_utc: bool) -> f64;

        /// Called when the local timezone changes
        fn clear(&mut self, time_zone_detection: TimeZoneDetection);
    }
} // namespace base
use crate::entities::business_opening_hours_interval::BusinessOpeningHoursInterval;
use serde::{Deserialize, Serialize};

///
///API Reference: [link](https://core.telegram.org/bots/api/#businessopeninghours)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct BusinessOpeningHours {
    ///Unique name of the time zone for which the opening hours are defined
    pub time_zone_name: String,

    ///List of time intervals describing business opening hours
    pub opening_hours: Vec<BusinessOpeningHoursInterval>,
}
// Divider: all content below this line will be preserved after code regen

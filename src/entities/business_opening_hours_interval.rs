use serde::{Deserialize, Serialize};

///
///API Reference: [link](https://core.telegram.org/bots/api/#businessopeninghoursinterval)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct BusinessOpeningHoursInterval {
    ///The minute's sequence number in a week, starting on Monday, marking the start of the time interval during which the business is open; 0 - 7 \* 24 \* 60
    pub opening_minute: i64,

    ///The minute's sequence number in a week, starting on Monday, marking the end of the time interval during which the business is open; 0 - 8 \* 24 \* 60
    pub closing_minute: i64,
}
// Divider: all content below this line will be preserved after code regen

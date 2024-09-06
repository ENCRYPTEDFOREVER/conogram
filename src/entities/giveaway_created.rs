use serde::{Deserialize, Serialize};

///This object represents a service message about the creation of a scheduled giveaway.
///
///API Reference: [link](https://core.telegram.org/bots/api/#giveawaycreated)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct GiveawayCreated {
    ///*Optional*. The number of Telegram Stars to be split between giveaway winners; for Telegram Star giveaways only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prize_star_count: Option<i64>,
}
// Divider: all content below this line will be preserved after code regen

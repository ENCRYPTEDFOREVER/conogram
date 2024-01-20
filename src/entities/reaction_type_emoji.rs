use serde::{Deserialize, Serialize};

///The reaction is based on an emoji.
///API Reference: [link](https://core.telegram.org/bots/api/#reactiontypeemoji)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ReactionTypeEmoji {
    ///Reaction emoji. Currently, it can be one of "👍", "👎", "❤", "🔥", "🥰", "👏", "😁", "🤔", "🤯", "😱", "🤬", "😢", "🎉", "🤩", "🤮", "💩", "🙏", "👌", "🕊", "🤡", "🥱", "🥴", "😍", "🐳", "❤‍🔥", "🌚", "🌭", "💯", "🤣", "⚡", "🍌", "🏆", "💔", "🤨", "😐", "🍓", "🍾", "💋", "🖕", "😈", "😴", "😭", "🤓", "👻", "👨‍💻", "👀", "🎃", "🙈", "😇", "😨", "🤝", "✍", "🤗", "🫡", "🎅", "🎄", "☃", "💅", "🤪", "🗿", "🆒", "💘", "🙉", "🦄", "😘", "💊", "🙊", "😎", "👾", "🤷‍♂", "🤷", "🤷‍♀", "😡"
    pub emoji: ReactionTypeEmojiEmoji,
}

///Reaction emoji. Currently, it can be one of "👍", "👎", "❤", "🔥", "🥰", "👏", "😁", "🤔", "🤯", "😱", "🤬", "😢", "🎉", "🤩", "🤮", "💩", "🙏", "👌", "🕊", "🤡", "🥱", "🥴", "😍", "🐳", "❤‍🔥", "🌚", "🌭", "💯", "🤣", "⚡", "🍌", "🏆", "💔", "🤨", "😐", "🍓", "🍾", "💋", "🖕", "😈", "😴", "😭", "🤓", "👻", "👨‍💻", "👀", "🎃", "🙈", "😇", "😨", "🤝", "✍", "🤗", "🫡", "🎅", "🎄", "☃", "💅", "🤪", "🗿", "🆒", "💘", "🙉", "🦄", "😘", "💊", "🙊", "😎", "👾", "🤷‍♂", "🤷", "🤷‍♀", "😡"
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "emoji")]
pub enum ReactionTypeEmojiEmoji {
    #[default]
    /// "👍"
    #[serde(rename = "👍")]
    ThumbsUp,

    /// "👎"
    #[serde(rename = "👎")]
    ThumbsDown,

    /// "❤"
    #[serde(rename = "❤")]
    RedHeart,

    /// "🔥"
    #[serde(rename = "🔥")]
    Fire,

    /// "🥰"
    #[serde(rename = "🥰")]
    SmilingFaceWithHearts,

    /// "👏"
    #[serde(rename = "👏")]
    ClappingHands,

    /// "😁"
    #[serde(rename = "😁")]
    BeamingFaceWithSmilingEyes,

    /// "🤔"
    #[serde(rename = "🤔")]
    ThinkingFace,

    /// "🤯"
    #[serde(rename = "🤯")]
    ExplodingHead,

    /// "😱"
    #[serde(rename = "😱")]
    FaceScreamingInFear,

    /// "🤬"
    #[serde(rename = "🤬")]
    FaceWithSymbolsOnMouth,

    /// "😢"
    #[serde(rename = "😢")]
    CryingFace,

    /// "🎉"
    #[serde(rename = "🎉")]
    PartyPopper,

    /// "🤩"
    #[serde(rename = "🤩")]
    StarStruck,

    /// "🤮"
    #[serde(rename = "🤮")]
    FaceVomiting,

    /// "💩"
    #[serde(rename = "💩")]
    PileOfPoo,

    /// "🙏"
    #[serde(rename = "🙏")]
    FoldedHands,

    /// "👌"
    #[serde(rename = "👌")]
    OkHand,

    /// "🕊"
    #[serde(rename = "🕊")]
    Dove,

    /// "🤡"
    #[serde(rename = "🤡")]
    ClownFace,

    /// "🥱"
    #[serde(rename = "🥱")]
    YawningFace,

    /// "🥴"
    #[serde(rename = "🥴")]
    WoozyFace,

    /// "😍"
    #[serde(rename = "😍")]
    SmilingFaceWithHeartEyes,

    /// "🐳"
    #[serde(rename = "🐳")]
    SpoutingWhale,

    /// "❤‍🔥"
    #[serde(rename = "❤‍🔥")]
    HeartOnFire,

    /// "🌚"
    #[serde(rename = "🌚")]
    NewMoonFace,

    /// "🌭"
    #[serde(rename = "🌭")]
    HotDog,

    /// "💯"
    #[serde(rename = "💯")]
    HundredPoints,

    /// "🤣"
    #[serde(rename = "🤣")]
    RollingOnTheFloorLaughing,

    /// "⚡"
    #[serde(rename = "⚡")]
    HighVoltage,

    /// "🍌"
    #[serde(rename = "🍌")]
    Banana,

    /// "🏆"
    #[serde(rename = "🏆")]
    Trophy,

    /// "💔"
    #[serde(rename = "💔")]
    BrokenHeart,

    /// "🤨"
    #[serde(rename = "🤨")]
    FaceWithRaisedEyebrow,

    /// "😐"
    #[serde(rename = "😐")]
    NeutralFace,

    /// "🍓"
    #[serde(rename = "🍓")]
    Strawberry,

    /// "🍾"
    #[serde(rename = "🍾")]
    BottleWithPoppingCork,

    /// "💋"
    #[serde(rename = "💋")]
    KissMark,

    /// "🖕"
    #[serde(rename = "🖕")]
    MiddleFinger,

    /// "😈"
    #[serde(rename = "😈")]
    SmilingFaceWithHorns,

    /// "😴"
    #[serde(rename = "😴")]
    SleepingFace,

    /// "😭"
    #[serde(rename = "😭")]
    LoudlyCryingFace,

    /// "🤓"
    #[serde(rename = "🤓")]
    NerdFace,

    /// "👻"
    #[serde(rename = "👻")]
    Ghost,

    /// "👨‍💻"
    #[serde(rename = "👨‍💻")]
    ManTechnolodyst,

    /// "👀"
    #[serde(rename = "👀")]
    Eyes,

    /// "🎃"
    #[serde(rename = "🎃")]
    JackOLantern,

    /// "🙈"
    #[serde(rename = "🙈")]
    SeeNoEvilMonkey,

    /// "😇"
    #[serde(rename = "😇")]
    SmilingFaceWithHalo,

    /// "😨"
    #[serde(rename = "😨")]
    FearfulFace,

    /// "🤝"
    #[serde(rename = "🤝")]
    Handshake,

    /// "✍"
    #[serde(rename = "✍")]
    WritingHand,

    /// "🤗"
    #[serde(rename = "🤗")]
    SmilingFaceWithOpenHands,

    /// "🫡"
    #[serde(rename = "🫡")]
    SalutingFace,

    /// "🎅"
    #[serde(rename = "🎅")]
    SantaClaus,

    /// "🎄"
    #[serde(rename = "🎄")]
    ChristmasTree,

    /// "☃"
    #[serde(rename = "☃")]
    Snowman,

    /// "💅"
    #[serde(rename = "💅")]
    NailPolish,

    /// "🤪"
    #[serde(rename = "🤪")]
    ZanyFace,

    /// "🗿"
    #[serde(rename = "🗿")]
    Moai,

    /// "🆒"
    #[serde(rename = "🆒")]
    CoolButton,

    /// "💘"
    #[serde(rename = "💘")]
    HeartWithArrow,

    /// "🙉"
    #[serde(rename = "🙉")]
    HearNoEvilMonkey,

    /// "🦄"
    #[serde(rename = "🦄")]
    Unicorn,

    /// "😘"
    #[serde(rename = "😘")]
    FaceBlowingAKiss,

    /// "💊"
    #[serde(rename = "💊")]
    Pill,

    /// "🙊"
    #[serde(rename = "🙊")]
    SpeakNoEvilMonkey,

    /// "😎"
    #[serde(rename = "😎")]
    SmilingFaceWithSunglasses,

    /// "👾"
    #[serde(rename = "👾")]
    AlienMonster,

    /// "🤷‍♂"
    #[serde(rename = "🤷‍♂")]
    ManShrugging,

    /// "🤷"
    #[serde(rename = "🤷")]
    PersonShrugging,

    /// "🤷‍♀"
    #[serde(rename = "🤷‍♀")]
    WomanShrugging,

    /// "😡"
    #[serde(rename = "😡")]
    EnragedFace,
}
// Divider: all content below this line will be preserved after code regen
impl From<ReactionTypeEmojiEmoji> for ReactionTypeEmoji {
    fn from(value: ReactionTypeEmojiEmoji) -> Self {
        Self { emoji: value }
    }
}

use std::fmt::Display;

pub type MessageEffect = u64;
#[derive(Debug, Clone, Copy)]
#[repr(u64)]
pub enum UsableMessageEffects {
    /// 👍
    ThumbsUp = 5107584321108051014,
    /// 👎
    ThumbsDown = 5104858069142078462,
    /// ❤️
    Heart = 5159385139981059251,
    /// 🔥
    Fire = 5104841245755180586,
    /// 🎉
    PartyPopper = 5046509860389126442,
    /// 💩
    Poop = 5046589136895476101,
}

impl Display for UsableMessageEffects {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&(*self as usize).to_string())
    }
}

impl From<UsableMessageEffects> for u64 {
    fn from(val: UsableMessageEffects) -> Self {
        val as Self
    }
}

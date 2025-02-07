use dashmap::DashMap;

use crate::entities::{
    chat_member::ChatMember, chat_member_updated::ChatMemberUpdated, misc::chat_id::ChatId,
};

#[derive(Debug, Default)]
pub struct ChatMemberCache {
    chat_members_chat_id: DashMap<(i64, i64), ChatMember>,
    chat_members_chat_username: DashMap<(String, i64), ChatMember>,
    username_to_chat_id: DashMap<String, i64>,
}

impl ChatMemberCache {
    pub fn cache_chat_member(&self, chat_id: &ChatId, chat_member: &ChatMember) {
        match chat_id {
            ChatId::Username(username) => {
                self.chat_members_chat_username.insert(
                    (
                        username.trim_start_matches('@').to_string(),
                        chat_member.user().id,
                    ),
                    chat_member.clone(),
                );
            }
            ChatId::Id(id) => {
                self.chat_members_chat_id
                    .insert((*id, chat_member.user().id), chat_member.clone());
            }
        }
    }

    /// Order:
    /// 1. save by id
    /// 2. save username -> int id
    pub fn cache_update(&self, chat_member_update: &ChatMemberUpdated) {
        let chat_member = chat_member_update.new_chat_member.clone();

        if let Some(username) = &chat_member_update.chat.username {
            self.username_to_chat_id.insert(
                username.trim_start_matches('@').to_lowercase(),
                chat_member_update.chat.id,
            );
        }

        self.chat_members_chat_id.insert(
            (chat_member_update.chat.id, chat_member.user().id),
            chat_member,
        );
    }

    /// Order:
    /// 1. get by int id (cuz updates are saved there)
    /// 2. get by username -> int id
    /// 2. get by username
    pub fn get(&self, chat_id: &ChatId, user_id: i64) -> Option<ChatMember> {
        let chat_id = match chat_id {
            ChatId::Username(username) => {
                let username = username.trim_start_matches('@').to_lowercase();
                let chat_id_int = self
                    .username_to_chat_id
                    .get(&username)
                    .map(|chat_id| *chat_id.value());

                if let Some(chat_id_int) = chat_id_int {
                    chat_id_int
                } else if let Some(chat_member) =
                    self.chat_members_chat_username.get(&(username, user_id))
                {
                    return Some(chat_member.value().clone());
                } else {
                    return None;
                }
            }
            ChatId::Id(id) => *id,
        };

        self.chat_members_chat_id
            .get(&(chat_id, user_id))
            .map(|chat_member| chat_member.value().clone())
    }
}

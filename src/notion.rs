pub struct NotionCommand {
    pub database_id: String,
    pub notify_user_id: String,
    pub notion_api_key: String,
}

impl NotionCommand {
    pub fn build(database_id: String, notify_user_id: String, notion_api_key: String) -> Self {
        Self {
            database_id,
            notify_user_id,
            notion_api_key,
        }
    }
}

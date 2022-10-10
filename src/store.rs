use rand::Rng;
use worker::Date;

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
abcdefghijklmnopqrstuvwxyz\
0123456789";
const GUID_LEN: usize = 30;

pub struct StoreSchema {
    pub name: String,
    pub rss_url: String,
    pub tags: Vec<String>,
    pub description: String,
    pub read: bool,
    pub guid: String,
    pub link: String,
    pub publishDate: Date,
}

impl StoreSchema {
    pub fn new(
        name: impl Into<String>,
        rss_url: impl Into<String>,
        tags: Vec<&str>,
        description: impl Into<String>,
        link: impl Into<String>,
        publishDate: Date,
    ) -> Self {
        let mut rng = rand::thread_rng();

        let guid: String = (0..GUID_LEN)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect();
        Self {
            name: name.into(),
            rss_url: rss_url.into(),
            tags: tags.iter().map(|&tag| tag.into()).collect(),
            description: description.into(),
            read: false,
            guid,
            link: link.into(),
            publishDate,
        }
    }
}

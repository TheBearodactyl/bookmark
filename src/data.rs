use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct BookmarkData {
	pub bookmarks: Vec<Bookmark>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Bookmark {
	pub name: String,
	pub path: String,
	pub desc: String
}
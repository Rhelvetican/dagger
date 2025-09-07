use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SmodsMetadata {
    author: Vec<String>,
    badge_colour: Option<String>,
    badge_text_colour: Option<String>,
    conflicts: Vec<String>,
    dependencies: Vec<String>,
    description: String,
    display_name: Option<String>,
    dump_loc: bool,
    id: String,
    main_file: String,
    name: String,
    prefix: String,
    priority: Option<i64>,
    provides: Vec<String>,
    version: String,
}

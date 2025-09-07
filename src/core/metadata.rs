use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct SmodsMetadata<'a> {
    author: Vec<&'a str>,
    badge_colour: Option<&'a str>,
    badge_text_colour: Option<&'a str>,
    conflicts: Vec<&'a str>,
    dependencies: Vec<&'a str>,
    description: &'a str,
    display_name: Option<&'a str>,
    dump_loc: bool,
    id: &'a str,
    main_file: &'a str,
    name: &'a str,
    prefix: &'a str,
    priority: Option<i64>,
    provides: Vec<&'a str>,
    version: &'a str,
}

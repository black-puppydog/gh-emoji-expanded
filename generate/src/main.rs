use serde_derive;

use std::collections::HashSet;
use std::env;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

#[derive(serde_derive::Deserialize)]
struct GithubEmoji {
    emoji: Option<String>,
    aliases: Vec<String>,
}

#[derive(serde_derive::Deserialize)]
struct EmojiDbEntry {
    unified: String,
    short_names: Vec<String>,
}

fn generate_genmoji_shortcodes(emoji_file: &Path) -> impl Iterator<Item = (String, String)> {
    let source = fs::read(emoji_file).expect(&format!("Can't load {}", emoji_file.display()));
    let github_emojis: Vec<GithubEmoji> = serde_json::from_slice(&source).unwrap();

    github_emojis
        .into_iter()
        .filter_map(|e| {
            if let Some(unicode_emoji) = &e.emoji {
                let code = format!("\"{}\"", unicode_emoji);
                Some(e.aliases.into_iter().map(move |name| (name, code.clone())))
            } else {
                None
            }
        })
        .flatten()
}

fn generate_emoji_db_shortcodes(emoji_file: &Path) -> impl Iterator<Item = (String, String)> {
    let source = fs::read(emoji_file).expect(&format!("Can't load {}", emoji_file.display()));
    let db_emojis: Vec<EmojiDbEntry> = serde_json::from_slice(&source).unwrap();

    db_emojis
        .into_iter()
        .map(|e| {
            let unicode_emoji: String = e
                .unified
                .split("-")
                .map(|s| format!("\\u{{{}}}", s))
                .collect();
            let code = format!("\"{}\"", unicode_emoji);
            e.short_names
                .into_iter()
                .map(move |name| (name, code.clone()))
        })
        .flatten()
}

fn main() {
    let root = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let parent = root.parent().unwrap();
    let dest = parent.join("src").join("data_generated.rs");

    let db_emojis: Vec<(String, String)> =
        generate_emoji_db_shortcodes(&parent.join("emoji_pretty.json")).collect();
    let genmoji_emojis: Vec<(String, String)> =
        generate_genmoji_shortcodes(&parent.join("gemoji/db/emoji.json")).collect();

    let all_emojis: Vec<(String, String)> = db_emojis
        .clone()
        .into_iter()
        .chain(genmoji_emojis.clone().into_iter())
        .collect();

    let mut file = BufWriter::new(File::create(&dest).unwrap());

    generate_code(&mut file, all_emojis.into_iter());
}

fn generate_code(file: &mut BufWriter<File>, emojis: impl Iterator<Item = (String, String)>) {
    write!(file, "/// Compile time generated lookup table for emoji.\n").unwrap();
    write!(file, "/// \n").unwrap();
    write!(file, "/// Taken from https://github.com/github/gemoji\n").unwrap();
    write!(
        file,
        "pub static EMOJI: phf::Map<&'static str, &'static str> = "
    )
    .unwrap();
    let mut m = phf_codegen::Map::new();

    let mut already_added: HashSet<String> = HashSet::new();

    for (shortcode, emoji) in emojis {
        if already_added.contains(&shortcode) {
            continue;
        }
        already_added.insert(shortcode.clone());
        m.entry(shortcode, &emoji);
    }

    let m = m.build();
    write!(file, "{};\n", m).unwrap();
}

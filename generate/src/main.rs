use serde_derive;

use std::collections::HashSet;
use std::env;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::PathBuf;

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

fn main() {
    let root = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let parent = root.parent().unwrap();
    let dest = parent.join("src").join("data_generated.rs");
    let mut file = BufWriter::new(File::create(&dest).unwrap());

    write!(
        &mut file,
        "/// Compile time generated lookup table for emoji.\n"
    )
    .unwrap();
    write!(&mut file, "/// \n").unwrap();
    write!(
        &mut file,
        "/// Taken from https://github.com/github/gemoji\n"
    )
    .unwrap();
    write!(
        &mut file,
        "pub static EMOJI: phf::Map<&'static str, &'static str> = "
    )
    .unwrap();
    let mut m = phf_codegen::Map::new();

    let source =
        fs::read(parent.join("emoji_pretty.json")).expect("Can't load ../emoji_pretty.json");
    let db_emojis: Vec<EmojiDbEntry> = serde_json::from_slice(&source).unwrap();

    let mut already_added: HashSet<String> = HashSet::new();

    for e in db_emojis {
        let unicode_emoji: String = e
            .unified
            .split("-")
            .map(|s| format!("\\u{{{}}}", s))
            .collect();
        let code = format!("\"{}\"", unicode_emoji);
        for name in e.short_names {
            m.entry(name.clone(), &code);
            already_added.insert(name);
        }
    }

    let source = fs::read(parent.join("gemoji/db/emoji.json"))
        .expect("Can't load ../gemoji/db/emoji.json. Try git submodule update --init");
    let github_emojis: Vec<GithubEmoji> = serde_json::from_slice(&source).unwrap();

    for e in github_emojis {
        if let Some(unicode_emoji) = &e.emoji {
            let code = format!("\"{}\"", unicode_emoji);
            for name in e.aliases {
                if !already_added.contains(&name) {
                    m.entry(name, &code);
                }
            }
        }
    }

    let m = m.build();
    write!(&mut file, "{};\n", m).unwrap();
}

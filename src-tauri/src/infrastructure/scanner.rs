use crate::domain::track::Track;
use crate::infrastructure::metadata::read_metadata;
use walkdir::WalkDir;

const AUDIO_EXTS: &[&str] = &[
    "flac", "alac", "wav", "aiff", "ape", "tak", "wv", "wavpack",
    "dsf", "dff", "dsd",
    "mp3", "aac", "ogg", "opus", "m4a",
];

fn is_audio(path: &std::path::Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .map(|e| AUDIO_EXTS.contains(&e.to_lowercase().as_str()))
        .unwrap_or(false)
}

pub fn scan_directory(dir: &str) -> Result<Vec<Track>, String> {
    let mut tracks = Vec::new();
    let mut errors = Vec::new();

    for entry in WalkDir::new(dir).follow_links(true).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if !path.is_file() || !is_audio(path) {
            continue;
        }
        match read_metadata(path) {
            Ok(track) => tracks.push(track),
            Err(e) => errors.push(format!("{}: {}", path.display(), e)),
        }
    }

    tracks.sort_by(|a, b| {
        a.artist.cmp(&b.artist)
            .then_with(|| a.album.cmp(&b.album))
            .then_with(|| a.disc_number.cmp(&b.disc_number))
            .then_with(|| a.track_number.cmp(&b.track_number))
    });

    if !errors.is_empty() {
        eprintln!("Scan errors:\n{}", errors.join("\n"));
    }

    Ok(tracks)
}

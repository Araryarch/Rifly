use crate::domain::track::Track;
use base64::Engine;
use lofty::config::WriteOptions;
use lofty::file::{AudioFile, TaggedFileExt};
use lofty::read_from_path;
use lofty::tag::{ItemKey, Accessor, ItemValue, TagItem};
use std::fs::OpenOptions;
use std::path::Path;

pub fn read_metadata(path: &Path) -> Result<Track, String> {
    let path_str = path.to_string_lossy().to_string();
    let ext = path.extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    let tagged = read_from_path(path).map_err(|e| format!("Read: {e}"))?;
    let properties = tagged.properties();
    let tag = tagged.tags().first().ok_or("No tags")?;

    let title = tag.title().unwrap_or_default().to_string();
    let artist = tag.artist().unwrap_or_default().to_string();
    let album = tag.album().unwrap_or_default().to_string();
    let album_artist = tag.get_string(ItemKey::AlbumArtist).unwrap_or_default().to_string();
    let genre = tag.genre().unwrap_or_default().to_string();
    let year = tag.date().map(|d| d.year).unwrap_or(0) as i32;

    let track_number = tag.track().unwrap_or(0) as i32;
    let disc_number = tag.get_string(ItemKey::DiscNumber)
        .and_then(|s| s.split('/').next()?.parse().ok())
        .unwrap_or(1);

    let duration = properties.duration().as_secs_f64();
    let sample_rate = properties.sample_rate().unwrap_or(0) as i32;
    let bit_depth = properties.bit_depth().unwrap_or(0) as i32;
    let channels = properties.channels().unwrap_or(2) as i32;

    let replaygain_track = tag.get_string(ItemKey::ReplayGainTrackGain)
        .and_then(|s| s.trim().trim_end_matches(" dB").parse().ok())
        .unwrap_or(0.0);
    let replaygain_album = tag.get_string(ItemKey::ReplayGainAlbumGain)
        .and_then(|s| s.trim().trim_end_matches(" dB").parse().ok())
        .unwrap_or(0.0);

    let has_artwork = !tag.pictures().is_empty();

    let composers: Vec<String> = tag.get_strings(ItemKey::Composer).map(|s| s.to_string()).collect();
    let genres: Vec<String> = if genre.is_empty() {
        tag.get_strings(ItemKey::Genre).map(|s| s.to_string()).collect()
    } else {
        vec![genre]
    };

    Ok(Track {
        id: 0,
        path: path_str,
        title: if title.is_empty() {
            path.file_stem().unwrap_or_default().to_string_lossy().to_string()
        } else { title },
        artist,
        album,
        album_artist,
        composers,
        genres,
        track_number,
        disc_number,
        duration,
        sample_rate,
        bit_depth,
        channels,
        format: ext,
        year,
        has_artwork,
        replaygain_track,
        replaygain_album,
    })
}

pub fn read_cover_art(path: &Path) -> Result<String, String> {
    let tagged = read_from_path(path).map_err(|e| format!("Read: {e}"))?;
    let tag = tagged.tags().first().ok_or("No tags")?;
    let picture = tag.pictures().first().ok_or("No cover art")?;
    let data = picture.data();
    let mime = picture.mime_type().map(|m| m.as_str()).unwrap_or("image/jpeg");
    let b64 = base64::engine::general_purpose::STANDARD.encode(data);
    Ok(format!("data:{};base64,{}", mime, b64))
}

pub fn write_metadata(
    path: &Path,
    title: Option<String>,
    artist: Option<String>,
    album: Option<String>,
    album_artist: Option<String>,
    genre: Option<String>,
    year: Option<i32>,
    track_number: Option<i32>,
) -> Result<(), String> {
    let mut tagged = read_from_path(path).map_err(|e| format!("Read: {e}"))?;

    let tag = tagged.first_tag_mut().ok_or("No tags")?;

    if let Some(v) = title { tag.set_title(v); }
    if let Some(v) = artist { tag.set_artist(v); }
    if let Some(v) = album { tag.set_album(v); }
    if let Some(v) = album_artist {
        tag.insert(TagItem::new(ItemKey::AlbumArtist, ItemValue::Text(v)));
    }
    if let Some(v) = genre { tag.set_genre(v); }
    if let Some(v) = year {
        tag.insert(TagItem::new(ItemKey::Year, ItemValue::Text(v.to_string())));
    }
    if let Some(v) = track_number { tag.set_track(v as u32);     }

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(path)
        .map_err(|e| format!("Open: {e}"))?;

    tagged.save_to(&mut file, WriteOptions::new()).map_err(|e| format!("Save: {e}"))
}

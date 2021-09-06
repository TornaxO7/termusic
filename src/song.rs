use crate::player::gst::GSTPlayer;
/**
 * MIT License
 *
 * termusic - Copyright (c) 2021 Larry Hao
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
use crate::songtag::lrc::Lyric;
use anyhow::{anyhow, Result};
use humantime::{format_duration, FormattedDuration};
use id3::frame::{Lyrics, Picture, PictureType};
use metaflac::Tag as FlacTag;
// use lofty::{self, AudioTag, MimeType, Picture, Tag as LoftyTag};
use mp4ameta::{Img, ImgFmt};
// use std::cmp::Ordering;
use std::ffi::OsStr;
use std::fs::rename;
use std::path::Path;
use std::str::FromStr;
use std::time::Duration;

#[derive(Clone)]
pub struct Song {
    /// Artist of the song
    pub artist: Option<String>,
    /// Album of the song
    pub album: Option<String>,
    /// Title of the song
    pub title: Option<String>,
    /// File path to the song
    pub file: Option<String>,
    /// Duration of the song
    pub duration: Duration,
    /// Name of the song
    pub name: Option<String>,
    /// Extension of the song
    pub ext: Option<String>,
    // / uslt lyrics
    pub lyric_frames: Vec<Lyrics>,
    pub lyric_selected: usize,
    pub parsed_lyric: Option<Lyric>,
    // pub lyrics: Option<String>,
    pub picture: Vec<Picture>,
}

impl Song {
    /// Optionally return the artist of the song
    /// If `None` it wasn't able to read the tags
    pub fn artist(&self) -> Option<&str> {
        match self.artist.as_ref() {
            Some(artist) => Some(artist),
            None => None,
        }
    }
    /// Optionally return the song's album
    /// If `None` failed to read the tags
    pub fn album(&self) -> Option<&str> {
        match self.album.as_ref() {
            Some(album) => Some(album),
            None => None,
        }
    }
    /// Optionally return the title of the song
    /// If `None` it wasn't able to read the tags
    pub fn title(&self) -> Option<&str> {
        match self.title.as_ref() {
            Some(title) => Some(title),
            None => None,
        }
    }

    pub fn file(&self) -> Option<&str> {
        match self.file.as_ref() {
            Some(file) => Some(file),
            None => None,
        }
    }

    pub fn duration(&self) -> FormattedDuration {
        format_duration(self.duration)
    }
    pub fn update_duration(&self) {
        if let Some(s) = &self.file() {
            if let Some(ext) = &self.ext {
                match ext.as_str() {
                    "mp3" => {
                        let mut id3_tag = id3::Tag::new();
                        if let Ok(t) = id3::Tag::read_from_path(s) {
                            id3_tag = t;
                        }

                        let duration_player = GSTPlayer::duration(s);
                        id3_tag.remove_duration();
                        id3_tag.set_duration(duration_player.mseconds() as u32);
                        let _ = id3_tag.write_to_path(s, id3::Version::Id3v24);
                    }
                    &_ => {}
                }
            }
        }
    }

    pub fn save_tag(&mut self) -> Result<()> {
        if let Some(ext) = &self.ext {
            match ext.as_str() {
                "mp3" => {
                    let mut id3_tag = id3::Tag::default();
                    if let Some(file) = self.file() {
                        if let Ok(t) = id3::Tag::read_from_path(file) {
                            id3_tag = t;
                        }
                    }

                    id3_tag.set_artist(self.artist().unwrap_or(&String::from("Unknown Artist")));
                    id3_tag.set_title(self.title().unwrap_or(&String::from("Unknown Title")));
                    id3_tag.set_album(self.album().unwrap_or(&String::from("Unknown Album")));
                    id3_tag.remove_all_lyrics();

                    if !self.lyric_frames.is_empty() {
                        let lyric_frames = self.lyric_frames.to_owned();
                        for l in lyric_frames {
                            id3_tag.add_lyrics(l);
                        }
                    }

                    for p in self.picture.iter() {
                        id3_tag.add_picture(p.to_owned());
                    }

                    if let Some(file) = self.file() {
                        id3_tag.write_to_path(file, id3::Version::Id3v24)?;
                    }
                }

                "m4a" => {
                    let mut m4a_tag = mp4ameta::Tag::default();
                    if let Some(file) = self.file() {
                        if let Ok(t) = mp4ameta::Tag::read_from_path(file) {
                            m4a_tag = t;
                        }
                    }

                    m4a_tag.set_artist(self.artist().unwrap_or(&String::from("Unknown Artist")));
                    m4a_tag.set_title(self.title().unwrap_or(&String::from("Unknown Title")));
                    m4a_tag.set_album(
                        self.album
                            .as_ref()
                            .unwrap_or(&String::from("Unknown Album")),
                    );
                    m4a_tag.remove_lyrics();

                    if !self.lyric_frames.is_empty() {
                        let lyric_frames = self.lyric_frames.to_owned();
                        for l in lyric_frames {
                            m4a_tag.set_lyrics(l.text);
                        }
                    }

                    for p in self.picture.iter() {
                        let fmt = match p.mime_type.as_str() {
                            "image/jpeg" => ImgFmt::Jpeg,
                            "image/bmp" => ImgFmt::Bmp,
                            "image/Png" => ImgFmt::Png,
                            &_ => ImgFmt::Jpeg,
                        };

                        let img = Img {
                            data: p.data.to_owned(),
                            fmt,
                        };

                        m4a_tag.set_artwork(img);
                    }

                    if let Some(file) = self.file() {
                        let _ = m4a_tag
                            .write_to_path(file)
                            .map_err(|e| anyhow!("write m4a tag error {:?}", e))?;
                    }
                }
                "flac" => {
                    let mut flac_tag = FlacTag::default();
                    if let Some(file) = self.file() {
                        if let Ok(t) = FlacTag::read_from_path(file) {
                            flac_tag = t;
                        }
                    }

                    flac_tag.set_vorbis(
                        "Artist",
                        vec![self.artist().unwrap_or(&String::from("Unknown Artist"))],
                    );
                    flac_tag.set_vorbis(
                        "Title",
                        vec![self.title().unwrap_or(&String::from("Unknown Title"))],
                    );
                    flac_tag.set_vorbis(
                        "Album",
                        vec![self
                            .album
                            .as_ref()
                            .unwrap_or(&String::from("Unknown Album"))],
                    );
                    flac_tag.remove_vorbis("Lyrics");

                    if !self.lyric_frames.is_empty() {
                        let lyric_frames = self.lyric_frames.to_owned();
                        for l in lyric_frames {
                            flac_tag.set_vorbis("Lyrics", vec![l.text]);
                        }
                    }

                    let pictures = self.picture.clone();
                    for p in pictures.into_iter() {
                        flac_tag.add_picture(
                            p.mime_type,
                            metaflac::block::PictureType::Other,
                            p.data,
                        );
                    }

                    if let Some(file) = self.file() {
                        let _ = flac_tag
                            .write_to_path(file)
                            .map_err(|e| anyhow!("write m4a tag error {:?}", e))?;
                    }
                }

                &_ => {}
            }
        }

        self.rename_by_tag()?;

        Ok(())
    }

    pub fn rename_by_tag(&mut self) -> Result<()> {
        let new_name = format!(
            "{}-{}.{}",
            self.artist().unwrap_or(&"Unknown Artist".to_string()),
            self.title().unwrap_or(&"Unknown Title".to_string()),
            self.ext.as_ref().unwrap_or(&"mp3".to_string()),
        );
        let new_name_path: &Path = Path::new(new_name.as_str());
        if let Some(file) = self.file() {
            let p_old: &Path = Path::new(file);
            if let Some(p_prefix) = p_old.parent() {
                let p_new = p_prefix.join(new_name_path);
                rename(p_old, &p_new)?;
                self.file = Some(String::from(p_new.to_string_lossy()));
                self.name = p_new
                    .file_name()
                    .and_then(OsStr::to_str)
                    .map(|x| x.to_string());
            }
        }
        Ok(())
    }

    pub fn set_lyric(&mut self, lyric_str: &str, lang_ext: &str) {
        let mut lyric_frames = self.lyric_frames.to_owned();
        match self.lyric_frames.get(self.lyric_selected) {
            Some(lyric_frame) => {
                lyric_frames.remove(self.lyric_selected);
                lyric_frames.insert(
                    self.lyric_selected,
                    Lyrics {
                        text: lyric_str.to_string(),
                        ..lyric_frame.to_owned()
                    },
                );
            }
            None => {
                lyric_frames.push(Lyrics {
                    lang: "eng".to_string(),
                    description: lang_ext.to_string(),
                    text: lyric_str.to_string(),
                });
            }
        }
        self.lyric_frames = lyric_frames;
    }

    pub fn set_photo(&mut self, picture: Picture) {
        self.picture.clear();
        self.picture.push(picture);
    }
}

impl FromStr for Song {
    type Err = anyhow::Error;
    // type Err = std::string::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let p: &Path = Path::new(s);
        let ext = p.extension().and_then(OsStr::to_str);

        match ext {
            Some("mp3") => {
                let name = p.file_name().and_then(OsStr::to_str).map(|x| x.to_string());

                let id3_tag = match id3::Tag::read_from_path(s) {
                    Ok(tag) => tag,
                    Err(_) => {
                        let mut t = id3::Tag::new();
                        let p: &Path = Path::new(s);
                        if let Some(p_base) = p.file_stem() {
                            t.set_title(p_base.to_string_lossy());
                        }
                        let _ = t.write_to_path(p, id3::Version::Id3v24);
                        t
                    }
                };

                let artist: Option<String> = id3_tag.artist().map(String::from);
                let album: Option<String> = id3_tag.album().map(String::from);
                let title: Option<String> = id3_tag.title().map(String::from);
                let mut lyrics: Vec<Lyrics> = Vec::new();
                for l in id3_tag.lyrics().cloned() {
                    lyrics.push(l);
                }
                lyrics.sort_by_cached_key(|a| a.description.to_owned());

                let mut parsed_lyric: Option<Lyric> = None;
                if !lyrics.is_empty() {
                    parsed_lyric = match Lyric::from_str(lyrics[0].text.as_ref()) {
                        Ok(l) => Some(l),
                        Err(_) => None,
                    };
                }

                let mut picture: Vec<Picture> = Vec::new();
                for p in id3_tag.pictures().cloned() {
                    picture.push(p);
                }

                let duration = id3_tag
                    .duration()
                    .map(|d| Duration::from_millis(d.into()))
                    .unwrap_or_else(|| Duration::from_secs(0));

                let file = Some(String::from(s));

                Ok(Self {
                    artist,
                    album,
                    title,
                    file,
                    duration,
                    name,
                    ext: Some("mp3".to_string()),
                    lyric_frames: lyrics,
                    lyric_selected: 0,
                    parsed_lyric,
                    picture,
                })
            }
            Some("m4a") => {
                let name = p.file_name().and_then(OsStr::to_str).map(|x| x.to_string());

                let m4a_tag = match mp4ameta::Tag::read_from_path(s) {
                    Ok(t) => t,
                    Err(_) => {
                        let mut t = mp4ameta::Tag::default();
                        let p: &Path = Path::new(s);
                        if let Some(p_base) = p.file_stem() {
                            t.set_title(p_base.to_string_lossy());
                        }
                        let _ = t.write_to_path(p);
                        t
                    }
                };

                let artist: Option<String> = m4a_tag.artist().map(String::from);
                let album: Option<String> = m4a_tag.album().map(String::from);
                let title: Option<String> = m4a_tag.title().map(String::from);

                let lyrics = m4a_tag.lyrics().map(String::from);
                let mut parsed_lyric: Option<Lyric> = None;
                if let Some(l) = &lyrics {
                    parsed_lyric = match Lyric::from_str(l) {
                        Ok(l) => Some(l),
                        Err(_) => None,
                    }
                }

                let mut lyric_frames: Vec<Lyrics> = Vec::new();
                if let Some(s) = lyrics {
                    lyric_frames.push(Lyrics {
                        lang: String::from("chi"),
                        description: String::from("Termusic"),
                        text: s,
                    });
                };

                let mut picture: Vec<Picture> = Vec::new();
                if let Some(artwork) = m4a_tag.artwork() {
                    let fmt = match artwork.fmt {
                        ImgFmt::Bmp => "image/bmp",
                        ImgFmt::Jpeg => "image/jpeg",
                        ImgFmt::Png => "image/png",
                    };
                    picture.push(Picture {
                        mime_type: fmt.to_string(),
                        picture_type: PictureType::Other,
                        description: "some image".to_string(),
                        data: artwork.data.to_vec(),
                    });
                }

                let duration = m4a_tag.duration().unwrap_or_else(|| Duration::from_secs(0));

                let file = Some(String::from(s));
                Ok(Self {
                    artist,
                    album,
                    title,
                    file,
                    duration,
                    name,
                    ext: Some("m4a".to_string()),
                    lyric_frames,
                    lyric_selected: 0,
                    parsed_lyric,
                    picture,
                })
            }
            Some("flac") => {
                let name = p.file_name().and_then(OsStr::to_str).map(|x| x.to_string());

                let flac_tag = match FlacTag::read_from_path(s) {
                    Ok(t) => t,
                    Err(_) => {
                        let mut t = FlacTag::default();
                        let p: &Path = Path::new(s);
                        if let Some(p_base) = p.file_stem() {
                            // t.set_title(p_base.to_string_lossy());
                            t.set_vorbis("Title", vec![p_base.to_string_lossy()]);
                        }
                        let _ = t.write_to_path(p);
                        t
                    }
                };

                let artist: Option<String>;
                let a_vec = flac_tag.get_vorbis("Artist");
                match a_vec {
                    Some(a_vec) => {
                        let mut a_string = String::new();
                        for a in a_vec.into_iter() {
                            a_string.push_str(a);
                        }
                        artist = Some(a_string);
                    }
                    None => artist = None,
                }

                let album: Option<String>;
                let album_vec = flac_tag.get_vorbis("Album");
                match album_vec {
                    Some(album_vec) => {
                        let mut album_string = String::new();
                        for a in album_vec.into_iter() {
                            album_string.push_str(a);
                        }
                        album = Some(album_string);
                    }
                    None => album = None,
                }

                let title: Option<String>;
                let title_vec = flac_tag.get_vorbis("Title");
                match title_vec {
                    Some(title_vec) => {
                        let mut title_string = String::new();
                        for t in title_vec.into_iter() {
                            title_string.push_str(t);
                        }
                        title = Some(title_string);
                    }
                    None => title = None,
                }

                let mut lyric_frames: Vec<Lyrics> = vec![];

                let lyric_vec = flac_tag.get_vorbis("Lyrics");
                if let Some(l_vec) = lyric_vec {
                    for l in l_vec.into_iter() {
                        lyric_frames.push(Lyrics {
                            lang: "eng".to_string(),
                            description: "termusic".to_string(),
                            text: l.to_string(),
                        })
                    }
                }

                let mut parsed_lyric: Option<Lyric> = None;
                if let Some(l) = lyric_frames.get(0) {
                    parsed_lyric = match Lyric::from_str(&l.text) {
                        Ok(l) => Some(l),
                        Err(_) => None,
                    }
                }

                let mut picture: Vec<Picture> = Vec::new();
                let picture_vec = flac_tag.pictures();
                for p in picture_vec.into_iter() {
                    picture.push(Picture {
                        mime_type: p.mime_type.clone(),
                        picture_type: PictureType::Other,
                        description: "some image".to_string(),
                        data: p.data.to_vec(),
                    })
                }

                let mut duration = Duration::from_secs(0);
                let stream_info = flac_tag.get_streaminfo();
                if let Some(s) = stream_info {
                    let secs = s.total_samples.checked_div(s.sample_rate as u64);
                    if let Some(s) = secs {
                        duration = Duration::from_secs(s);
                    }
                }
                // let duration = flac_tag.get_streaminfo().and_then();
                //     .duration()
                //     .unwrap_or_else(|| Duration::from_secs(0));

                let file = Some(String::from(s));
                Ok(Self {
                    artist,
                    album,
                    title,
                    file,
                    duration,
                    name,
                    ext: Some("flac".to_string()),
                    lyric_frames,
                    lyric_selected: 0,
                    parsed_lyric,
                    picture,
                })
            }
            _ => {
                let artist = Some(String::from("Not Support?"));
                let album = Some(String::from("Not Support?"));
                let title = Some(String::from(s));
                let file = Some(String::from(s));
                let duration = Duration::from_secs(0);
                let name = Some(String::from(""));
                let parsed_lyric: Option<Lyric> = None;
                let lyric_frames: Vec<Lyrics> = Vec::new();
                let picture: Vec<Picture> = Vec::new();
                Ok(Self {
                    artist,
                    album,
                    title,
                    file,
                    duration,
                    name,
                    ext: ext.map(|x| x.to_string()),
                    lyric_frames,
                    lyric_selected: 0,
                    parsed_lyric,
                    picture,
                })
            }
        }
    }
}

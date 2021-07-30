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
// use mpv::{MpvHandler, MpvHandlerBuilder};
pub mod mpv;
pub mod vlc_player;
use crate::song::Song;
use mpv::MPVAudioPlayer;
use vlc_player::VLCAudioPlayer;
// use anyhow::{anyhow, Result};
use anyhow::Result;

#[allow(non_camel_case_types, unused)]
pub enum PlayerType {
    mp3,
    m4a,
}

// pub fn choose_player(song: Song) -> Result<PlayerType> {
//     match song.ext.as_ref().unwrap().as_str() {
//         "mp3" => return Ok(PlayerType::mp3),
//         "m4a" => return Ok(PlayerType::m4a),
//         &_ => return Err(anyhow!("Unsupported")),
//     }
// }

pub trait AudioPlayer {
    fn queue_and_play(&mut self, new: Song);
    fn volume(&mut self) -> i64;
    fn volume_up(&mut self);
    fn volume_down(&mut self);
    fn pause(&mut self);
    fn resume(&mut self);
    fn is_paused(&mut self) -> bool;
    fn seek(&mut self, secs: i64) -> Result<()>;
    fn get_progress(&mut self) -> (f64, i64, i64, String);
}

pub struct Player {
    pub mpv_player: MPVAudioPlayer,
    pub vlc_player: VLCAudioPlayer,
    pub player_type: PlayerType,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            mpv_player: MPVAudioPlayer::new(),
            vlc_player: VLCAudioPlayer::new(),
            // player_type: PlayerType::m4a,
            player_type: PlayerType::mp3,
        }
    }
}

// impl Player {
//     pub fn new(song: Song) -> Result<dyn AudioPlayer> {
//         let player_type = choose_player(song)?;
//         match player_type {
//             PlayerType::mp3 => {return MPVAudioPlayer::new()}
//             PlayerType::m4a => {}
//         }

//     }
// }
impl AudioPlayer for Player {
    fn queue_and_play(&mut self, new: Song) {
        match self.player_type {
            PlayerType::mp3 => self.mpv_player.queue_and_play(new),
            PlayerType::m4a => self.vlc_player.queue_and_play(new),
        }
    }
    fn volume(&mut self) -> i64 {
        match self.player_type {
            PlayerType::mp3 => self.mpv_player.volume(),
            _ => 0,
        }
    }
    fn volume_up(&mut self) {}
    fn volume_down(&mut self) {}
    fn pause(&mut self) {}
    fn resume(&mut self) {}
    fn is_paused(&mut self) -> bool {
        match self.player_type {
            PlayerType::mp3 => self.mpv_player.is_paused(),
            _ => true,
        }
    }
    fn seek(&mut self, secs: i64) -> Result<()> {
        match self.player_type {
            PlayerType::mp3 => self.mpv_player.seek(secs),
            _ => Ok(()),
        }
    }
    fn get_progress(&mut self) -> (f64, i64, i64, String) {
        match self.player_type {
            PlayerType::mp3 => self.mpv_player.get_progress(),
            _ => (0 as f64, 0, 0, "".to_string()),
        }
    }
}
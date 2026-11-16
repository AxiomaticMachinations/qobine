//! Personal mixes ("DailyQ" and "WeeklyQ").
//!
//! WeeklyQ is generated server side by Qobuz. DailyQ is generated client side, mirroring the
//! official Android app: seed tracks are picked from the local listening history (favouring the
//! most played genre) and combined with `dynamic/suggest` recommendations.

use std::collections::HashSet;

use controls_module::models::Track;

pub const DAILY_MIX_TITLE: &str = "DailyQ";
pub const WEEKLY_MIX_TITLE: &str = "WeeklyQ";

pub const MIX_SIZE: usize = 30;
pub const MAX_SEED_TRACKS: usize = 9;
pub const MIN_MIX_SIZE: usize = 10;
pub const HISTORY_DAYS: i64 = 30;
pub const MIN_GENRE_TRACKS: i64 = 5;
pub const MIN_GENRE_ARTISTS: i64 = 2;

/// Genres the Android app never uses as daily mix seed (classical sub-genres).
pub const EXCLUDED_GENRES: [u32; 14] =
    [59, 60, 61, 62, 63, 73, 75, 76, 140, 167, 170, 177, 178, 179];

/// A track counts as played after 30 seconds, or half its length for short tracks.
pub fn play_threshold_seconds(duration_seconds: u32) -> u64 {
    (duration_seconds as u64 / 2).clamp(1, 30)
}

/// Combines played tracks and suggestions into a mix of at most [`MIX_SIZE`] tracks,
/// spreading suggestions evenly between the played tracks.
pub fn build_daily_mix(played: Vec<Track>, suggestions: Vec<Track>) -> Vec<Track> {
    let mut seen = HashSet::new();
    let played: Vec<Track> = played.into_iter().filter(|t| seen.insert(t.id)).collect();
    let suggestions: Vec<Track> = suggestions
        .into_iter()
        .filter(|t| seen.insert(t.id))
        .collect();

    let played_count = MIX_SIZE
        .saturating_sub(suggestions.len())
        .max(MIN_MIX_SIZE)
        .min(played.len());
    let suggestion_count = (MIX_SIZE - played_count).min(suggestions.len());

    let played: Vec<Track> = played.into_iter().take(played_count).collect();
    let mut suggestions = suggestions.into_iter().take(suggestion_count);

    if played.is_empty() {
        return suggestions.collect();
    }

    let ratio = suggestion_count as f64 / played.len() as f64;
    let mut mix = Vec::with_capacity(played_count + suggestion_count);
    let mut added_suggestions = 0;

    for (index, track) in played.into_iter().enumerate() {
        mix.push(track);
        let target = ((index + 1) as f64 * ratio).round() as usize;
        while added_suggestions < target {
            let Some(suggestion) = suggestions.next() else {
                break;
            };
            mix.push(suggestion);
            added_suggestions += 1;
        }
    }

    mix.extend(suggestions);
    mix
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tracks(ids: std::ops::Range<u32>) -> Vec<Track> {
        ids.map(|id| Track {
            id,
            ..Default::default()
        })
        .collect()
    }

    fn ids(tracks: &[Track]) -> Vec<u32> {
        tracks.iter().map(|t| t.id).collect()
    }

    #[test]
    fn interleaves_suggestions_between_played_tracks() {
        let mix = build_daily_mix(tracks(0..3), tracks(100..106));
        assert_eq!(ids(&mix), vec![0, 100, 101, 1, 102, 103, 2, 104, 105]);
    }

    #[test]
    fn caps_mix_size_and_removes_duplicates() {
        let mut suggestions = tracks(100..150);
        suggestions.insert(
            0,
            Track {
                id: 1,
                ..Default::default()
            },
        );
        let mix = build_daily_mix(tracks(0..9), suggestions);

        assert_eq!(mix.len(), MIX_SIZE);
        assert_eq!(mix.iter().filter(|t| t.id < 100).count(), 9);
        let unique: HashSet<_> = ids(&mix).into_iter().collect();
        assert_eq!(unique.len(), MIX_SIZE);
    }

    #[test]
    fn suggestions_only_when_no_played_tracks() {
        let mix = build_daily_mix(vec![], tracks(100..140));
        assert_eq!(ids(&mix), (100..130).collect::<Vec<_>>());
    }

    #[test]
    fn play_threshold() {
        assert_eq!(play_threshold_seconds(300), 30);
        assert_eq!(play_threshold_seconds(20), 10);
        assert_eq!(play_threshold_seconds(0), 1);
    }
}

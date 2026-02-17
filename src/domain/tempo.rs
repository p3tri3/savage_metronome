// Provides logic for tempo calculations, like tap tempo.
use std::time::Instant;

pub fn calculate_tap_tempo(tap_times: &[Instant]) -> Option<f32> {
    if tap_times.len() < 2 {
        return None;
    }

    let intervals: Vec<f32> = tap_times
        .windows(2)
        .map(|w| w[1].duration_since(w[0]).as_secs_f32())
        .collect();

    let avg = intervals.iter().sum::<f32>() / intervals.len() as f32;
    if avg > 0.0 {
        Some(60.0 / avg)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_calculate_tap_tempo() {
        let now = Instant::now();
        let taps = vec![
            now,
            now + Duration::from_millis(500),
            now + Duration::from_millis(1000),
        ];

        let bpm = calculate_tap_tempo(&taps);
        assert!(bpm.is_some());
        assert!((bpm.unwrap() - 120.0).abs() < 0.001);
    }
}
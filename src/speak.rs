use crate::geometry::{coord_distance, Grain, Space};

pub fn speak(grain: &Grain, space: &Space) -> String {
    if space.offers.is_empty() {
        return grain.text.trim().to_string();
    }

    let mut ranked: Vec<(f32, String)> = space
        .offers
        .iter()
        .map(|offer| {
            let score = (-7.5 * coord_distance(&grain.coord, &offer.coord)).exp();
            (score, offer.text.trim().to_string())
        })
        .collect();

    ranked.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

    let best = ranked[0].0;
    let mut chosen: Vec<String> = Vec::new();
    for (score, text) in ranked {
        if !chosen.is_empty() && score < best * 0.42 {
            break;
        }
        let normalized = normalize_passage(&text);
        if chosen.iter().any(|existing| similar(existing, &normalized)) {
            continue;
        }
        chosen.push(normalized);
        if chosen.len() >= 2 {
            break;
        }
    }

    if chosen.is_empty() {
        grain.text.trim().to_string()
    } else {
        chosen.join(" ")
    }
}

fn normalize_passage(text: &str) -> String {
    let trimmed = text.trim().trim_matches(|c| c == '"' || c == '\'').trim();
    let mut passage = trimmed.to_string();
    if !passage.ends_with('.') && !passage.ends_with('!') && !passage.ends_with('?') {
        passage.push('.');
    }
    passage
}

fn similar(a: &str, b: &str) -> bool {
    let ta = a.to_ascii_lowercase();
    let tb = b.to_ascii_lowercase();
    ta == tb || ta.contains(&tb) || tb.contains(&ta)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn speak_prefers_the_nearer_offer() {
        let mut space = Space::new();
        space
            .add_axis(
                "warmth".into(),
                "ice chart fluorescent".into(),
                "ember darling held".into(),
            )
            .unwrap();
        space
            .add_offer(
                vec![("warmth".into(), 0.1)],
                "The instruments are laid out.".into(),
            )
            .unwrap();
        space
            .add_offer(
                vec![("warmth".into(), 0.9)],
                "I keep thinking of your hands.".into(),
            )
            .unwrap();

        let cold = space.feel("ice fluorescent chart").unwrap();
        let warm = cold
            .along(&space, "warmth", 0.9, 1.0, &HashSet::new())
            .unwrap();
        let spoken = speak(&warm, &space);
        assert!(
            spoken.to_ascii_lowercase().contains("hands"),
            "unexpected speak: {spoken}"
        );
    }
}

use {
  crate::source::source::{get_close_words, get_possible_words},
  levenshtein::levenshtein
};

pub async fn select_close_words<'a>(source: &str, words: &Vec<&'a str>) -> &'a str {
  let close_words = get_close_words().await;

  // TODO: Implement this; now returning the first word

  words[0]
}

pub async fn match_closest_word(word: &str) -> Option<&'static str> {
  let possible_words = match get_possible_words().await.get(word.chars().count() - 1) {
    Some(words) => words,
    None => return None
  };

  let mut best_candidate: Option<&str> = None;
  let mut best_distance = usize::MAX;

  for possible_word in possible_words {
    let distance = levenshtein(word, possible_word);
    if distance < best_distance {
      best_candidate = Some(possible_word);
      best_distance = distance;
    }
  }

  let close_words = get_close_words().await.get(word).unwrap();
  if close_words.len() > 0 {
    let close_words: Vec<&'static str> = close_words.iter().map(|&word| word).collect();
    best_candidate = Some(select_close_words(word, &close_words).await);
  }

  best_candidate
}

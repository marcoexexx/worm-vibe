use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreRecord {
  score: u64,
  length: usize,
  timestamp: i64,
}

impl ScoreRecord {
  pub fn new(score: u64, length: usize, timestamp: i64) -> Self {
    Self {
      score,
      length,
      timestamp,
    }
  }

  pub fn score(&self) -> u64 {
    self.score
  }

  pub fn length(&self) -> usize {
    self.length
  }

  pub fn timestamp(&self) -> i64 {
    self.timestamp
  }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ScoreHistory {
  records: Vec<ScoreRecord>,
}

impl ScoreHistory {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn add(&mut self, record: ScoreRecord) {
    self.records.push(record);
    self.records.sort_by(|a, b| b.score.cmp(&a.score));
  }

  pub fn top_n(&self, n: usize) -> &[ScoreRecord] {
    let end = n.min(self.records.len());
    &self.records[..end]
  }

  pub fn records(&self) -> &[ScoreRecord] {
    &self.records
  }

  pub fn is_empty(&self) -> bool {
    self.records.is_empty()
  }
}

/// Port for score persistence — implemented in `infra` crate.
pub trait ScorePersistence {
  fn load(&self) -> Result<ScoreHistory, String>;
  fn save(&self, history: &ScoreHistory) -> Result<(), String>;
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn score_history_starts_empty() {
    let history = ScoreHistory::new();
    assert!(history.is_empty());
    assert_eq!(history.records().len(), 0);
  }

  #[test]
  fn score_history_sorts_descending() {
    let mut history = ScoreHistory::new();
    history.add(ScoreRecord::new(100, 10, 1000));
    history.add(ScoreRecord::new(500, 20, 2000));
    history.add(ScoreRecord::new(200, 15, 3000));

    let records = history.records();
    assert_eq!(records[0].score(), 500);
    assert_eq!(records[1].score(), 200);
    assert_eq!(records[2].score(), 100);
  }

  #[test]
  fn top_n_returns_at_most_n_records() {
    let mut history = ScoreHistory::new();
    for i in 0..10 {
      history.add(ScoreRecord::new(i * 100, 5, i as i64));
    }

    assert_eq!(history.top_n(3).len(), 3);
    assert_eq!(history.top_n(20).len(), 10); // only 10 exist
  }

  #[test]
  fn top_n_empty_history() {
    let history = ScoreHistory::new();
    assert_eq!(history.top_n(5).len(), 0);
  }

  #[test]
  fn score_record_accessors() {
    let record = ScoreRecord::new(42, 7, 9999);
    assert_eq!(record.score(), 42);
    assert_eq!(record.length(), 7);
    assert_eq!(record.timestamp(), 9999);
  }
}

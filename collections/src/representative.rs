use std::collections::HashMap;

pub(crate) fn mean(v: &Vec<i32>) -> Option<f64> {
  if v.len() == 0 {
    return None;
  }
  Some(v.iter().map(|x| *x as f64).sum::<f64>() / v.len() as f64)
}

pub(crate) fn median(v: &[i32]) -> Option<f64> {
  match v.len() {
    0 => None,
    l if l % 2 == 1 => Some(v[l / 2].into()),
    l => Some((v[l / 2] as f64 + v[l / 2] as f64) / 2.),
  }
}

pub(crate) fn mode(v: &[i32]) -> Option<Vec<i32>> {
  if v.len() == 0 {
    return None;
  }

  let mut cnt: HashMap<i32, i32> = HashMap::new();
  let mut max = 0;

  for vi in v {
    let entry = cnt.entry(*vi).or_insert(0);
    *entry += 1;
    max = max.max(*entry);
  }

  Some(
    cnt
      .iter()
      .filter(|(_, val)| *val == &max)
      .map(|(key, _)| *key)
      .collect(),
  )
}

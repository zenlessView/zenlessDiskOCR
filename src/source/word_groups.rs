use std::collections::HashSet;

fn is_word_close(word_a: &str, word_b: &str) -> bool {
  let mut identifier = false;
  let mut char_a = word_a.chars();
  let mut char_b = word_b.chars();

  while let (Some(a), Some(b)) = (char_a.next(), char_b.next()) {
    if a != b {
      if identifier {
        return false;
      }
      identifier = true;
    }
  }

  identifier
}

fn bron_kerbosch(
  r: &mut HashSet<usize>,
  p: &mut HashSet<usize>,
  x: &mut HashSet<usize>,
  graph: &Vec<HashSet<usize>>,
  result: &mut Vec<HashSet<usize>>,
) {
  if p.is_empty() && x.is_empty() {
    result.push(r.clone());
    return;
  }

  let p_clone = p.clone();
  for v in p_clone.iter() {
    let mut r_new = r.clone();
    let mut p_new = HashSet::<usize>::new();
    let mut x_new = HashSet::<usize>::new();

    r_new.insert(*v);
    for w in p.iter() {
      if graph[*v].contains(w) {
        p_new.insert(*w);
      }
    }
    for w in x.iter() {
      if graph[*v].contains(w) {
        x_new.insert(*w);
      }
    }

    bron_kerbosch(&mut r_new, &mut p_new, &mut x_new, graph, result);

    p.remove(v);
    x.insert(*v);
  }
}

fn get_maximal_cliques(n: usize, edges: &Vec<(usize, usize)>) -> Vec<HashSet<usize>> {
  let mut graph = vec![HashSet::<usize>::new(); n];
  for (a, b) in edges.iter() {
    graph[*a].insert(*b);
    graph[*b].insert(*a);
  }

  let mut r = HashSet::<usize>::new();
  let mut p = (0..n).collect::<HashSet<usize>>();
  let mut x = HashSet::<usize>::new();
  let mut result = vec![];

  bron_kerbosch(&mut r, &mut p, &mut x, &graph, &mut result);

  result
}

pub fn get_close_word_groups(words: &Vec<&'static str>) -> Vec<Vec<&'static str>> {
  let mut edges = vec![];
  for i in 0..words.len() {
    for j in i + 1..words.len() {
      if is_word_close(words[i], words[j]) {
        edges.push((i, j));
      }
    }
  }

  let maximal_cliques = get_maximal_cliques(words.len(), &edges);

  let mut result = vec![];
  for clique in maximal_cliques.iter() {
    let mut group = vec![];
    for i in clique.iter() {
      group.push(words[*i]);
    }
    result.push(group);
  }

  result
}
use hashbrown::HashMap;

fn roll_north(map: &mut Vec<Vec<u8>>) {
  let mut done = false;
  while !done {
    done = true;
    for r in 0..map.len() - 1 {
      for c in 0..map[0].len() {
        if map[r+1][c] == b'O' && map[r][c] == b'.' {
          map[r][c] = b'O';
          map[r+1][c] = b'.';
          done = false;
        }
      }
    }
  }
}

fn rotate(map: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
  let mut newmap = vec![vec![0; map.len()]; map[0].len()];
  for r in 0..map.len() {
    for c in 0..map[0].len() {
      newmap[c][map.len() - 1 - r] = map[r][c];
    }
  }
  newmap
}

fn load(map: &Vec<Vec<u8>>) -> usize {
  (0..map.len())
    .map(|r| (0..map[0].len())
      .filter(|&c| map[r][c] == b'O')
      .map(|_| map.len() - r)
      .sum::<usize>()
    )
    .sum()
}

#[aoc::main(14)]
fn main(input: &str) -> (usize, usize) {
  let mut map = input.split('\n').map(|l| l.as_bytes().to_vec()).collect::<Vec<_>>();
  let p1 = {
    let mut map = map.clone();
    roll_north(&mut map);
    load(&map)
  };

  let mut seen = HashMap::new();
  for i in 1..1000000000 {
    for _ in 0..4 {
      roll_north(&mut map);
      map = rotate(&map);
    }
    if let Some(seen_at) = seen.insert(map.clone(), i) {
      if (1000000000 - i) % (i - seen_at) == 0 {
        break;
      }
    }
  }
  (p1, load(&map))
}
// BFS to find minimal moves between stack states.

use std::collections::HashSet;
use itertools::Itertools;

/// Index of the top (last Some) element, or None if empty.
fn find_last(container: &[Option<char>]) -> Option<usize> {
    for i in (0..container.len()).rev() {
        if container[i].is_some() {
            return Some(i)
        }
    }
    None
}

fn main() {
    // Goal state (top of the stack is the highest index with Some).
    let correct_containers = vec![
        vec![Some('b'), Some('c'), None],
        vec![Some('d'), Some('a'), None],
        vec![Some('e'), None]
    ];

    // Initial frontier (current BFS layer).
    let mut possibilities = vec![vec![
        vec![None, None, None],
        vec![Some('a'), Some('b'), Some('c')],
        vec![Some('d'), Some('e')]
    ]];

    // Visited states.
    let mut seen = HashSet::new();
    seen.extend(possibilities.clone());

    // Number of moves so far.
    let mut depth = 0;

    'outer: loop {
        // Next BFS layer.
        let mut next_possibilities = Vec::new();

        for possibility in &possibilities {
            // Reached goal?
            if *possibility == correct_containers {
                break 'outer;
            }

            // Try moves: for each ordering of (src, dst, other).
            for p in vec![0, 1, 2].into_iter().permutations(3) {
                let mut a = possibility[p[0]].clone(); // src
                let mut b = possibility[p[1]].clone(); // dst
                let c = possibility[p[2]].clone();     // untouched

                if let Some(last_a) = find_last(&a) {
                    // First empty slot in dst.
                    let last_empty_b = find_last(&b).map(|i| i + 1).unwrap_or(0);

                    // Move only if dst has space.
                    if last_empty_b < b.len() {
                        b[last_empty_b] = a[last_a];
                        a[last_a] = None;

                        // Reassemble in original order.
                        let mut next_possibility = vec![vec![]; 3];
                        next_possibility[p[0]] = a;
                        next_possibility[p[1]] = b;
                        next_possibility[p[2]] = c;

                        // Add unseen states to frontier.
                        if seen.insert(next_possibility.clone()) {
                            next_possibilities.push(next_possibility);
                        }
                    }
                }
            }
        }

        possibilities = next_possibilities;
        depth += 1;
    }

    println!("minimal moves: {}", depth);
}

#![allow(dead_code)]

fn build_sa(s: &str) -> Vec<usize> {
    let n = s.len();
    let mut sa = (0..n).map(|x| x).collect::<Vec<usize>>();
    let mut rank = s
        .as_bytes()
        .iter()
        .map(|&c| c as usize)
        .collect::<Vec<usize>>();
    let mut gap = 1;

    let mut temp_rank = vec![0; n];

    while temp_rank[n - 1] != n - 1 {
        sa.sort_by(|&i, &j| {
            if rank[i] != rank[j] {
                rank[i].cmp(&rank[j])
            } else {
                let i = i + gap;
                let j = j + gap;
                if i < n && j < n {
                    rank[i].cmp(&rank[j])
                } else {
                    j.cmp(&i)
                }
            }
        });
        for i in 0..(n - 1) {
            let add = if rank[sa[i]] != rank[sa[i + 1]] {
                rank[sa[i]].cmp(&rank[sa[i + 1]])
            } else {
                let a = sa[i] + gap;
                let b = sa[i + 1] + gap;
                if a < n && b < n {
                    rank[a].cmp(&rank[b])
                } else {
                    b.cmp(&a)
                }
            };
            let add = match add {
                std::cmp::Ordering::Less => 1,
                _ => 0,
            };
            temp_rank[i + 1] = temp_rank[i] + add;
        }
        for i in 0..n {
            rank[sa[i]] = temp_rank[i];
        }
        gap *= 2;
    }

    sa
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(build_sa("banana"), vec![5, 3, 1, 0, 4, 2]);
    }
}

#![allow(dead_code)]

fn build_sa_by_comparison(s: &str) -> Vec<usize> {
    let n = s.len();
    let mut sa = (0..n).map(|x| x).collect::<Vec<usize>>();
    let mut rank = s
        .as_bytes()
        .iter()
        .map(|&c| c as usize)
        .collect::<Vec<usize>>();
    let mut gap = 1;
    while gap < n {
        let sorting = |&i: &usize, &j: &usize| {
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
        };

        sa.sort_by(sorting);
        let mut nrank = vec![0; n];
        for i in 0..(n - 1) {
            let add = match sorting(&sa[i], &sa[i + 1]) {
                std::cmp::Ordering::Less => 1,
                _ => 0,
            };
            nrank[sa[i + 1]] = nrank[sa[i]] + add;
        }
        rank = nrank.clone();
        gap <<= 1;
    }

    sa
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(build_sa_by_comparison("banana"), vec![5, 3, 1, 0, 4, 2]);
    }

    #[test]
    fn test2() {}
}

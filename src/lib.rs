#![allow(dead_code)]

fn build_sa_by_comparison(s: String) -> Vec<usize> {
    let n = s.len() + 1;
    let mut sa = (0..n).map(|x| x).collect::<Vec<usize>>();
    let mut rank = vec![0; n];
    s.as_bytes().iter().enumerate().for_each(|(i, &c)| {
        rank[i] = c as usize;
    });

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

fn build_sa_by_non_comparison(s: String) -> Vec<usize> {
    let n = s.len() + 1;
    let mut sa = (0..n).map(|x| x).collect::<Vec<usize>>();
    let mut rank = vec![0; n];
    s.as_bytes().iter().enumerate().for_each(|(i, &c)| {
        rank[i] = c as usize;
    });
    let mut pos = vec![0; n];
    let mut gap = 1;

    while rank[sa[n - 1]] != n - 1 {
        let mut freq = vec![0; 256];

        for i in 0..n {
            let curr_rank = rank[(i + gap).min(n - 1)];
            freq[curr_rank] += 1;
        }
        for i in 0..255 {
            freq[i + 1] += freq[i];
        }
        for i in (0..n).rev() {
            let curr_rank = rank[(i + gap).min(n - 1)];
            freq[curr_rank] -= 1;
            pos[freq[curr_rank]] = i;
        }

        freq = vec![0; 256];

        for i in 0..n {
            freq[rank[i]] += 1;
        }
        for i in 0..255 {
            freq[i + 1] += freq[i];
        }
        for i in (0..n).rev() {
            let curr = rank[pos[i]];
            freq[curr] -= 1;
            sa[freq[curr]] = pos[i];
        }

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

fn build_lcpa(s: String) -> Vec<usize> {
    let n = s.len() + 1;
    let sa = build_sa_by_non_comparison(s.clone());
    let s = s + "$";
    let s = s.as_bytes();
    let mut lcpa = vec![0; n];
    let mut pos = vec![0; n];
    sa.iter().enumerate().for_each(|(i, &c)| {
        pos[c] = i;
    });

    let mut k = 0;
    for i in 0..n {
        let curr = pos[i];
        if curr == n - 1 {
            continue;
        }
        let j = sa[curr + 1];
        while i + k < n && j + k < n && s[i + k] == s[j + k] {
            k += 1;
        }
        lcpa[curr + 1] = k;
        if k > 0 {
            k -= 1;
        }
    }

    lcpa
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            build_sa_by_comparison("banana".to_owned()),
            vec![6, 5, 3, 1, 0, 4, 2]
        );
        assert_eq!(
            build_sa_by_comparison("mississipi".to_owned()),
            vec![10, 9, 7, 4, 1, 0, 8, 6, 3, 5, 2]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            build_sa_by_non_comparison("banana".to_owned()),
            vec![6, 5, 3, 1, 0, 4, 2]
        );
        assert_eq!(
            build_sa_by_non_comparison("mississipi".to_owned()),
            vec![10, 9, 7, 4, 1, 0, 8, 6, 3, 5, 2]
        );
    }

    #[test]
    fn test3() {
        assert_eq!(build_lcpa("banana".to_owned()), vec![0, 0, 1, 3, 0, 0, 2]);
        assert_eq!(
            build_lcpa("mississipi".to_owned()),
            vec![0, 0, 1, 1, 4, 0, 0, 0, 2, 1, 3]
        );
    }
}

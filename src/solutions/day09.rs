use std::iter::{self};

pub fn part1(input: &str) -> u64 {
    let mut state = Vec::new();

    for (chunk, id) in input.as_bytes().chunks(2).zip(1..) {
        let block = chunk[0] & 0xF;
        let free = chunk.get(1).map_or(0, |&c| c & 0xF);

        state.extend(iter::repeat(Some(id - 1)).take(block as usize));
        state.extend(iter::repeat(None).take(free as usize));
    }

    let mut slice = state.as_slice();
    let mut idx = 0;
    let mut checksum = 0;

    loop {
        let from = slice
            .iter()
            .copied()
            .enumerate()
            .filter_map(|(i, opt)| opt.map(|val| (i, val)))
            .next_back();

        let to = slice
            .iter()
            .enumerate()
            .filter_map(|(i, opt)| match opt {
                Some(id) => {
                    checksum += (idx + i as u64) * (*id as u64);
                    None
                }
                None => Some(i),
            })
            .next();

        let Some(((from_idx, from_val), to_idx)) = from.zip(to) else {
            break;
        };

        if to_idx > from_idx {
            break;
        }

        idx += to_idx as u64;
        checksum += idx * (from_val) as u64;
        idx += 1;

        slice = &slice[to_idx + 1..from_idx];
    }

    checksum
}

pub fn part2(input: &str) -> usize {
    let mut state = Vec::new();
    let mut metas = Vec::new();

    for (chunk, id) in input.as_bytes().chunks(2).zip(1..) {
        let block = chunk[0] & 0xF;
        let free = chunk.get(1).map_or(0, |&c| c & 0xF);

        let meta = Meta {
            idx: state.len(),
            len: block as usize,
        };

        metas.push(meta);
        state.extend(iter::repeat(Some(id - 1)).take(block as usize));
        state.extend(iter::repeat(None).take(free as usize));
    }

    while let Some(meta) = metas.pop() {
        let Meta {
            idx: from_idx,
            len: from_len,
        } = meta;

        let (mut start, end) = state.split_at_mut(from_idx);

        loop {
            let Some(to_idx) = start.iter().position(Option::is_none) else {
                break;
            };

            start = &mut start[to_idx..];

            let to_len = start.iter().copied().take_while(Option::is_none).count();

            if to_len >= from_len {
                end[..from_len].swap_with_slice(&mut start[..from_len]);
                break;
            }

            start = &mut start[to_len..];
        }
    }

    state
        .iter()
        .enumerate()
        .filter_map(|(idx, opt)| opt.map(|n| (idx, n)))
        .map(|(idx, id)| idx * id)
        .sum()
}

struct Meta {
    idx: usize,
    len: usize,
}

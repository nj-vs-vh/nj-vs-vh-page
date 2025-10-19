use itertools::Itertools;

pub fn median_cut(values: &mut [[u8; 3]], depth: usize) -> Option<Vec<[u8; 3]>> {
    if values.len() == 0 {
        return None;
    }
    if values.len() == 1 {
        return Some(vec![values[0].to_owned()]);
    }
    if depth == 0 {
        let mean = values
            .iter()
            .fold([0; 3], |a, b| {
                [
                    a[0] as u32 + b[0] as u32,
                    a[1] as u32 + b[1] as u32,
                    a[2] as u32 + b[2] as u32,
                ]
            })
            .map(|v| v / (values.len() as u32));
        return Some(vec![[mean[0] as u8, mean[1] as u8, mean[2] as u8]]);
    }

    let mut ranges = Vec::<u8>::new();
    for dimension in 0..3 {
        ranges.push(match values.iter().map(|v| v[dimension]).minmax() {
            itertools::MinMaxResult::MinMax(min, max) => max - min,
            _ => return None,
        });
    }

    let split_by = match ranges.iter().position_max() {
        Some(i) => i,
        None => return None,
    };

    values.sort_by_key(|v| v[split_by]);

    let split_at = values.len() / 2;
    Some(
        [
            median_cut(&mut values[..split_at], depth - 1).unwrap_or(vec![]),
            median_cut(&mut values[split_at..], depth - 1).unwrap_or(vec![]),
        ]
        .concat(),
    )
}

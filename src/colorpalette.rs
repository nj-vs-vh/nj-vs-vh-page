use itertools::Itertools;

#[allow(dead_code)]
pub enum PaletteExtractionAlgorithm {
    MedianCut = 0,
    MeanCut = 1,
    ModeBisect = 2,
}

pub fn extract_palette(
    values: &mut [[u8; 3]],
    depth: usize,
    algorithm: &PaletteExtractionAlgorithm,
) -> Option<Vec<[u8; 3]>> {
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

    let split_at = match algorithm {
        PaletteExtractionAlgorithm::MedianCut => values.len() / 2,
        PaletteExtractionAlgorithm::MeanCut => {
            let mean = (values.iter().map(|v| v[split_by] as u32).sum::<u32>()
                / (values.len() as u32)) as u8;
            values
                .iter()
                .enumerate()
                .filter(|(_, v)| v[split_by] > mean)
                .next()
                .unwrap()
                .0
        }
        PaletteExtractionAlgorithm::ModeBisect => {
            let mut left_sum: f32 = 0.0;
            let mut left_sqsum: f32 = 0.0;
            let mut left_count: f32 = 0.0;
            let (mut right_sum, mut right_sqsum) = values
                .iter()
                .map(|c| {
                    let val = c[split_by] as f32;
                    (val, val.powi(2))
                })
                .reduce(|acc, el| (acc.0 + el.0, acc.1 + el.1))
                .unwrap();
            let mut right_count = values.len() as f32;
            let std_full =
                (right_sqsum / right_count - (right_sum / right_count).powi(2)).sqrt() as u32;

            let stdsums: Vec<u32> = values
                .iter()
                .take(values.len() - 1)
                .map(|color| {
                    let v = color[split_by] as f32;
                    let vsq = v.powi(2);
                    left_sum += v;
                    left_sqsum += vsq;
                    left_count += 1.0;
                    right_sum -= v;
                    right_sqsum -= vsq;
                    right_count -= 1.0;

                    ((left_sqsum / left_count - (left_sum / left_count).powi(2)).sqrt()
                        + (right_sqsum / right_count - (right_sum / right_count).powi(2)).sqrt())
                        as u32
                })
                .collect();
            let best_partitition_idx = stdsums.iter().position_min().unwrap();

            if stdsums[best_partitition_idx] < std_full {
                best_partitition_idx + 1
            } else {
                0
            }
        }
    };

    tracing::debug!(
        "Extracting palette from {} values @ depth {}: splitting at {}",
        values.len(),
        depth,
        split_at
    );

    Some(
        [
            extract_palette(&mut values[..split_at], depth - 1, algorithm).unwrap_or(vec![]),
            extract_palette(&mut values[split_at..], depth - 1, algorithm).unwrap_or(vec![]),
        ]
        .concat(),
    )
}

#[allow(dead_code)]
fn mean_std(values: impl Iterator<Item = u8>) -> Option<(f32, f32)> {
    match values
        .map(|el| {
            let elf = el as f32;
            (1, elf, elf.powi(2))
        })
        .reduce(|acc, e| (acc.0 + e.0, acc.1 + e.1, acc.2 + e.2))
    {
        None => None,
        Some((count, sum, sqsum)) => {
            let mean = sum / (count as f32);
            let sqmean = sqsum / count as f32;
            Some((mean, (sqmean - mean.powi(2)).sqrt()))
        }
    }
}

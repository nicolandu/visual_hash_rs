use sha2::{Digest, Sha512};

struct Pos {
    x: usize,
    y: usize,
}

pub fn to_art(input: &[u8], width: usize, height: usize) -> Result<String, &'static str> {
    if width == 0 {
        return Err("width is 0");
    };
    if height == 0 {
        return Err("height is 0");
    };

    // row-major (index is [y][x])
    let mut canvas: Vec<Vec<u32>> = vec![vec![0; width]; height];
    let init_pos = Pos {
        x: (width - 1) / 2,
        y: (height - 1) / 2,
    };
    let mut pos = init_pos;

    let mut hasher = Sha512::new();

    // write input message
    hasher.update(input);

    // read hash digest and consume hasher
    let result = hasher.finalize();
    for byte in result {
        for i in 0..4 {
            match (byte >> (2 * i)) & 0b11 {
                0b_00 => {
                    // up left
                    pos.x = pos.x.saturating_sub(1);
                    pos.y = pos.y.saturating_sub(1);
                }
                0b_01 => {
                    // up right
                    pos.x = pos.x.saturating_add(1).clamp(0, width - 1);
                    pos.y = pos.y.saturating_sub(1);
                }
                0b_10 => {
                    // down left
                    pos.x = pos.x.saturating_sub(1);
                    pos.y = pos.y.saturating_add(1).clamp(0, height - 1);
                }
                0b_11 => {
                    // down right
                    pos.x = pos.x.saturating_add(1).clamp(0, width - 1);
                    pos.y = pos.y.saturating_add(1).clamp(0, height - 1);
                }
                _ => unreachable!(),
            }
            canvas[pos.y][pos.x] += 1;
        }
    }
    Ok(canvas
        .into_iter()
        .flat_map(|row| {
            row.into_iter()
                .map(|c| {
                    ([
                        ' ', '.', 'o', '+', '=', '*', 'B', '0', 'X', '@', '%', '&', '#', '/', '^',
                    ])[(c % 14) as usize]
                })
                .chain(['\n'])
        })
        .collect::<String>())
}

use gif::{Encoder, Frame, Repeat};
use std::borrow::Cow;
use std::fs::File;

const WIDTH: usize = 394;
const HEIGHT: usize = 226;

const MARGIN: usize = 10;

fn main() {
    let iters = 10000;
    let mut state = Life {
        state: [[false; WIDTH]; HEIGHT],
    };
    let init = (300, 120);
    state.state[init.1][init.0 + 1] = true;
    state.state[init.1 + 1][init.0 + 3] = true;
    state.state[init.1 + 2][init.0] = true;
    state.state[init.1 + 2][init.0 + 1] = true;
    state.state[init.1 + 2][init.0 + 4] = true;
    state.state[init.1 + 2][init.0 + 5] = true;
    state.state[init.1 + 2][init.0 + 6] = true;

    let color_map = &[0x29, 0x2e, 0x42, 0xc4, 0x7f, 0xd5];
    let mut image = File::create("tests/samples/beacon.gif").unwrap();
    let mut encoder = Encoder::new(
        &mut image,
        (WIDTH - MARGIN) as u16,
        (HEIGHT - MARGIN) as u16,
        color_map,
    )
    .unwrap();
    encoder.set_repeat(Repeat::Infinite).unwrap();
    for _ in 0..iters {
        let x = state.state;
        let curr = x
            .iter()
            .skip(1)
            .take(HEIGHT - MARGIN)
            .map(|x| {
                x.iter()
                    .skip(1)
                    .take(WIDTH - MARGIN)
                    .map(|x| if *x { 1 } else { 0 })
                    .collect::<Vec<u8>>()
            })
            .enumerate()
            .fold(
                [0; (WIDTH - MARGIN) * (HEIGHT - MARGIN)],
                |mut sum, curr| {
                    for i in 0..WIDTH - MARGIN {
                        sum[i + curr.0 * (WIDTH - MARGIN)] = curr.1[i];
                    }
                    sum
                },
            );
        let mut frame = Frame::default();
        frame.width = (WIDTH - MARGIN) as u16;
        frame.height = (HEIGHT - MARGIN) as u16;
        frame.buffer = Cow::Borrowed(&curr);
        encoder.write_frame(&frame).unwrap();
        state = state.update();
    }
}

#[derive(Debug)]
struct Life {
    state: [[bool; WIDTH]; HEIGHT],
}

impl Life {
    fn update(&self) -> Self {
        let mut ret = self.state;
        for (x, a) in self.state.iter().enumerate() {
            for (y, b) in a.iter().enumerate() {
                let mut count = 0;
                for i in -1..=1 {
                    let (x, y) = (x as i32, y as i32);
                    for j in -1..=1 {
                        let new = (x + i, y + j);
                        if new.0 < 0 || new.0 >= HEIGHT as i32 || new.1 < 0 || new.1 >= WIDTH as i32
                        {
                            continue;
                        }
                        let new = (new.0 as usize, new.1 as usize);
                        if self.state[new.0][new.1] {
                            count += 1;
                        }
                    }
                }
                if *b {
                    count -= 1;
                    ret[x][y] = count == 2 || count == 3;
                } else {
                    ret[x][y] = count == 3;
                }
            }
        }
        Self { state: ret }
    }
}

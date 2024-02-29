use gif::{Encoder, Frame, Repeat};
use std::borrow::Cow;
use std::fs::File;

const WIDTH: usize = 6;
const HEIGHT: usize = 6;

fn main() {
    let iters = 2;
    let mut state = Life {
        state: [
            [false, false, false, false, false, false],
            [false, true, true, false, false, false],
            [false, true, false, false, false, false],
            [false, false, false, false, true, false],
            [false, false, false, true, true, false],
            [false, false, false, false, false, false],
        ],
    };

    let mut ret: Vec<[[bool; WIDTH]; HEIGHT]> = Vec::new();
    for _ in 0..iters {
        println!("{state:?}");
        ret.push(state.state);
        state = state.update();
    }
    let states = ret.iter().map(|x| {
        x.iter()
            .map(|x| {
                x.iter()
                    .map(|x| if *x { 1 } else { 0 })
                    .collect::<Vec<u8>>()
            })
            .enumerate()
            .fold([0; WIDTH * HEIGHT], |mut sum, curr| {
                for i in 0..WIDTH {
                    sum[i + curr.0 * WIDTH] = curr.1[i];
                }
                sum
            })
    });
    let color_map = &[0xFF, 0xFF, 0xFF, 0, 0, 0];
    let mut image = File::create("tests/samples/beacon.gif").unwrap();
    let mut encoder = Encoder::new(&mut image, WIDTH as u16, HEIGHT as u16, color_map).unwrap();
    encoder.set_repeat(Repeat::Infinite).unwrap();
    for state in states {
        let mut frame = Frame::default();
        frame.width = WIDTH as u16;
        frame.height = HEIGHT as u16;
        frame.buffer = Cow::Borrowed(&state);
        encoder.write_frame(&frame).unwrap();
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
                        if new.0 < 0 || new.0 == WIDTH as i32 || new.1 < 0 || new.1 == HEIGHT as i32
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

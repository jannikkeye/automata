use rand::prelude::*;

pub fn mut_two<T>(first_index: usize, second_index: usize, items: &mut [T]) -> (&mut T, &mut T) {
    assert!(first_index != second_index);
    let split_at_index = if first_index < second_index {
        second_index
    } else {
        first_index
    };
    let (first_slice, second_slice) = items.split_at_mut(split_at_index);
    if first_index < second_index {
        (&mut first_slice[first_index], &mut second_slice[0])
    } else {
        (&mut second_slice[0], &mut first_slice[second_index])
    }
}

pub fn move_x(direction: i8, current: u32, width: u32, height: u32) -> u32 {
    let mut temp = current;

    if direction == -1 {
        if current as f32 / width as f32 <= 1.0 {
            temp = current + width - 1;
        } else {
            temp = current - 1;
        }
    }

    if direction == 1 {
        if (current + 1) % width == 0 {
            temp = current - (width - 1);
        } else {
            temp = current + 1;
        }
    }

    temp
}

pub fn move_y(direction: i8, current: u32, width: u32, height: u32) -> u32 {
    let mut temp: u32 = current;

    if direction == -1 {
        if current as f32 / (width as f32) < 1.0 {
            temp = current + (height - 1) * width;
        } else {
            temp = current - width;
        }
    }

    if direction == 1 {
        if current as f32 / width as f32 >= height as f32 - 1.0 {
            if current as f32 / (width as f32) < height as f32 {
                temp = current - (height - 1) * width;
            }
        } else {
            temp = current + width;
        }
    }

    temp
}

pub fn get_surrounding_indices(current: u32, width: u32, height: u32) -> [u32; 8] {
    let top_left = move_y(-1, move_x(-1, current, width, height), width, height);
    let top = move_y(-1, move_x(0, current, width, height), width, height);
    let top_right = move_y(-1, move_x(1, current, width, height), width, height);
    let right = move_y(0, move_x(1, current, width, height), width, height);
    let bottom_right = move_y(1, move_x(1, current, width, height), width, height);
    let bottom = move_y(1, move_x(0, current, width, height), width, height);
    let bottom_left = move_y(1, move_x(-1, current, width, height), width, height);
    let left = move_y(0, move_x(-1, current, width, height), width, height);

    [
        top_left,
        top,
        top_right,
        right,
        bottom_right,
        bottom,
        bottom_left,
        left,
    ]
}

#[test]
fn test_move() {
    {
        let a = move_x(-1, 4, 4, 3);
        let b = move_x(-1, 5, 4, 3);
        let c = move_x(1, 6, 4, 3);
        let d = move_x(1, 7, 4, 3);
        assert_eq!(a, 7);
        assert_eq!(b, 4);
        assert_eq!(c, 7);
        assert_eq!(d, 4);
    }

    {
        let a = move_y(-1, 0, 4, 3);
        let b = move_y(-1, 4, 4, 3);
        let c = move_y(1, 7, 4, 3);
        let d = move_y(1, 11, 4, 3);
        assert_eq!(a, 8);
        assert_eq!(b, 0);
        assert_eq!(c, 11);
        assert_eq!(d, 3);
    }
}

pub fn get_other_index(current: u32, width: u32, height: u32) -> u32 {
    let mut var: u32;
    let mut rng = thread_rng();
    let direction_x = match rng.gen_range(0, 2) {
        0 => -1,
        1 => 1,
        _ => 0,
    };
    let direction_y = match rng.gen_range(0, 2) {
        0 => -1,
        1 => 1,
        _ => 0,
    };

    var = move_x(direction_x, current, width, height);
    var = move_y(direction_y, var, width, height);

    var
}

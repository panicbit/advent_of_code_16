use super::*;
use ndarray::{Ix2,arr1, aview1, aview2};

#[test]
fn init() {
    let lcd = Lcd::new();
    assert_eq!(lcd.field.row(0).len(), 50);
    assert_eq!(lcd.field.column(0).len(), 6);
    assert_eq!(lcd.lit_pixels(), 0);
    lcd.field[Ix2(4, 49)];
}

#[test]
fn parse_ops() {
    assert_eq!(Op::from_str("rect 3x2"), Op::Rect { w: 3, h: 2 });
    assert_eq!(Op::from_str("rotate row y=42 by 12"), Op::RotRow { y: 42, by: 12 });
    assert_eq!(Op::from_str("rotate column x=3 by 4"), Op::RotCol { x: 3, by:  4 });
}

#[test]
fn op_rect() {
    let mut lcd = Lcd::new();
    let area = 3 * 2;
    lcd.modify(Op::Rect { w: 3, h: 2 });
    assert_eq!(lcd.lit_pixels(), area);
    assert_eq!(
        lcd.field.slice(s![..2, ..3]),
        aview2(&[
            [true, true, true],
            [true, true, true],
        ])
    );
}

#[test]
fn op_rot_col() {
    let mut lcd = Lcd::new();
    let area = 3 * 2;
    lcd.modify(Op::Rect { w: 3, h: 2 });
    lcd.modify(Op::RotCol { x: 1, by: 1 });
    assert_eq!(lcd.lit_pixels(), area);
    assert_eq!(
        lcd.field.slice(s![..3, ..4]),
        aview2(&[
            [ true, false,  true, false],
            [ true,  true,  true, false],
            [false,  true, false, false],
        ])
    );
}

#[test]
fn op_rot_row() {
    let mut lcd = Lcd::new();
    let area = 3 * 2;
    lcd.modify(Op::Rect { w: 3, h: 2 });
    lcd.modify(Op::RotCol { x: 1, by: 1 });
    lcd.modify(Op::RotRow { y: 0, by: 4 });
    assert_eq!(lcd.lit_pixels(), area);
    assert_eq!(
        lcd.field.slice(s![..3, ..7]),
        aview2(&[
            [false, false, false, false,  true, false,  true],
            [ true,  true,  true, false, false, false, false],
            [false,  true, false, false, false, false, false],
        ])
    );
}

#[test]
fn rotate() {
    let mut arr = arr1(&[1,2,3,4]);
    super::rotate(arr.view_mut());
    assert_eq!(arr, aview1(&[4,1,2,3]));
}

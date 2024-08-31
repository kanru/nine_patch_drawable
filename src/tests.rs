use crate::{NinePatchDrawable, Patch, PatchKind::*, RectF};
use pretty_assertions::assert_eq;

#[test]
fn nine_slice() {
    #[rustfmt::skip]
    let bitmap_monochrome = [
        0, 9, 0, 0, 0, 9, 0, 0, 0,
        9, 6, 1, 1, 1, 6, 0, 0, 0,
        0, 1, 1, 1, 1, 1, 9, 0, 0,
        0, 1, 1, 1, 1, 1, 9, 0, 0,
        0, 1, 1, 1, 1, 1, 9, 0, 0,
        9, 6, 1, 1, 1, 6, 0, 0, 0,
        0, 0, 9, 9, 9, 0, 0, 0, 0,
    ];
    let mut bitmap = vec![];
    for color in bitmap_monochrome {
        match color {
            0 => bitmap.extend_from_slice(&[0, 0, 0, 0]),
            1 => bitmap.extend_from_slice(&[1, 1, 1, 0]),
            6 => bitmap.extend_from_slice(&[0x80, 0x80, 0x80, 0]),
            9 => bitmap.extend_from_slice(&[0xFF, 0xFF, 0xFF, 0]),
            _ => unreachable!(),
        }
    }
    let drawable = NinePatchDrawable::new(&bitmap, 36, 7, 7).unwrap();
    assert_eq!(1.0, drawable.margin_left);
    assert_eq!(1.0, drawable.margin_top);
    assert_eq!(1.0, drawable.margin_right);
    assert_eq!(1.0, drawable.margin_bottom);
    assert_eq!(
        drawable.scale_to(7, 7),
        [
            Patch {
                source: RectF {
                    left: 1.0,
                    top: 1.0,
                    right: 2.0,
                    bottom: 2.0
                },
                target: RectF {
                    left: 1.0,
                    top: 1.0,
                    right: 2.0,
                    bottom: 2.0
                },
                h_kind: Fixed,
                v_kind: Fixed
            },
            Patch {
                source: RectF {
                    left: 2.0,
                    top: 1.0,
                    right: 5.0,
                    bottom: 2.0
                },
                target: RectF {
                    left: 2.0,
                    top: 1.0,
                    right: 5.0,
                    bottom: 2.0
                },
                h_kind: Stretching,
                v_kind: Fixed
            },
            Patch {
                source: RectF {
                    left: 5.0,
                    top: 1.0,
                    right: 6.0,
                    bottom: 2.0
                },
                target: RectF {
                    left: 5.0,
                    top: 1.0,
                    right: 6.0,
                    bottom: 2.0
                },
                h_kind: Fixed,
                v_kind: Fixed
            },
            Patch {
                source: RectF {
                    left: 1.0,
                    top: 2.0,
                    right: 2.0,
                    bottom: 5.0
                },
                target: RectF {
                    left: 1.0,
                    top: 2.0,
                    right: 2.0,
                    bottom: 5.0
                },
                h_kind: Fixed,
                v_kind: Stretching
            },
            Patch {
                source: RectF {
                    left: 2.0,
                    top: 2.0,
                    right: 5.0,
                    bottom: 5.0
                },
                target: RectF {
                    left: 2.0,
                    top: 2.0,
                    right: 5.0,
                    bottom: 5.0
                },
                h_kind: Stretching,
                v_kind: Stretching
            },
            Patch {
                source: RectF {
                    left: 5.0,
                    top: 2.0,
                    right: 6.0,
                    bottom: 5.0
                },
                target: RectF {
                    left: 5.0,
                    top: 2.0,
                    right: 6.0,
                    bottom: 5.0
                },
                h_kind: Fixed,
                v_kind: Stretching
            },
            Patch {
                source: RectF {
                    left: 1.0,
                    top: 5.0,
                    right: 2.0,
                    bottom: 6.0
                },
                target: RectF {
                    left: 1.0,
                    top: 5.0,
                    right: 2.0,
                    bottom: 6.0
                },
                h_kind: Fixed,
                v_kind: Fixed
            },
            Patch {
                source: RectF {
                    left: 2.0,
                    top: 5.0,
                    right: 5.0,
                    bottom: 6.0
                },
                target: RectF {
                    left: 2.0,
                    top: 5.0,
                    right: 5.0,
                    bottom: 6.0
                },
                h_kind: Stretching,
                v_kind: Fixed
            },
            Patch {
                source: RectF {
                    left: 5.0,
                    top: 5.0,
                    right: 6.0,
                    bottom: 6.0
                },
                target: RectF {
                    left: 5.0,
                    top: 5.0,
                    right: 6.0,
                    bottom: 6.0
                },
                h_kind: Fixed,
                v_kind: Fixed
            }
        ]
    );

    assert_eq!(
        drawable.scale_to(14, 14),
        [
            Patch {
                source: RectF {
                    left: 1.0,
                    top: 1.0,
                    right: 2.0,
                    bottom: 2.0
                },
                target: RectF {
                    left: 1.0,
                    top: 1.0,
                    right: 2.0,
                    bottom: 2.0
                },
                h_kind: Fixed,
                v_kind: Fixed
            },
            Patch {
                source: RectF {
                    left: 2.0,
                    top: 1.0,
                    right: 5.0,
                    bottom: 2.0
                },
                target: RectF {
                    left: 2.0,
                    top: 1.0,
                    right: 12.0,
                    bottom: 2.0
                },
                h_kind: Stretching,
                v_kind: Fixed
            },
            Patch {
                source: RectF {
                    left: 5.0,
                    top: 1.0,
                    right: 6.0,
                    bottom: 2.0
                },
                target: RectF {
                    left: 12.0,
                    top: 1.0,
                    right: 13.0,
                    bottom: 2.0
                },
                h_kind: Fixed,
                v_kind: Fixed
            },
            Patch {
                source: RectF {
                    left: 1.0,
                    top: 2.0,
                    right: 2.0,
                    bottom: 5.0
                },
                target: RectF {
                    left: 1.0,
                    top: 2.0,
                    right: 2.0,
                    bottom: 12.0
                },
                h_kind: Fixed,
                v_kind: Stretching
            },
            Patch {
                source: RectF {
                    left: 2.0,
                    top: 2.0,
                    right: 5.0,
                    bottom: 5.0
                },
                target: RectF {
                    left: 2.0,
                    top: 2.0,
                    right: 12.0,
                    bottom: 12.0
                },
                h_kind: Stretching,
                v_kind: Stretching
            },
            Patch {
                source: RectF {
                    left: 5.0,
                    top: 2.0,
                    right: 6.0,
                    bottom: 5.0
                },
                target: RectF {
                    left: 12.0,
                    top: 2.0,
                    right: 13.0,
                    bottom: 12.0
                },
                h_kind: Fixed,
                v_kind: Stretching
            },
            Patch {
                source: RectF {
                    left: 1.0,
                    top: 5.0,
                    right: 2.0,
                    bottom: 6.0
                },
                target: RectF {
                    left: 1.0,
                    top: 12.0,
                    right: 2.0,
                    bottom: 13.0
                },
                h_kind: Fixed,
                v_kind: Fixed
            },
            Patch {
                source: RectF {
                    left: 2.0,
                    top: 5.0,
                    right: 5.0,
                    bottom: 6.0
                },
                target: RectF {
                    left: 2.0,
                    top: 12.0,
                    right: 12.0,
                    bottom: 13.0
                },
                h_kind: Stretching,
                v_kind: Fixed
            },
            Patch {
                source: RectF {
                    left: 5.0,
                    top: 5.0,
                    right: 6.0,
                    bottom: 6.0
                },
                target: RectF {
                    left: 12.0,
                    top: 12.0,
                    right: 13.0,
                    bottom: 13.0
                },
                h_kind: Fixed,
                v_kind: Fixed
            }
        ]
    );
}

#[test]
fn twenty_five_patch() {
    #[rustfmt::skip]
    let bitmap_monochrome = [
        0, 9, 0, 9, 0, 9, 0, 0, 0,
        9, 6, 1, 1, 1, 6, 0, 0, 0,
        0, 1, 1, 1, 1, 1, 9, 0, 0,
        9, 1, 1, 1, 1, 1, 9, 0, 0,
        0, 1, 1, 1, 1, 1, 9, 0, 0,
        9, 6, 1, 1, 1, 6, 0, 0, 0,
        0, 0, 9, 9, 9, 0, 0, 0, 0,
    ];
    let mut bitmap = vec![];
    for color in bitmap_monochrome {
        match color {
            0 => bitmap.extend_from_slice(&[0, 0, 0, 0]),
            1 => bitmap.extend_from_slice(&[1, 1, 1, 0]),
            6 => bitmap.extend_from_slice(&[0x80, 0x80, 0x80, 0]),
            9 => bitmap.extend_from_slice(&[0xFF, 0xFF, 0xFF, 0]),
            _ => unreachable!(),
        }
    }
    let drawable = NinePatchDrawable::new(&bitmap, 36, 7, 7).unwrap();

    assert_eq!(1.0, drawable.margin_left);
    assert_eq!(1.0, drawable.margin_top);
    assert_eq!(1.0, drawable.margin_right);
    assert_eq!(1.0, drawable.margin_bottom);
    assert_eq!(
        drawable.scale_to(7, 7),
        [
            Patch {
                source: RectF {
                    left: 1.0,
                    top: 1.0,
                    right: 2.0,
                    bottom: 2.0
                },
                target: RectF {
                    left: 1.0,
                    top: 1.0,
                    right: 2.0,
                    bottom: 2.0
                },
                h_kind: Fixed,
                v_kind: Fixed
            },
            Patch {
                source: RectF {
                    left: 2.0,
                    top: 1.0,
                    right: 3.0,
                    bottom: 2.0
                },
                target: RectF {
                    left: 2.0,
                    top: 1.0,
                    right: 3.0,
                    bottom: 2.0
                },
                h_kind: Stretching,
                v_kind: Fixed
            },
            Patch {
                source: RectF {
                    left: 3.0,
                    top: 1.0,
                    right: 4.0,
                    bottom: 2.0
                },
                target: RectF {
                    left: 3.0,
                    top: 1.0,
                    right: 4.0,
                    bottom: 2.0
                },
                h_kind: Fixed,
                v_kind: Fixed
            },
            Patch {
                source: RectF {
                    left: 4.0,
                    top: 1.0,
                    right: 5.0,
                    bottom: 2.0
                },
                target: RectF {
                    left: 4.0,
                    top: 1.0,
                    right: 5.0,
                    bottom: 2.0
                },
                h_kind: Stretching,
                v_kind: Fixed
            },
            Patch {
                source: RectF {
                    left: 5.0,
                    top: 1.0,
                    right: 6.0,
                    bottom: 2.0
                },
                target: RectF {
                    left: 5.0,
                    top: 1.0,
                    right: 6.0,
                    bottom: 2.0
                },
                h_kind: Fixed,
                v_kind: Fixed
            },
            Patch {
                source: RectF {
                    left: 1.0,
                    top: 2.0,
                    right: 2.0,
                    bottom: 3.0
                },
                target: RectF {
                    left: 1.0,
                    top: 2.0,
                    right: 2.0,
                    bottom: 3.0
                },
                h_kind: Fixed,
                v_kind: Stretching
            },
            Patch {
                source: RectF {
                    left: 2.0,
                    top: 2.0,
                    right: 3.0,
                    bottom: 3.0
                },
                target: RectF {
                    left: 2.0,
                    top: 2.0,
                    right: 3.0,
                    bottom: 3.0
                },
                h_kind: Stretching,
                v_kind: Stretching
            },
            Patch {
                source: RectF {
                    left: 3.0,
                    top: 2.0,
                    right: 4.0,
                    bottom: 3.0
                },
                target: RectF {
                    left: 3.0,
                    top: 2.0,
                    right: 4.0,
                    bottom: 3.0
                },
                h_kind: Fixed,
                v_kind: Stretching
            },
            Patch {
                source: RectF {
                    left: 4.0,
                    top: 2.0,
                    right: 5.0,
                    bottom: 3.0
                },
                target: RectF {
                    left: 4.0,
                    top: 2.0,
                    right: 5.0,
                    bottom: 3.0
                },
                h_kind: Stretching,
                v_kind: Stretching
            },
            Patch {
                source: RectF {
                    left: 5.0,
                    top: 2.0,
                    right: 6.0,
                    bottom: 3.0
                },
                target: RectF {
                    left: 5.0,
                    top: 2.0,
                    right: 6.0,
                    bottom: 3.0
                },
                h_kind: Fixed,
                v_kind: Stretching
            },
            Patch {
                source: RectF {
                    left: 1.0,
                    top: 3.0,
                    right: 2.0,
                    bottom: 4.0
                },
                target: RectF {
                    left: 1.0,
                    top: 3.0,
                    right: 2.0,
                    bottom: 4.0
                },
                h_kind: Fixed,
                v_kind: Fixed
            },
            Patch {
                source: RectF {
                    left: 2.0,
                    top: 3.0,
                    right: 3.0,
                    bottom: 4.0
                },
                target: RectF {
                    left: 2.0,
                    top: 3.0,
                    right: 3.0,
                    bottom: 4.0
                },
                h_kind: Stretching,
                v_kind: Fixed
            },
            Patch {
                source: RectF {
                    left: 3.0,
                    top: 3.0,
                    right: 4.0,
                    bottom: 4.0
                },
                target: RectF {
                    left: 3.0,
                    top: 3.0,
                    right: 4.0,
                    bottom: 4.0
                },
                h_kind: Fixed,
                v_kind: Fixed
            },
            Patch {
                source: RectF {
                    left: 4.0,
                    top: 3.0,
                    right: 5.0,
                    bottom: 4.0
                },
                target: RectF {
                    left: 4.0,
                    top: 3.0,
                    right: 5.0,
                    bottom: 4.0
                },
                h_kind: Stretching,
                v_kind: Fixed
            },
            Patch {
                source: RectF {
                    left: 5.0,
                    top: 3.0,
                    right: 6.0,
                    bottom: 4.0
                },
                target: RectF {
                    left: 5.0,
                    top: 3.0,
                    right: 6.0,
                    bottom: 4.0
                },
                h_kind: Fixed,
                v_kind: Fixed
            },
            Patch {
                source: RectF {
                    left: 1.0,
                    top: 4.0,
                    right: 2.0,
                    bottom: 5.0
                },
                target: RectF {
                    left: 1.0,
                    top: 4.0,
                    right: 2.0,
                    bottom: 5.0
                },
                h_kind: Fixed,
                v_kind: Stretching
            },
            Patch {
                source: RectF {
                    left: 2.0,
                    top: 4.0,
                    right: 3.0,
                    bottom: 5.0
                },
                target: RectF {
                    left: 2.0,
                    top: 4.0,
                    right: 3.0,
                    bottom: 5.0
                },
                h_kind: Stretching,
                v_kind: Stretching
            },
            Patch {
                source: RectF {
                    left: 3.0,
                    top: 4.0,
                    right: 4.0,
                    bottom: 5.0
                },
                target: RectF {
                    left: 3.0,
                    top: 4.0,
                    right: 4.0,
                    bottom: 5.0
                },
                h_kind: Fixed,
                v_kind: Stretching
            },
            Patch {
                source: RectF {
                    left: 4.0,
                    top: 4.0,
                    right: 5.0,
                    bottom: 5.0
                },
                target: RectF {
                    left: 4.0,
                    top: 4.0,
                    right: 5.0,
                    bottom: 5.0
                },
                h_kind: Stretching,
                v_kind: Stretching
            },
            Patch {
                source: RectF {
                    left: 5.0,
                    top: 4.0,
                    right: 6.0,
                    bottom: 5.0
                },
                target: RectF {
                    left: 5.0,
                    top: 4.0,
                    right: 6.0,
                    bottom: 5.0
                },
                h_kind: Fixed,
                v_kind: Stretching
            },
            Patch {
                source: RectF {
                    left: 1.0,
                    top: 5.0,
                    right: 2.0,
                    bottom: 6.0
                },
                target: RectF {
                    left: 1.0,
                    top: 5.0,
                    right: 2.0,
                    bottom: 6.0
                },
                h_kind: Fixed,
                v_kind: Fixed
            },
            Patch {
                source: RectF {
                    left: 2.0,
                    top: 5.0,
                    right: 3.0,
                    bottom: 6.0
                },
                target: RectF {
                    left: 2.0,
                    top: 5.0,
                    right: 3.0,
                    bottom: 6.0
                },
                h_kind: Stretching,
                v_kind: Fixed
            },
            Patch {
                source: RectF {
                    left: 3.0,
                    top: 5.0,
                    right: 4.0,
                    bottom: 6.0
                },
                target: RectF {
                    left: 3.0,
                    top: 5.0,
                    right: 4.0,
                    bottom: 6.0
                },
                h_kind: Fixed,
                v_kind: Fixed
            },
            Patch {
                source: RectF {
                    left: 4.0,
                    top: 5.0,
                    right: 5.0,
                    bottom: 6.0
                },
                target: RectF {
                    left: 4.0,
                    top: 5.0,
                    right: 5.0,
                    bottom: 6.0
                },
                h_kind: Stretching,
                v_kind: Fixed
            },
            Patch {
                source: RectF {
                    left: 5.0,
                    top: 5.0,
                    right: 6.0,
                    bottom: 6.0
                },
                target: RectF {
                    left: 5.0,
                    top: 5.0,
                    right: 6.0,
                    bottom: 6.0
                },
                h_kind: Fixed,
                v_kind: Fixed
            }
        ]
    );
    assert_eq!(
        drawable.scale_to(48, 48),
        [
            Patch {
                source: RectF {
                    left: 1.0,
                    top: 1.0,
                    right: 2.0,
                    bottom: 2.0
                },
                target: RectF {
                    left: 1.0,
                    top: 1.0,
                    right: 2.0,
                    bottom: 2.0
                },
                h_kind: Fixed,
                v_kind: Fixed
            },
            Patch {
                source: RectF {
                    left: 2.0,
                    top: 1.0,
                    right: 3.0,
                    bottom: 2.0
                },
                target: RectF {
                    left: 2.0,
                    top: 1.0,
                    right: 23.5,
                    bottom: 2.0
                },
                h_kind: Stretching,
                v_kind: Fixed
            },
            Patch {
                source: RectF {
                    left: 3.0,
                    top: 1.0,
                    right: 4.0,
                    bottom: 2.0
                },
                target: RectF {
                    left: 23.5,
                    top: 1.0,
                    right: 24.5,
                    bottom: 2.0
                },
                h_kind: Fixed,
                v_kind: Fixed
            },
            Patch {
                source: RectF {
                    left: 4.0,
                    top: 1.0,
                    right: 5.0,
                    bottom: 2.0
                },
                target: RectF {
                    left: 24.5,
                    top: 1.0,
                    right: 46.0,
                    bottom: 2.0
                },
                h_kind: Stretching,
                v_kind: Fixed
            },
            Patch {
                source: RectF {
                    left: 5.0,
                    top: 1.0,
                    right: 6.0,
                    bottom: 2.0
                },
                target: RectF {
                    left: 46.0,
                    top: 1.0,
                    right: 47.0,
                    bottom: 2.0
                },
                h_kind: Fixed,
                v_kind: Fixed
            },
            Patch {
                source: RectF {
                    left: 1.0,
                    top: 2.0,
                    right: 2.0,
                    bottom: 3.0
                },
                target: RectF {
                    left: 1.0,
                    top: 2.0,
                    right: 2.0,
                    bottom: 23.5
                },
                h_kind: Fixed,
                v_kind: Stretching
            },
            Patch {
                source: RectF {
                    left: 2.0,
                    top: 2.0,
                    right: 3.0,
                    bottom: 3.0
                },
                target: RectF {
                    left: 2.0,
                    top: 2.0,
                    right: 23.5,
                    bottom: 23.5
                },
                h_kind: Stretching,
                v_kind: Stretching
            },
            Patch {
                source: RectF {
                    left: 3.0,
                    top: 2.0,
                    right: 4.0,
                    bottom: 3.0
                },
                target: RectF {
                    left: 23.5,
                    top: 2.0,
                    right: 24.5,
                    bottom: 23.5
                },
                h_kind: Fixed,
                v_kind: Stretching
            },
            Patch {
                source: RectF {
                    left: 4.0,
                    top: 2.0,
                    right: 5.0,
                    bottom: 3.0
                },
                target: RectF {
                    left: 24.5,
                    top: 2.0,
                    right: 46.0,
                    bottom: 23.5
                },
                h_kind: Stretching,
                v_kind: Stretching
            },
            Patch {
                source: RectF {
                    left: 5.0,
                    top: 2.0,
                    right: 6.0,
                    bottom: 3.0
                },
                target: RectF {
                    left: 46.0,
                    top: 2.0,
                    right: 47.0,
                    bottom: 23.5
                },
                h_kind: Fixed,
                v_kind: Stretching
            },
            Patch {
                source: RectF {
                    left: 1.0,
                    top: 3.0,
                    right: 2.0,
                    bottom: 4.0
                },
                target: RectF {
                    left: 1.0,
                    top: 23.5,
                    right: 2.0,
                    bottom: 24.5
                },
                h_kind: Fixed,
                v_kind: Fixed
            },
            Patch {
                source: RectF {
                    left: 2.0,
                    top: 3.0,
                    right: 3.0,
                    bottom: 4.0
                },
                target: RectF {
                    left: 2.0,
                    top: 23.5,
                    right: 23.5,
                    bottom: 24.5
                },
                h_kind: Stretching,
                v_kind: Fixed
            },
            Patch {
                source: RectF {
                    left: 3.0,
                    top: 3.0,
                    right: 4.0,
                    bottom: 4.0
                },
                target: RectF {
                    left: 23.5,
                    top: 23.5,
                    right: 24.5,
                    bottom: 24.5
                },
                h_kind: Fixed,
                v_kind: Fixed
            },
            Patch {
                source: RectF {
                    left: 4.0,
                    top: 3.0,
                    right: 5.0,
                    bottom: 4.0
                },
                target: RectF {
                    left: 24.5,
                    top: 23.5,
                    right: 46.0,
                    bottom: 24.5
                },
                h_kind: Stretching,
                v_kind: Fixed
            },
            Patch {
                source: RectF {
                    left: 5.0,
                    top: 3.0,
                    right: 6.0,
                    bottom: 4.0
                },
                target: RectF {
                    left: 46.0,
                    top: 23.5,
                    right: 47.0,
                    bottom: 24.5
                },
                h_kind: Fixed,
                v_kind: Fixed
            },
            Patch {
                source: RectF {
                    left: 1.0,
                    top: 4.0,
                    right: 2.0,
                    bottom: 5.0
                },
                target: RectF {
                    left: 1.0,
                    top: 24.5,
                    right: 2.0,
                    bottom: 46.0
                },
                h_kind: Fixed,
                v_kind: Stretching
            },
            Patch {
                source: RectF {
                    left: 2.0,
                    top: 4.0,
                    right: 3.0,
                    bottom: 5.0
                },
                target: RectF {
                    left: 2.0,
                    top: 24.5,
                    right: 23.5,
                    bottom: 46.0
                },
                h_kind: Stretching,
                v_kind: Stretching
            },
            Patch {
                source: RectF {
                    left: 3.0,
                    top: 4.0,
                    right: 4.0,
                    bottom: 5.0
                },
                target: RectF {
                    left: 23.5,
                    top: 24.5,
                    right: 24.5,
                    bottom: 46.0
                },
                h_kind: Fixed,
                v_kind: Stretching
            },
            Patch {
                source: RectF {
                    left: 4.0,
                    top: 4.0,
                    right: 5.0,
                    bottom: 5.0
                },
                target: RectF {
                    left: 24.5,
                    top: 24.5,
                    right: 46.0,
                    bottom: 46.0
                },
                h_kind: Stretching,
                v_kind: Stretching
            },
            Patch {
                source: RectF {
                    left: 5.0,
                    top: 4.0,
                    right: 6.0,
                    bottom: 5.0
                },
                target: RectF {
                    left: 46.0,
                    top: 24.5,
                    right: 47.0,
                    bottom: 46.0
                },
                h_kind: Fixed,
                v_kind: Stretching
            },
            Patch {
                source: RectF {
                    left: 1.0,
                    top: 5.0,
                    right: 2.0,
                    bottom: 6.0
                },
                target: RectF {
                    left: 1.0,
                    top: 46.0,
                    right: 2.0,
                    bottom: 47.0
                },
                h_kind: Fixed,
                v_kind: Fixed
            },
            Patch {
                source: RectF {
                    left: 2.0,
                    top: 5.0,
                    right: 3.0,
                    bottom: 6.0
                },
                target: RectF {
                    left: 2.0,
                    top: 46.0,
                    right: 23.5,
                    bottom: 47.0
                },
                h_kind: Stretching,
                v_kind: Fixed
            },
            Patch {
                source: RectF {
                    left: 3.0,
                    top: 5.0,
                    right: 4.0,
                    bottom: 6.0
                },
                target: RectF {
                    left: 23.5,
                    top: 46.0,
                    right: 24.5,
                    bottom: 47.0
                },
                h_kind: Fixed,
                v_kind: Fixed
            },
            Patch {
                source: RectF {
                    left: 4.0,
                    top: 5.0,
                    right: 5.0,
                    bottom: 6.0
                },
                target: RectF {
                    left: 24.5,
                    top: 46.0,
                    right: 46.0,
                    bottom: 47.0
                },
                h_kind: Stretching,
                v_kind: Fixed
            },
            Patch {
                source: RectF {
                    left: 5.0,
                    top: 5.0,
                    right: 6.0,
                    bottom: 6.0
                },
                target: RectF {
                    left: 46.0,
                    top: 46.0,
                    right: 47.0,
                    bottom: 47.0
                },
                h_kind: Fixed,
                v_kind: Fixed
            }
        ]
    );
}

use super::*;
use std::f64::consts::PI;

#[test]
fn key_frame_timef_should_be_straight_conversion() {
    let key_frame = KeyFrame::new(5, 30.5);

    assert_eq!(key_frame.timef(), 5 as f64);
}

#[test]
fn animated_value_with_no_frames() {
    let anim_val = AnimatedValue::linear(Vec::new());

    let test_results = vec![20.14, 0.0, PI, -PI]
        .iter()
        .map(|val| anim_val.get_value(*val).round() as i64)
        .collect::<Vec<i64>>();

    for tr in test_results {
        assert_eq!(0, tr);
    }
}

#[test]
fn constant_animated_value() {
    let anim_val = AnimatedValue::constant(10.0);

    let test_results = vec![20.14, 0.0, PI, -PI]
        .iter()
        .map(|val| anim_val.get_value(*val).round() as i64)
        .collect::<Vec<i64>>();

    for tr in test_results {
        assert_eq!(10, tr);
    }
}

#[test]
fn linear_animated_value() {
    let anim_val = AnimatedValue::linear(vec![KeyFrame::new(0, 0.0), KeyFrame::new(50, 1.0)]);

    let expected_input_output_pairs = vec![
        (0.0, 0.0),
        (10.0, 0.2),
        (25.0, 0.5),
        (50.0, 1.0),
        (500.0, 1.0),
        (-10.0, 0.0),
    ];

    for (input, expected_output) in expected_input_output_pairs {
        let output = anim_val.get_value(input);
        assert_eq!(expected_output, output);
    }
}

#[test]
fn find_frames_ba_when_zero_frames_should_be_none() {
    let anim_val = AnimatedValue::linear(Vec::new());

    let result = anim_val.find_frames_before_and_after(0.0);

    assert_eq!(None, result);
}

#[test]
fn find_frames_ba_when_one_frame_should_be_both_same() {
    let anim_val = AnimatedValue::linear(vec![KeyFrame::new(0, 0.0)]);

    let result = anim_val.find_frames_before_and_after(0.0);

    assert_eq!(Some((0, 0)), result);
}

#[test]
fn find_frames_ba_when_two_frames_on_either_side_of_time() {
    let anim_val = AnimatedValue::linear(vec![KeyFrame::new(0, 0.0), KeyFrame::new(50, 1.0)]);

    let result = anim_val.find_frames_before_and_after(25.0);

    assert_eq!(Some((0, 1)), result);
}

#[test]
fn find_frames_ba_when_two_frames_after_time() {
    let anim_val = AnimatedValue::linear(vec![KeyFrame::new(0, 0.0), KeyFrame::new(50, 1.0)]);

    let result = anim_val.find_frames_before_and_after(-10.0);

    assert_eq!(Some((0, 0)), result);
}

#[test]
fn find_frames_ba_when_two_frames_before_time() {
    let anim_val = AnimatedValue::linear(vec![KeyFrame::new(0, 0.0), KeyFrame::new(50, 1.0)]);

    let result = anim_val.find_frames_before_and_after(100.0);

    assert_eq!(Some((1, 1)), result);
}

#[test]
fn find_frames_ba_when_several_frames() {
    let anim_val = AnimatedValue::linear(vec![
        KeyFrame::new(0, 0.0),
        KeyFrame::new(50, 1.0),
        KeyFrame::new(100, 0.0),
        // intentionally out of order to test that it's sorted
        KeyFrame::new(75, 1.0),
    ]);

    let expected_input_output_pairs = vec![
        (-1.0, (0, 0)),
        (0.0, (0, 0)),
        (10.0, (0, 1)),
        (50.0, (1, 1)),
        (75.0, (2, 2)),
        (100.0, (3, 3)),
        (150.0, (3, 3)),
    ];

    for (input, expected_output) in expected_input_output_pairs {
        let output = anim_val.find_frames_before_and_after(input);
        assert_eq!(expected_output, output.expect("should have found a frame"));
    }
}

#[test]
fn test_is_constant_function() {
    let no_keyframes = AnimatedValue {
        frames: vec![],
        equation: linear_animation_equation
    };
    let one_keyframe = AnimatedValue {
        frames: vec![KeyFrame::new(5, 5.0)],
        equation: linear_animation_equation
    };
    let two_keyframes_different_value = AnimatedValue {
        frames: vec![KeyFrame::new(0, 1.0), KeyFrame::new(24, 40.0)],
        equation: linear_animation_equation
    };
    // Should still not be considered a constant
    let two_keyframes_same_value = AnimatedValue {
        frames: vec![KeyFrame::new(0, 1.0), KeyFrame::new(24, 1.0)],
        equation: linear_animation_equation
    };

    assert!(no_keyframes.is_constant());
    assert!(one_keyframe.is_constant());
    assert!(!two_keyframes_different_value.is_constant());
    assert!(!two_keyframes_same_value.is_constant());
}

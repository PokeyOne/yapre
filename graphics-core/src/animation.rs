/// A value that is animated by keyframes.
pub struct AnimatedValue {
    /// Essentially a map of time to value.
    frames: Vec<(f64, f64)>,
    /// Equation that determines the intermediate value between to frames.
    /// The input is the percentage between the two frames, and is always
    /// between 0 and 1. The output should be also be between 0 and 1, and is
    /// the actual percentage between the two frames.
    equation: fn(f64) -> f64
}

/// Just returns the input, because the physical percentage through should
/// match the time percentage through.
pub fn linear_animation_equation(percentage: f64) -> f64 {
    percentage
}

/// Essentially a parabolic curve. Before the halfway point, the value is
/// y = 2x^2, after the halfway point, the value is y = -2(x-1)^2 + 1.
/// For Desmos graph: https://www.desmos.com/calculator/wwojdahtet
///
/// This function is defined to be 0 when x < 0, and 1 when x > 1.
pub fn ease_in_out_animation_equation(percentage: f64) -> f64 {
    if percentage < 0.0 { // Before the graph starts
        0.0
    } else if percentage < 0.5 { // first half of the graph
        2.0 * percentage * percentage
    } else if percentage < 1.0 { // second half of the graph
        -2.0 * (percentage - 1.0).powi(2) + 1.0
    } else { // After the graph ends
        1.0
    }
}

impl AnimatedValue {
    /// Just a constant value that will not change with time.
    pub fn constant(value: f64) -> AnimatedValue {
        AnimatedValue {
            frames: vec![(0.0, value)],
            equation: linear_animation_equation
        }
    }

    /// Linearly animated value. Frames should be sorted by time. The first
    /// number in each frame is the time, and the second number is the value.
    /// The number of frames should be at least 1, but if there are zero, this
    /// method will not complain. The result will be a constant of zero.
    pub fn linear(mut frames: Vec<(f64, f64)>) -> AnimatedValue {
        frames.sort_unstable();
        AnimatedValue {
            frames,
            equation: linear_animation_equation
        }
    }

    /// An animated value that is eased in and out. Frames should be sorted by
    /// time. The first number in each frame is the time, and the second
    /// number is the value. The number of frames should be at least 1, but if
    /// there are zero, this method will not complain. The result will be a
    /// constant of zero.
    pub fn ease_in_out(mut frames: Vec<(f64, f64)>) -> AnimatedValue {
        frames.sort_unstable();
        AnimatedValue {
            frames,
            equation: ease_in_out_animation_equation
        }
    }

    pub fn insert_frame(&mut self, time: f64, value: f64) {
        self.frames.push((time, value));
        self.frames.sort_unstable();
    }

    /// Returns the value at the given time.
    pub fn get_value(&self, time: f64) -> f64 {
        // Check for not having any frames.
        if self.frames.len() == 0 {
            return 0.0;
        }

        // Check for only having one frame.
        if self.frames.len() == 1 {
            return self.frames[0].1;
        }

        // Check for being before the first frame.
        if time < self.frames[0].0 {
            return self.frames[0].1;
        }

        // Check for being after the last frame.
        if time > self.frames[self.frames.len() - 1].0 {
            return self.frames[self.frames.len() - 1].1;
        }

        // Find the two frames that are before and after the time.
        let (before, after) = self.find_frames_before_and_after(time);
        let before = &self.frames[before];
        let after = &self.frames[after];

        let frame_time_difference = after.0 - before.0;
        let time_since_before = time - before.0;

        self.equation(time_since_before / frame_time_difference)
    }

    /// Finds the indices of the two frames that are before and after the time.
    fn find_frames_before_and_after(&self, time: f64) -> (usize, usize) {
        // TODO: This is iterative, but it's probably not the most efficient
        //       should be binary search.
        let mut index = 0;
        for i in 0..self.frames.len() {
            if self.frames[i].0 > time {
                index = i;
                break;
            }
        }
        (index - 1, index)
    }
}

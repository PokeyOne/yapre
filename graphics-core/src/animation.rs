#[cfg(test)]
mod tests;

use std::cmp::Ordering;

/// A simple value mapped to a frame/time in the scene. The time stored is an
/// integer because it should be mapped to a frame number. Frame numbers can
/// be negative.
#[derive(Clone, Debug, Copy)]
pub struct KeyFrame {
    /// The frame number that this node on the timeline.
    pub time: i64,
    /// The value at said KeyFrame
    pub value: f64
}

impl KeyFrame {
    /// Construct a new key frame at the specified time and place.
    pub fn new(time: i64, value: f64) -> Self {
        Self { time, value }
    }

    /// Gets the time as a float. This is common in the calculation of values
    /// at certain times.
    ///
    /// # Examples
    /// ```
    /// # use yapre_graphics_core::animation::KeyFrame;
    /// let key_frame = KeyFrame::new(45, 1.0);
    /// assert_eq!(key_frame.timef(), 45.0f64);
    /// ```
    //noinspection SpellCheckingInspection
    pub fn timef(&self) -> f64 {
        self.time as f64
    }
}

impl Eq for KeyFrame {}

impl PartialEq<Self> for KeyFrame {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time
    }
}

impl PartialOrd<Self> for KeyFrame {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for KeyFrame {
    fn cmp(&self, other: &Self) -> Ordering {
        self.time.cmp(&other.time)
    }
}

/// A value that is animated by keyframes.
#[derive(Clone, Debug)]
pub struct AnimatedValue {
    /// Essentially a map of time to value.
    frames: Vec<KeyFrame>,
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
            frames: vec![KeyFrame::new(0, value)],
            equation: linear_animation_equation
        }
    }

    /// Linearly animated value. Frames should be sorted by time.
    pub fn linear(mut frames: Vec<KeyFrame>) -> AnimatedValue {
        frames.sort_unstable();
        AnimatedValue {
            frames,
            equation: linear_animation_equation
        }
    }

    /// An animated value that is eased in and out. Frames should be sorted by
    /// time.
    pub fn ease_in_out(mut frames: Vec<KeyFrame>) -> AnimatedValue {
        frames.sort_unstable();
        AnimatedValue {
            frames,
            equation: ease_in_out_animation_equation
        }
    }

    pub fn insert_frame(&mut self, time: i64, value: f64) {
        self.frames.push(KeyFrame::new(time, value));
        self.frames.sort_unstable();
    }

    /// Returns the value at the given time.
    pub fn get_value(&self, time: f64) -> f64 {
        // Check for not having any frames.
        if self.frames.len() == 0 {
            return 0.0;
        }

        // Check for only having one frame or time is before the first frame.
        if self.frames.len() == 1 || time < self.frames[0].timef() {
            return self.frames[0].value;
        }

        // Check for being after the last frame.
        if time > self.frames[self.frames.len() - 1].timef() {
            return self.frames[self.frames.len() - 1].value;
        }

        // Find the two frames that are before and after the time.
        let (before, after) = match self.find_frames_before_and_after(time) {
            Some(frames) => frames,
            // None is unreachable because we already checked for zero frames.
            None => unreachable!()
        };
        if before == after {
            // If the two frames are the same, return the value of the frame.
            return self.frames[before].value;
        }

        let before = &self.frames[before];
        let after = &self.frames[after];

        let frame_time_difference = (after.time - before.time) as f64;
        let time_since_before = time - before.time as f64;

        (self.equation)(time_since_before / frame_time_difference)
    }

    /// Finds the indices of the two frames that are before and after the time.
    ///
    /// If there are not frames, this will return None, otherwise it will be
    /// Some value. If time is before all the frames, or there is only one
    /// frame, the result will be (0, 0). If the time is after the last frame,
    /// the result will be (len - 1, len - 1). If there are two or more frames
    /// and the time is between two frames, the result will be Some tuple with
    /// the index of the frame before and the frame after the time value.
    ///
    /// If the time value is exactly the same as a frame, the result will be
    /// Some tuple with the index of the frame that it is equal to.
    fn find_frames_before_and_after(&self, time: f64) -> Option<(usize, usize)> {
        if self.frames.len() == 0 {
            return None;
        } else if self.frames.len() == 1 {
            return Some((0, 0));
        } else if time <= self.frames[0].timef() {
            return Some((0, 0));
        } else if time >= self.frames[self.frames.len() - 1].timef() {
            let li = self.frames.len() - 1;
            return Some((li, li));
        }

        // TODO: This is iterative, but it's probably not the most efficient
        //       should be binary search.
        let mut index = 0;
        for i in 0..self.frames.len() {
            if self.frames[i].timef() > time {
                index = i;
                break;
            } else if self.frames[i].time == time as i64 {
                return Some((i, i));
            }
        }
        println!("{}", index);
        println!("{:?}", self);
        println!("input time: {}", time);
        Some((index - 1, index))
    }
}

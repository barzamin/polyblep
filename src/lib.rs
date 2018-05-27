use std::f64::consts::PI;

#[derive(Debug, Clone, Copy)]
pub enum Wave {
    Sine,
    Sawtooth,
    Square,
}

fn blep(t: f64, dt: f64) -> f64 {
    if t < dt {
        // 0 <= t' < 1

        let t = t / dt;
        2. * t - t.powi(2) - 1.
    } else if t > (1. - dt) {
        // -1 < t' < 0

        let t = (t - 1.) / dt;
        t.powi(2) + 2. * t + 1.
    } else {
        0.
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Oscillator {
    wave: Wave,
    phase: f64,
    sample_rate: f64,
    frequency: f64,
}

impl Oscillator {
    pub fn new(sample_rate: f64, frequency: f64, wave: Wave) -> Oscillator {
        Oscillator {
            wave,
            phase: 0.,
            sample_rate,
            frequency,
        }
    }

    fn dt(&self) -> f64 {
        self.frequency / self.sample_rate
    }

    fn naive_sample(&self, shape: Wave) -> f64 {
        match shape {
            Wave::Sawtooth => self.phase / PI - 1.,
            Wave::Sine => self.phase.sin(),
            Wave::Square => if self.phase < PI {
                1.
            } else {
                -1.
            },
        }
    }

    pub fn next_sample(&mut self) -> f64 {
        let t = self.phase / (2. * PI);
        let sample = match self.wave {
            Wave::Sawtooth => self.naive_sample(self.wave) - blep(t, self.dt()),
            Wave::Square => {
                self.naive_sample(self.wave) + blep(t, self.dt()) - blep((t + 0.5) % 1., self.dt())
            }
            Wave::Sine => self.naive_sample(self.wave),
        };

        self.phase += self.dt() * (2. * PI);
        while self.phase >= 2. * PI {
            self.phase -= 2. * PI;
        }

        sample
    }
}

impl Iterator for Oscillator {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.next_sample())
    }
}

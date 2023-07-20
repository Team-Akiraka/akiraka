pub mod sine {
    use std::f64::consts::PI;

    pub fn ease_in(x: f64) -> f64 {
        1.0 - ((x * PI) / 2.0).cos()
    }

    pub fn ease_out(x: f64) -> f64 {
        ((x * PI) / 2.0).sin()
    }

    pub fn ease_both(x: f64) -> f64 {
        x
    }
}

pub mod circ {
    pub fn ease_in(x: f64) -> f64 {
        1.0 - (1.0 - x.powf(2.0))
    }

    pub fn ease_out(x: f64) -> f64 {
        1.0 - (-2.0 * x + 2.0).powf(5.0) / 2.0
    }

    pub fn ease_both(x: f64) -> f64 {
        if x < 0.5 {
            (1.0 - (1.0 - (2.0 * x).powf(2.0))) / 2.0
        } else {
            ((1.0 - (-2.0 * x + 2.0).powf(2.0)) + 1.0) / 2.0
        }
    }
}

pub mod elastic {
    use std::f64::consts::PI;

    const C4: f64 = (0.4 * PI) / 6.0;

    pub fn ease_in(x: f64) -> f64 {
        x
    }

    pub fn ease_out(x: f64) -> f64 {
        if x == 0.0 {
            0.0
        } else {
            if x == 1.0 {
                1.0
            } else {
                (2.0_f64).powf(-10.0 * x) * ((x * 10.0 - 0.75) * C4).sin() + 1.0
            }
        }
    }

    pub fn ease_both(x: f64) -> f64 {
        0.0
    }
}

pub mod back {
    const C1: f64 = 1.74158;
    const C3: f64 = C1 + 1.0;

    pub fn ease_in(x: f64) -> f64 {
        x
    }

    pub fn ease_out(x: f64) -> f64 {
        1.0 + C3 * (x - 1.0).powf(3.0) + C1 * (x - 1.0).powf(2.0)
    }

    pub fn ease_both(x: f64) -> f64 {
        x
    }
}

pub mod expo {
    const C1: f64 = 1.74158;
    const C3: f64 = C1 + 1.0;

    pub fn ease_in(x: f64) -> f64 {
        x
    }

    pub fn ease_out(x: f64) -> f64 {
        if x == 1.0 {
            1.0
        } else {
            1.0 - (2.0_f64).powf(-10.0 * x)
        }
    }

    pub fn ease_both(x: f64) -> f64 {
        x
    }
}

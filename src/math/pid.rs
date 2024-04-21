pub struct Pid {
    pub kp: f32,
    pub ki: f32,
    pub kd: f32,
    e_prev: f32,
}

impl Clone for Pid {
    fn clone(&self) -> Self {
        Self {
            kp: self.kp,
            ki: self.ki,
            kd: self.kd,
            e_prev: 0.0,
        }
    }
}

impl Default for Pid {
    fn default() -> Self {
        Self {
            kp: 1.0,
            ki: 0.0,
            kd: 0.0,
            e_prev: 0.0,
        }
    }
}

impl Pid {
    pub fn new(kp: f32, ki: f32, kd: f32) -> Self {
        Pid {
            kp,
            ki,
            kd,
            e_prev: 0.0,
        }
    }

    fn integral(prev: f32, new: f32, dt: f32) -> f32 {
        if prev * new > 0.0 {
            // they are same
            // * =prev
            // |\
            // | \
            // |  \
            // |   * =new
            // |   |
            //0*---*
            //   ^dt length
        } else {
            // 2 triangles
            // * =prev
            // |\
            // | \
            //0*--X----*  =dt length
            //     \   |
            //      \  |
            //       \ |
            //        \|
            //         * =new
            // S = prev * 0.5 * x1 + new * 0.5 * x2 where x1 + x2 = dt
            // S = prev * 0.5 * dt + x2 * (new - prev) * 0.5
            // (0, prev) -> (dt, new)
            // k * x + c
            // S(k*x+c) = k/2*x^2 + c*x + j
            // S(dt) -> k/2*dt^2 + c*dt + j???
            // k = (new - prev)/dt
            // c = prev
            // j = ????
            // S(dt) = ((new - prev) / 2 + prev) * dt
        }
        ((new - prev) * 0.5 + prev) * dt
    }

    pub fn val(&mut self, current: f32, desired: f32, dt: f32) -> f32 {
        let e_now = desired - current;
        let diff = e_now - self.e_prev;
        let p = e_now * self.kp;
        let i = Self::integral(self.e_prev, e_now, dt) * self.ki;
        let d = diff / dt * self.kd;
        let value = p + i + d;
        self.e_prev = e_now;
        value
    }

    pub fn reset(&mut self) {
        self.e_prev = 0.0;
    }
}

#[test]
fn test_pid() {
    let process = |value, input| value + input;

    let desired = 1.0;
    let mut pid = Pid::new(0.1, 0.5, 0.00001);
    let mut value = 0.0;
    for _ in 0..100 {
        let dt = 0.0001;
        value = process(value, pid.val(value, desired, dt));
    }
    println!("{desired:?} {value:?}");
    assert!((desired - value).abs() < 0.0001);
}

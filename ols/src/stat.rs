pub mod statools {
    pub fn count_rss(v1: &Vec<f64>, v2: &Vec<f64>, b1: f64, b0: f64) -> f64 {
        let len = v1.len();
        let mut sum = 0.0;
        for i in 0..len {
            let delta = v2[i] - b0 - b1 * v1[i];
            sum += delta * delta;
        }
        return sum;
    }
    pub fn count_sse(v1: &Vec<f64>, v2: &Vec<f64>, b1: f64, b0: f64) -> f64 {
        let len = v1.len();
        let mut sum = 0.0;
        let mean_y = mean(v2);
        for i in 0..len {
            let delta = b0 + b1 * v1[i] - mean_y;
            sum += delta * delta;
        }
        return sum;
    }
    pub fn count_sst(v2: &Vec<f64>) -> f64 {
        let len = v2.len();
        let mut sum = 0.0;
        let mean_y = mean(v2);
        for i in 0..len {
            let delta = v2[i] - mean_y;
            sum += delta * delta;
        }
        return sum;
    }
    pub fn count_b1(v1: &Vec<f64>, v2: &Vec<f64>) -> f64 {
        return cov(v1, v2) / var(v1);
    }
    pub fn count_b0(v1: &Vec<f64>, v2: &Vec<f64>, b1: f64) -> f64 {
        let x_mean = mean(v1);
        let y_mean = mean(v2);
        return y_mean - b1 * x_mean;
    }
    pub fn cov(v1: &Vec<f64>, v2: &Vec<f64>) -> f64 {
        let len = v1.len();
        let m1 = mean(&v1);
        let m2 = mean(&v2);
        let mut sum = 0.0;
        for i in 0..len {
            sum += (v1[i] - m1) * (v2[i] - m2);
        }
        return sum / len as f64;
    }
    pub fn var(v: &Vec<f64>) -> f64 {
        let len = v.len();
        let m = mean(&v);
        let mut sum = 0.0;
        for i in 0..len {
            sum += (v[i] - m) * (v[i] - m);
        }
        return sum / len as f64;
    }
    pub fn mean(v: &Vec<f64>) -> f64 {
        let l = v.len();
        let mut s = 0.0;
        for i in v {
            s += i;
        }
        return s / l as f64;
    }
    pub fn min(v: &Vec<f64>) -> f64 {
        let mut min = 0.0;
        for i in v {
            if i < &min {
                min = *i;
            }
        }
        return min;
    }
}

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
    pub fn count_sse(x: &Vec<f64>, y: &Vec<f64>, b1: f64, b0: f64) -> f64 {
        let len = x.len();
        let mut sum = 0.0;
        let mean_y = mean(y);
        for i in 0..len {
            let y_hat = b0 + b1 * x[i];
            sum += (y_hat - mean_y) * (y_hat - mean_y);
        }
        return sum;
    }
    pub fn count_sst(y: &Vec<f64>) -> f64 {
        let len = y.len();
        let mut sum = 0.0;
        let mean_y = mean(y);
        for i in 0..len {
            let delta = y[i] - mean_y;
            sum += delta * delta;
        }
        return sum;
    }
    pub fn count_b1(x: &Vec<f64>, y: &Vec<f64>) -> f64 {
        return cov(x, y) / var(x);
    }
    pub fn count_b0(x: &Vec<f64>, y: &Vec<f64>, b1: f64) -> f64 {
        let mean_x = mean(x);
        let mean_y = mean(y);
        return mean_y - b1 * mean_x;
    }
    pub fn cov(x: &Vec<f64>, y: &Vec<f64>) -> f64 {
        if x.len() != y.len() || x.len() == 0 {
            return 0.0;
        }
        let len = x.len();
        let mean_x = mean(&x);
        let mean_y = mean(&y);
        let mut sum = 0.0;
        for i in 0..len {
            sum += (x[i] - mean_x) * (y[i] - mean_y);
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
    pub fn mean(values: &Vec<f64>) -> f64 {
        let l = values.len();
        let mut s = 0.0;
        for val in values {
            s += val;
        }
        return s / l as f64;
    }
    pub fn min(values: &Vec<f64>) -> f64 {
        let mut min = 0.0;
        for &val in values {
            if val < min {
                min = val;
            }
        }
        return min;
    }
}

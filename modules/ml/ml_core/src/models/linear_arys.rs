use ndarray::{array, Array1, Array2, ArrayView, concatenate, Axis, Ix2, s, Array};

pub struct LinearModelArys {
    pub W : Array1<f64>
}

impl LinearModelArys {
    pub fn _get_trained_variable(&self) -> &Array1<f64> {
        &self.W
    }

    // Gradiant descent for multiple features
    pub fn _fit(&mut self, X_train: Array2<f64>, Y_train: Array2<f64>, epoch: i32, alpha: f64, is_classification: bool) {

        //println!("ENTERING THE FIT {:?}, {:?}, {}, {}, {}", X_train, Y_train, epoch, alpha, is_classification);
        let m  = X_train.nrows(); // number of training example
        let mf : f64 = m as f64; // number of training example as float 64
        let bias = Array::<f64, Ix2>::from_elem((X_train.shape()[0], 1), 1.);

        let X_train = concatenate![Axis(1), bias, X_train];

        for _ in 0..epoch {
            let mut Wbis = self.W.clone();
            for j in 0..(self.W.shape()[0]) {
                let mut w = Wbis[j];
                let mut grad = 0.;
                //println!("j : {},\n{:?}", j, X_train);

                for i in 0..m {
                    //getting the training example features [i]
                    let X_i = X_train.slice(s![i, ..]);

                    //getting the training example output
                    let Y_i = &Y_train.slice(s![i, ..]);

                    //Hypotesis function : WᵗX
                    let mut hyp = self.W.dot(&X_i);
                    //println!("«loop : {}..{} -hypothesis {:?}»",i, m , hyp);

                    if is_classification {
                        hyp = sig(&hyp);
                    }

                    let loss = hyp - Y_i;
                    grad += loss[[0]] * X_i[j];
                }
                //should implement the cost function W = W - (α / n) Σ (WᵗX - Y) * X
                // for each W at the same time
                Wbis[j] = w - (alpha / mf) * grad;
            }
            // updating W with new values
            self.W = Wbis;
        }
        println!("new weight : {}", self.W);
    }


    pub fn predict(&self, x_predict : Array1<f64>) -> f64 {
        let bias : Array1<f64> = array![1.0];
        let x = concatenate![Axis(0), bias, x_predict];
        println!("PREDICT : x : {:?}, W : {:?}", x, self.W);

        self.W.dot(&x)
    }
}

fn sig(a: &f64) -> f64 {
    1.0 / (1.0 + (-a).exp())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_regression_1() {
        let x: Array2<f64> = array![
        [1.0, 8.0],
        [4.0, 2.0],
        [5.0, 6.0]
    ];
        let y : Array2<f64> = array![
        [0.],
        [0.],
        [1.0]
    ];

        let mut model = LinearModelArys { W: array![0.1, 0.1, 0.1]};

        model._fit(x, y, 50000, 0.01, true);
        let res1 = model.predict(array![1., 8.]);
        let res1_sig = sig(&res1);

        println!("{}", res1);
        println!("{}", res1_sig);

    }
}
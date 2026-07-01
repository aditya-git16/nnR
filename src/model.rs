// making a basic model 
// Initially only taking a single parameter w

pub struct Model {
    pub w : f64,
}

impl Model {
    pub fn new () -> Self {
        Model { w: 1.0 }
    }

    pub fn update(&mut self , actual_value : f64 , input : f64) {
        
        // learning rate
        let l_rate = 0.05;

        let w_new = self.w - l_rate*(2.0*(self.w*input - actual_value)*input);

        self.w = w_new;
    }

    pub fn prediction(&self , x : f64 ) -> f64 {
        return self.w * x;
    }
}

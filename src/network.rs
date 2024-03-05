use super::{matrix::Matrix, activations::Activation};

pub struct Network<'a>{
    layers: Vec<usize>,
    weights: Vec<Matrix>,
    biases: Vec<Matrix>,
    data: Vec<Matrix>,
    learning_rate: f64,
    activation: Activation<'a>,
}

impl Network<'_>{
    pub fn new<'a>(layers: Vec<usize>, learning_rate: f64, activation: Activation<'a>) -> Network{
        let mut weights = vec![];
        let mut biases = vec![];

        for i in 0..layers.len()-1{
            weights.push(Matrix::random(layers[i+1], layers[i]));
            biases.push(Matrix::random(layers[i+1], layers[i]));
        }

        Network{
            layers,
            weights,
            biases,
            data: vec![],
            learning_rate,
            activation,
        }
    }

    pub fn feed_forward(&mut self, inputs: Vec<f64>) -> Vec<f64> {
        if inputs.len() != self.layers[0]{
            panic!("Inavlid number of inputs");
        }

        let mut current = Matrix::from(vec![inputs]).transpose();
        self.data = vec![current.clone()];

        for i in 0..self.layers.len() - 1{
            current = self.weights[i]
            .multiply(&current)
            .add(&self.biases[i])
            .map(self.activation.function);

        self.data.push(current.clone());
        }

        current.transpose().data[0].to_owned()
    } 

    pub fn back_propagate(&mut self, outputs: Vec<f64>, targets: Vec<f64>){
        if targets.len() != self.layers[self.layers.len() -1]{
            panic!("Invalid argument count");
        }

        let mut parsed = Matrix::from(vec![outputs]);
        let mut errors = Matrix::from(vec![targets]).subtract(&parsed); //Error calc = Target - Output
        let mut gradients = parsed.map(self.activation.derivative);

        for i in (0..self.layers.len()-1).rev(){
            gradients = gradients
            .dot_product(&errors)
            .map(&|x| x * self.learning_rate);

            self.weights[i] = self.weights[i].add(&gradients.multiply(&self.data[i].transpose()));
            self.biases[i] = self.biases[i].add(&gradients);

            errors = self.weights[i].transpose().multiply(&errors);
            gradients = self.data[i].map(self.activation.derivative);
        }
    }

    pub fn train(&mut self, inputs: Vec<Vec<f64>>, targets: Vec<Vec<f64>>, epochs: u16){
        for i in 1..=epochs{
            if epochs < 100 || i % (epochs/100) == 0 {
                println!("Epoch {} of {}", i, epochs);
            }
            for j in 0..inputs.len() {
                let outputs = self.feed_forward(inputs[j].clone());
                self.back_propagate(outputs, targets[j].clone());
            }
        }
    }
}
use activations::SIGMOID;
use network::Network;

pub mod activations;
pub mod matrix;
pub mod network;


// XOR Test
// 0,0 -> 0
// 0,1 -> 1
// 1,0 -> 1
// 1,1 -> 0

fn main() {
    let inputs = vec![
        vec![0.0, 0.0],
        vec![0.0, 1.0],
        vec![1.0, 0.0],
        vec![1.0, 1.0],
    ];

    let targets = vec![
        vec![0.0],
        vec![1.0],
        vec![1.0],
        vec![0.0]
    ];
    
    let mut network = Network::new(vec![2,3,1], 0.5, SIGMOID);

    network.train(inputs, targets, 10000);

    println!("0 and 0 {:?}", network.feed_forward(vec![0.0, 0.0]));
	println!("0 and 1 {:?}", network.feed_forward(vec![0.0, 1.0]));
	println!("1 and 0 {:?}", network.feed_forward(vec![1.0, 0.0]));
	println!("1 and 1 {:?}", network.feed_forward(vec![1.0, 1.0]));
}

// Outputs
// 0 and 0 [0.0011161326398356243]
// 0 and 1 [0.9836577235752275]
// 1 and 0 [0.9854072704733727]
// 1 and 1 [0.020678408884407383]

use std::usize;

pub struct Buffer {
    pub text_layers: Vec<String>,
}
impl Buffer {
    pub fn new() -> Buffer {
        Buffer {
            text_layers: Vec::new(),
        }
    }
    pub fn write_text(&mut self, x: usize, y: usize, text: &String) {
        if self.text_layers.len() < y {
            self.add_text_layers_up_to(y);
        }
        self.text_layers[y].insert_str(x, text);
    }
    fn add_text_layers_up_to(&mut self, y: usize) {
        for _ in 0..(y + 1 - self.text_layers.len()) {
            self.text_layers.push("".to_string());
        }
    }
    pub fn remove_text(&mut self, layer: usize, from: usize, to: usize) {
        let layer_len = self.get_layer_len(layer);
        if from >= to || to > layer_len {
            return;
        }
        let split_output = self.text_layers[layer].split_at(from);
        let mut output_text = split_output.0.to_string();
        output_text += split_output.1.split_at(to - from).1;

        self.text_layers[layer] = output_text;
    }

    pub fn get_layer_len(&self, y: usize) -> usize {
        if self.text_layers.len() > y {
            return self.text_layers[y].len();
        }
        0
    }
}

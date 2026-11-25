// Expect: E0382
// ANCHOR: assoc_fn
struct TextBuffer {
    content: String,
}

impl TextBuffer {
    fn new(prefix: &str) -> Self {
        Self {
            content: String::from(prefix),
        }
    }
}
// ANCHOR_END: assoc_fn

// ANCHOR: three_receivers
impl TextBuffer {
    fn len(&self) -> usize {
        self.content.len()
    }

    fn append(&mut self, text: &str) {
        self.content.push_str(text);
    }

    fn into_string(self) -> String {
        self.content
    }
}
// ANCHOR_END: three_receivers

fn main() {
    let mut buf = TextBuffer::new("[INFO] ");
    buf.append("service ready");
    println!("len: {}", buf.len());

    let output = buf.into_string();
    println!("output: {output}");
    // println!("after into_string: {}", buf.len());
}

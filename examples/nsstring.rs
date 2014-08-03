extern crate cocoa;

use cocoa::base::ObjCMethodCall;

fn main() {
    let s = unsafe {
        "NSString"
            .send("stringWithUTF8String:", "Hello world!")
    };
    println!("Hello World: {}", s);
}

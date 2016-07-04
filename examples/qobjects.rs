#[macro_use]
extern crate qml;

use qml::*;

struct Test;

impl Test {
    pub fn launchGoose(&self, i: i32, i2: String) {
        println!("GOOSE HI from {} and {}", i2, i);
    }
}

Q_OBJECT!(
Test:
    signals:
         fn testname (a: i32, b: i32);

    slots:
         fn launchGoose(i: i32, launchText: String);
);

fn main() {
    let mut qqae = QmlEngine::new();
    let mut test = Box::new(Test);
    let qobj = QObject::new(&test, &mut qqae);
    test.testname(&qqae, 54, 55);
    test.qslot_call("launchGoose",
                    vec![42.into(), "QML Rust".to_string().into()]);
    println!("{:?}", test.qmeta());
}
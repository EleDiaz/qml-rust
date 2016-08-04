
#[macro_use] extern crate qml;
use qml::*;
pub struct SimpleList(Vec<i32>);

impl QModel for SimpleList {
    fn row_count(&self) -> i32 {
        self.0.len() as i32
    }

    fn data(&self, index: QModelIndex, role: i32) -> QVariant {
        QVariant::from(self.0[index.row() as usize])
    }

    fn roles_names(&self) -> Vec<String> {
        vec!["name".to_string(), "number".to_string()]
    }

    fn flags(&self, index: QModelIndex) -> i32 {
        0
    }
}

// ...

fn main() {
    let mut qqae = QmlEngine::new();
    let model = SimpleList(vec![1,74,7,8,75]);
    let mut qalm = QAbstractListModel::new(&model);
    qqae.set_property("listModel", &qalm.get_qvar());

    qqae.load_file("examples/listmodel.qml");
    qqae.exec();
    qqae.quit();
}

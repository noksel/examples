//#![windows_subsystem = "windows"] //switch off console...
mod form;
mod TodoWidget;
use qt_core::q_init_resource;
use qt_widgets::QApplication;

//use qt_gui::{QStandardItem, QStandardItemModel};
//use qt_widgets::{QApplication, QCheckBox, QListView, QPushButton, QRadioButton, QWidget};
//use std::collections::BTreeSet;
//use std::rc::Rc;

//use crate::TodoWidget;

fn main() {
	println!("ff");
    QApplication::init(|_| {
        q_init_resource!("resources");
        let todo_widget = TodoWidget::TodoWidget::new();
        todo_widget.show();
        unsafe { QApplication::exec() }
    })
}

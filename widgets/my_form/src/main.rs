//#![windows_subsystem = "windows"] //switch off console...

use cpp_core::{Ptr, StaticUpcast};
use qt_core::{
   // q_init_resource, qs, slot, CheckState, ItemDataRole, QBox, QObject, QPtr,
	q_init_resource, qs, slot, CheckState, QBox, QObject, QPtr,
    SlotNoArgs,
};
//use qt_gui::{QStandardItem, QStandardItemModel};
use qt_ui_tools::ui_form;
//use qt_widgets::{QApplication, QCheckBox, QListView, QPushButton, QRadioButton, QWidget};
use qt_widgets::{QApplication, QCheckBox, QPushButton, QRadioButton, QWidget};
//use std::collections::BTreeSet;
use std::rc::Rc;

#[ui_form("../ui/form.ui")]
#[derive(Debug)]
struct Form {
    widget: QBox<QWidget>,
    check_box: QPtr<QCheckBox>,
    radio_button: QPtr<QRadioButton>,
    push_button: QPtr<QPushButton>,
}

#[derive(Debug)]
struct TodoWidget {
    form: Form,
  //  source_model: QBox<QStandardItemModel>,
 //   proxy_model: QBox<QSortFilterProxyModel>,
}

impl StaticUpcast<QObject> for TodoWidget {
    unsafe fn static_upcast(ptr: Ptr<Self>) -> Ptr<QObject> {
        ptr.form.widget.as_ptr().static_upcast()
    }
}

impl TodoWidget {
    fn new() -> Rc<Self> {
        unsafe {
            let this = Rc::new(TodoWidget {
                form: Form::load(),
              //  source_model: QStandardItemModel::new_0a(),
              //  proxy_model: QSortFilterProxyModel::new_0a(),
            });
            this.init();
            this
        }
    }

    unsafe fn init(self: &Rc<Self>) {
		self.form
			.push_button
			.clicked()
			.connect(&self.slot_on_push_button_clicked());
		self.form.check_box.set_check_state(CheckState::Checked);
		self.form.radio_button.set_checked(true);
    }
	
	#[slot(SlotNoArgs)]
    unsafe fn on_push_button_clicked(self: &Rc<Self>) {
		println!("handle");
		self.form.push_button.set_text(&qs("New item"));
    }

    fn show(self: &Rc<Self>) {
        unsafe {
            self.form.widget.show();
        }
    }
}

fn main() {
	println!("ff");
    QApplication::init(|_| {
        q_init_resource!("resources");
        let todo_widget = TodoWidget::new();
        todo_widget.show();
        unsafe { QApplication::exec() }
    })
}

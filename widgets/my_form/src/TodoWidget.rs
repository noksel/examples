
//pub use self::TodoWidget;
use cpp_core::{Ptr, StaticUpcast};
use qt_core::{
   // q_init_resource, qs, slot, CheckState, ItemDataRole, QBox, QObject, QPtr,
	qs, slot, CheckState, QObject, 
    SlotNoArgs,
};
//use qt_gui::{QStandardItem, QStandardItemModel};
//use qt_widgets::{QApplication, QCheckBox, QListView, QPushButton, QRadioButton, QWidget};
//use std::collections::BTreeSet;
use std::rc::Rc;
//pub use Form;
use crate::form::Form;

#[derive(Debug)]
pub struct TodoWidget {
    form: Form,
}

impl StaticUpcast<QObject> for TodoWidget {
    unsafe fn static_upcast(ptr: Ptr<Self>) -> Ptr<QObject> {
        ptr.form.widget.as_ptr().static_upcast()
    }
}

impl TodoWidget {
    pub fn new() -> Rc<Self> {
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

    pub unsafe fn init(self: &Rc<Self>) {
		self.form
			.push_button
			.clicked()
			.connect(&self.slot_on_push_button_clicked());
		self.form.check_box.set_check_state(CheckState::Checked);
		self.form.radio_button.set_checked(true);
    }
	
	#[slot(SlotNoArgs)]
    pub unsafe fn on_push_button_clicked(self: &Rc<Self>) {
		println!("handle");
		self.form.push_button.set_text(&qs("New item"));
    }

    pub fn show(self: &Rc<Self>) {
        unsafe {
            self.form.widget.show();
        }
    }
}

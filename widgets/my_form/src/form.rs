use qt_ui_tools::ui_form;
use qt_widgets::{QCheckBox, QPushButton, QRadioButton, QWidget};
use qt_core::{QBox,QPtr};
#[ui_form("../ui/form.ui")]
#[derive(Debug)]
pub struct Form {
    pub widget: QBox<QWidget>,
    pub check_box: QPtr<QCheckBox>,
    pub radio_button: QPtr<QRadioButton>,
    pub push_button: QPtr<QPushButton>,
}
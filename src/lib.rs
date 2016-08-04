#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]
extern crate libc;
#[macro_use]
extern crate lazy_static;

mod qmlengine;
mod qvariant;
mod qabstractlistmodel;
mod qinthasharray;
mod utils;
mod qmodelindex;
mod types;
mod qurl;
mod qobject;
mod qmeta;
mod qtypes;
#[macro_use]
mod macros;
mod qmlregister;

pub use qmlengine::QmlEngine;
pub use qvariant::QVariant;
pub use qabstractlistmodel::{QModel, QAbstractListModel, QListModel};
pub use qmodelindex::QModelIndex;
pub use qobject::QObject;
pub use qmeta::{QObjectMacro, emit_signal};
pub use qtypes::*;
pub use qmlregister::QMLRegisterable;

#[doc(hidden)]
pub use libc::c_void;

#[doc(hidden)]
pub use qmlregister::{register_qml_type, register_qml_singleton_type};

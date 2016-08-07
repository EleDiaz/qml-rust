use libc;
use std::ptr::null_mut;

use qvariant::*;
use types::*;
use qmodelindex::*;
use qinthasharray::*;

extern "C" {

    fn dos_qabstractlistmodel_qmetaobject() -> DosQMetaObject;
    fn dos_qabstractlistmodel_create(callbackObject: *const libc::c_void,
                                     metaObject: DosQMetaObject,
                                     dObjectCallback: DObjectCallback,
                                     rowCountCallback: RowCountCallback,
                                     columnCountCallback: ColumnCountCallback,
                                     dataCallback: DataCallback,
                                     setDataCallback: SetDataCallback,
                                     roleNamesCallback: RoleNamesCallback,
                                     flagsCallback: FlagsCallback,
                                     headerDataCallback: HeaderDataCallback)
                                     -> DosQAbstractListModel;
    fn dos_qabstractlistmodel_beginInsertRows(vptr: DosQAbstractListModel,
                                              parent: DosQModelIndex,
                                              first: i32,
                                              last: i32);
    fn dos_qabstractlistmodel_endInsertRows(vptr: DosQAbstractListModel);

    fn dos_qabstractlistmodel_beginResetModel(vptr: DosQAbstractListModel);
    fn dos_qabstractlistmodel_endResetModel(vptr: DosQAbstractListModel);
}

/// Called when a slot should be executed
/// @param self The pointer to the `QObject` in the binded language
/// @param slotName The slotName as `DosQVariant`. It should not be deleted
/// @param argc The number of arguments
/// @param argv An array of `DosQVariant` pointers. They should not be deleted
pub type DObjectCallback = extern "C" fn(*const libc::c_void, DosQVariant, i32, *const DosQVariant);

/// Called when the `QAbstractListModel::rowCount` method must be executed
/// @param self The pointer to the `QAbstractListModel` in the binded language
/// @param index The parent `DosQModelIndex`. It should not be deleted
/// @param result The rowCount result. This must be deferenced and filled from the binded language.
/// It should not be deleted
pub type RowCountCallback = extern "C" fn(*const libc::c_void, DosQModelIndex, *mut i32);

/// Called when the `QAbstractListModel::columnCount` method must be executed
/// @param self The pointer to the `QAbstractListModel` in the binded language
/// @param index The parent `DosQModelIndex`. It should not be deleted
/// @param result The rowCount result. This must be deferenced and filled from the binded language.
/// It should not be deleted
pub type ColumnCountCallback = extern "C" fn(*const libc::c_void, DosQModelIndex, *mut i32);

/// Called when the `QAbstractListModel::data` method must be executed
/// @param self The pointer to the `QAbstractListModel` in the binded language
/// @param index The `DosQModelIndex` to which we request the data. It should not be deleted
/// @param result The `DosQVariant` result. This must be deferenced and filled from the binded language.
/// It should not be deleted. See `dos_qvariant_assign` or other `DosQVariant` setters
pub type DataCallback = extern "C" fn(*const libc::c_void, DosQModelIndex, i32, MutDosQVariant);

pub type SetDataCallback = extern "C" fn(*const libc::c_void,
                                         DosQModelIndex,
                                         DosQVariant,
                                         i32,
                                         *mut bool);

pub type RoleNamesCallback = extern "C" fn(*const libc::c_void, MutDosQHashIntQByteArray);

pub type FlagsCallback = extern "C" fn(*const libc::c_void, DosQModelIndex, *mut i32);

pub type HeaderDataCallback = extern "C" fn(*const libc::c_void, i32, i32, i32, MutDosQVariant);

/// All models for AbstractListModel must implement it. You can abstract your model
/// with whatever thing like a "list"
/// # Examples
/// ```
/// # #[macro_use] extern crate qml;
/// # use qml::*;
/// pub struct SimpleList(Vec<i32>);
///
/// impl QModel for SimpleList {
///     fn row_count(&self) -> i32 {
///         self.0.len() as i32
///     }
///
///     fn data(&self, index: QModelIndex, role: i32) -> QVariant {
///         QVariant::from("hodfadfadadfl".to_string())
///     }
///
///     fn roles_names(&self) -> Vec<String> {
///         vec![]
///     }
///
///     fn flags(&self, index: QModelIndex) -> i32 {
///         0
///     }
/// }
///
/// // ...
///
/// # fn main() {
/// let mut qqae = QmlEngine::new();
/// let model = SimpleList(vec![1,74,7,8,75]);
/// let mut qalm = QAbstractListModel::new(&model);
/// qqae.set_property("listModel", &qalm.get_qvar());
///
/// qqae.load_file("examples/listmodel.qml");
/// qqae.exec();
/// qqae.quit();
/// # }
/// ```
/// ```
pub trait QModel {

    /// Returns the number of rows
    fn row_count(&self) -> i32;

    // make a macro to generate a custom enum with defaults roles
    // and the new ones
    /// Returns the data stored under the given role for the item referred to by the index.
    /// Note: If you do not have a value to return, return an invalid QVariant instead of returning 0.
    /// http://doc.qt.io/qt-5/qabstractitemmodel.html#data
    fn data(&self, QModelIndex, i32) -> QVariant;

    /// Returns the new roles
    /// http://doc.qt.io/qt-5/qabstractitemmodel.html#roleNames
    fn roles_names(&self) -> Vec<String>;

    // TODO: return ItemFlags, no a integer
    /// Returns the item flags for the given index.
    /// The base class implementation returns a combination of flags that enables the item (ItemIsEnabled) and allows it to be selected (ItemIsSelectable).
    fn flags(&self, QModelIndex) -> i32;

}

use std::sync::atomic::{AtomicPtr, Ordering};

// TODO: Drop??
/// Defines a AbstractListModel with basic features
/// http://doc.qt.io/qt-5/qabstractitemmodel.html
pub struct QAbstractListModel<'a, T: 'a + QModel> {
    model: &'a T,
    wrapped: AtomicPtr<WQAbstractListModel>
}

impl<'a, T : QModel> QModel for QAbstractListModel<'a, T> {
    fn row_count(&self) -> i32{
        self.model.row_count()
    }

    fn data(&self, index : QModelIndex, role : i32) -> QVariant {
        self.model.data(index, role)
    }

    fn roles_names(&self) -> Vec<String> {
        self.model.roles_names()
    }

    fn flags(&self, index : QModelIndex) -> i32 {
        self.model.flags(index)
    }
}

extern "C" fn row_count_callback<T : QModel>(Qself: *const libc::c_void,
                                 index: DosQModelIndex,
                                 result: *mut i32) {
    unsafe {
        let qlist = &*(Qself as *const QAbstractListModel<T>);
        *result = qlist.row_count();
    }
}

extern "C" fn data_callback<T : QModel>(Qself: *const libc::c_void,
                       index: DosQModelIndex,
                       role: i32,
                       result: MutDosQVariant) {
    let qindex: QModelIndex = index.into();
    unsafe {
        let qlist = &*(Qself as *const QAbstractListModel<T>);
        let mut qvar: QVariant = result.into();
        qvar.set(&qlist.data(qindex, role));
    }
}

extern "C" fn role_names_callback<T : QModel>(Qself: *const libc::c_void, result: MutDosQHashIntQByteArray) {
    unsafe {
        let qlist = &*(Qself as *const QAbstractListModel<T>);
        let hash: QHashIntQByteArray = result.into();
        for (i, name) in qlist.roles_names().iter().enumerate() {
            hash.insert(START_ROLE + i as i32, name);
        }
    }
}

// TODO: unimplemented
extern "C" fn flags_callback<T : QModel>(Qself: *const libc::c_void,
                             index: DosQModelIndex,
                             result: *mut i32) {
    unsafe {
        let qlist = &*(Qself as *const QAbstractListModel<T>);
    }
}


impl<'a, T : QModel> QAbstractListModel<'a, T> {

    pub fn new(model: &'a T) -> Box<Self> {
        unsafe {
            let result = QAbstractListModel {
                wrapped: AtomicPtr::new(null_mut()),
                model: model
            };
            // Probably need an explanation on why do I need a box
            let mut boxer = Box::new(result);

            let dqmo = dos_qabstractlistmodel_qmetaobject();
            let dqalm =
                dos_qabstractlistmodel_create(&*boxer as *const QAbstractListModel<T> as *const libc::c_void,
                                              dqmo,
                                              RustObjectCallback, // no need
                                              row_count_callback::<T>,
                                              RustColumnCountCallback, // no need
                                              data_callback::<T>,
                                              RustSetDataCallback, // no need
                                              role_names_callback::<T>,
                                              flags_callback::<T>,
                                              RustHeaderDataCallback);// no need
            boxer.wrapped = AtomicPtr::new(dqalm);

            boxer
        }
    }

    /// Gets a `QVariant` associate
    pub fn get_qvar(&self) -> QVariant {
        self.wrapped.load(Ordering::Relaxed).into()
    }
}


extern "C" fn RustRowCountCallback(Qself: *const libc::c_void,
                                   index: DosQModelIndex,
                                   result: *mut i32) {
    unsafe {
        let qlist = &*(Qself as *const QListModel);
        *result = qlist.row_count() as i32;
    }
}

extern "C" fn RustColumnCountCallback(Qself: *const libc::c_void,
                                      parent: DosQModelIndex,
                                      result: *mut i32) {
}

extern "C" fn RustSetDataCallback(Qself: *const libc::c_void,
                                  index: DosQModelIndex,
                                  value: DosQVariant,
                                  role: i32,
                                  result: *mut bool) {
    println!("SET DATA HELLO");
}

extern "C" fn RustFlagsCallback(Qself: *const libc::c_void,
                                index: DosQModelIndex,
                                result: *mut i32) {
    println!("IVE GOT FLAGS CALLBACK");
}

extern "C" fn RustObjectCallback(Qself: *const libc::c_void,
                                 slotname: DosQVariant,
                                 argc: i32,
                                 argv: *const DosQVariant) {
    println!("SLOT WAS EXECUTED. hi");
}

extern "C" fn RustHeaderDataCallback(Qself: *const libc::c_void,
                                     section: i32,
                                     orientation: i32,
                                     role: i32,
                                     result: MutDosQVariant) {
    println!("FINAL CALLBACK");
}

extern "C" fn RustDataCallback(Qself: *const libc::c_void,
                               index: DosQModelIndex,
                               role: i32,
                               result: MutDosQVariant) {
    let qindex: QModelIndex = index.into();
    unsafe {
        let qlist = &*(Qself as *const QListModel);
        let data = &qlist.model[qindex.row() as usize][(role - START_ROLE) as usize];
        let mut qvar: QVariant = result.into();
        qvar.set(data);
    }
}

const START_ROLE: i32 = 0x0100;
extern "C" fn RustRoleNamesCallback(Qself: *const libc::c_void, result: MutDosQHashIntQByteArray) {
    unsafe {
        let qlist = &*(Qself as *const QListModel);
        let hash: QHashIntQByteArray = result.into();
        for (i, name) in qlist.rolenames.iter().enumerate() {
            hash.insert(START_ROLE + i as i32, name);
        }
    }
}

/// Allows providing a custom model to QML
pub struct QListModel<'a> {
    wrapped: AtomicPtr<WQAbstractListModel>,
    model: Vec<Vec<QVariant>>,
    rolenames: Vec<&'a str>,
}

impl<'a> QListModel<'a> {
    /// Rolenames are roles of provided data, that are mapped to corresponding roles in QML.
    pub fn new<'b>(rolenames: &'b [&'a str]) -> Box<Self> {
        unsafe {
            let mut rs = Vec::new();
            rs.extend_from_slice(rolenames);
            let result = QListModel {
                wrapped: AtomicPtr::new(null_mut()),
                model: Vec::new(),
                rolenames: rs,
            };
            // Probably need an explanation on why do I need a box
            let mut boxer = Box::new(result);

            let dqmo = dos_qabstractlistmodel_qmetaobject();
            let dqalm =
                dos_qabstractlistmodel_create(&*boxer as *const QListModel as *const libc::c_void,
                                              dqmo,
                                              RustObjectCallback, // no need
                                              RustRowCountCallback,
                                              RustColumnCountCallback, // no need
                                              RustDataCallback,
                                              RustSetDataCallback, // no need
                                              RustRoleNamesCallback,
                                              RustFlagsCallback, // no need
                                              RustHeaderDataCallback);// no need
            boxer.wrapped = AtomicPtr::new(dqalm);

            boxer
        }
    }

    /// Returns an amount of rows in this model
    pub fn row_count(&self) -> usize {
        self.model.len()
    }

    /// Gets a `QVariant` associate
    pub fn get_qvar(&self) -> QVariant {
        self.wrapped.load(Ordering::Relaxed).into()
    }

    /// Inserts a row into model
    ///
    /// Note that it clones all incoming qvariants as modifying them is not allowed.
    pub fn insert_row<T>(&mut self, qvars: T)
        where T: Iterator<Item = QVariant>
    {
        unsafe {
            let index = QModelIndex::new();
            dos_qabstractlistmodel_beginInsertRows(self.wrapped.load(Ordering::Relaxed),
                                                   get_model_ptr(&index),
                                                   self.model.len() as i32,
                                                   (self.model.len() + 1) as i32);
            self.model.push(qvars.collect());
            dos_qabstractlistmodel_endInsertRows(self.wrapped.load(Ordering::Relaxed));
        }
    }

    /// Sets a data for this QAbstractListModel
    pub fn set_data(&mut self, qvars: Vec<Vec<QVariant>>) {
        unsafe {
            dos_qabstractlistmodel_beginResetModel(self.wrapped.load(Ordering::Relaxed));
            self.model = qvars;
            dos_qabstractlistmodel_endResetModel(self.wrapped.load(Ordering::Relaxed));
        }
    }

    /// Gets an immutable view of the data
    pub fn view_data(&self) -> &[Vec<QVariant>] {
        &self.model
    }
}

impl<'a, 'b> From<&'a QListModel<'b>> for QVariant {
    fn from(i: &QListModel) -> QVariant {
        i.get_qvar()
    }
}



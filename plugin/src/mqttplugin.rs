// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Author: Simon Brummer (simon.brummer@posteo.de)
//         Jaap Geurts (jaap.geurts@gmail.com)
// Description: Implementation of the Rust backend.

use qmetaobject::{prelude::*, qml_register_singleton_type, QSingletonInit};
use std::ffi::{CStr, CString};

use crate::mqttconnection::MQTTConnection;

#[derive(Default, QObject)]
struct QmlPlugin {
    base: qt_base_class!(trait QQmlExtensionPlugin),
    plugin: qt_plugin!("org.qt-project.Qt.QQmlExtensionInterface"),
}


impl QQmlExtensionPlugin for QmlPlugin {
    fn register_types(&mut self, uri: &CStr) {
        let expected_uri = CString::new("org.kde.plasma.private.mqtt").unwrap();
        if expected_uri.as_c_str() != uri {
            panic!(
                "Error: Module URI is unexpected. Expected URI: {}, actual URI: {}",
                expected_uri.to_str().unwrap(),
                uri.to_str().unwrap()
            );
        }

        let qml_name = CString::new("MQTTPlugin").unwrap();
        qml_register_singleton_type::<MQTTPlugin>(uri, 1, 0, qml_name.as_c_str());
        let qml_name = CString::new("MQTTConnection").unwrap();
        qml_register_type::<MQTTConnection>(uri, 1, 0, qml_name.as_c_str());
    }

    //   fn initialize_engine(&mut self, engine: &mut QmlEngine, uri: &CStr) {
    //      println!("init engine");         
    //   }

}

#[allow(non_snake_case)]
#[derive(Default, QObject)]
pub struct MQTTPlugin {
    // Define "base" - class
    base: qt_base_class!(trait QObject),

    // return an MQTT connection
    // getConnection: qt_method!(fn(&mut self, name: String, topic: String) -> QJSValue),

   
}

impl MQTTPlugin {
    // TODO: fix. doedsn't work currently, because there is no access to qml engine
    // #[allow(non_snake_case)]
    // pub fn getConnection(&mut self, name: String, topic: String) -> QJSValue {
    //  //   return QVariant::from(QObjectPinned::new(&MQTTConnection::new(name, topic)));
    //  //return QmlEngine::new_qobject(self, MQTTConnection::new(name,topic));
    //  //QVariant::from(QObjectPinned::new(RefCell::from(MQTTConnection::new(name,topic))));
    // }

}

impl QSingletonInit for MQTTPlugin {
    fn init(&mut self) {

       // nothing to do for now.
    }
}

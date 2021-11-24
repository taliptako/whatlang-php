extern crate whatlang;

use whatlang::{detect};
use ext_php_rs::prelude::*;
use std::collections::HashMap;
use ext_php_rs::boxed::ZBox;
use ext_php_rs::types::ZendObject;


#[php_function]
pub fn detectLang(text: &str) -> ZBox<ZendObject> {
    let info = detect(text).unwrap();

    let mut obj = ZendObject::new_stdclass();

    obj.set_property("lang", info.lang().name());
    obj.set_property("script", info.script().name());
    obj.set_property("confidence", info.confidence());
    obj.set_property("is_reliable", info.is_reliable());

    obj
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
}
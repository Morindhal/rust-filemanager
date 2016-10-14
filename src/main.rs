extern crate filemanager_core;

use std::path::Path;
// rfcf is (R)ust (F)ilemanager (C)ore (F)unctions
use filemanager_core::filemanager_core_functions as rfcf;
// rfcs is (R)ust (F)ilemanager (C)ore (S)tructs
use filemanager_core::filemanager_core_structs as rfcs;



fn main()
{
    let path = Path::new("/home/");
    let mut displaytype = rfcs::DisplayType{ ..Default::default() };
    rfcf::printout(path, displaytype);
}

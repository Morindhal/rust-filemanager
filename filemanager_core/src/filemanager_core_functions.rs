
use std::path::Path;
use filemanager_core_structs as rfcs;

pub fn printout(path:  &Path, displaytype: rfcs::DisplayType)
{
    match path.read_dir()
    {
        Err(e) =>  println!("{}",e),
        Ok(k) => for paths in k
        {
            match paths
            {
                Err(e2) =>  println!("{}",e2),
                Ok(k2) =>
                    if k2.path().is_dir() && displaytype.show_folder { println!("{} <-- folder",k2.path().file_name().unwrap().to_str().unwrap()) }
                    else if k2.path().is_file() && displaytype.show_files { println!("{} <-- file",k2.path().display()) }
            }
        }
    }
}

/*
* notes:
* .path().file_name().unwrap().to_str().unwrap()            <-- gets the filename, add try! or match code instead of unwrap when used -- 
* 
**/

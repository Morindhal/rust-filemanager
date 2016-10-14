use std::path::Path;


struct DisplayType
{
    show_folder: bool,
    show_hidden: bool, //This will likely be a OS-specific thing later, so at this time it is unimplemented.
    show_files: bool,
}


fn main()
{
    let path = Path::new("/home/");
    let displaytype = DisplayType{show_folder: true, show_hidden: false, show_files: true};
    printout(path, displaytype);
}

fn printout(path:  &std::path::Path, displaytype: DisplayType)
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

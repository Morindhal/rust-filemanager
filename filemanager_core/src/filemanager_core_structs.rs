

impl Default for DisplayType
{
    fn default() -> DisplayType
    {
        DisplayType { show_folder: true, show_hidden: false, show_files: true }
   }
}


pub struct DisplayType
{
    pub show_folder: bool,
    pub show_hidden: bool, //This will likely be a OS-specific thing later, so at this time it is unimplemented.
    pub show_files: bool,
}

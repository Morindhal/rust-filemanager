#[derive(Copy)]
pub struct DisplayType
{
    pub show_folder: bool,
    pub show_hidden: bool, //This will likely be a OS-specific thing later, so at this time it is unimplemented.
    pub show_files: bool,
}

impl Default for DisplayType
{
    fn default() -> DisplayType
    {
        DisplayType { show_folder: true, show_hidden: false, show_files: true }
   }
}

impl Clone for DisplayType
{
    fn clone(&self) -> DisplayType { *self }
}

use std::fmt;
impl fmt::Debug for DisplayType
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "
                    Show Folders: {}
                    Show Hidden Files: {}
                    Show Files: {}
                    ", self.show_folder, self.show_hidden, self.show_files)
    }
}

// This will likely never be used, I only implemented this for educational purposes. I should remember to remove this when I continue this project.
impl fmt::Display for DisplayType
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "({}, {}, {})", self.show_folder, self.show_hidden, self.show_files)
    }
}

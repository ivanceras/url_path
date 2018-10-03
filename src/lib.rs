//! url_path
//! Manipulate url paths without requiring the file to exist in the server or OS
//! This is useful for manipulating location urls
//! Example usage:
//! ```rust
//! use url_path::UrlPath;
//!
//! fn main(){
//!     let url_path1 = UrlPath::new("src/md/./../../README.md");
//!     let normalized_path1 = url_path1.normalize();
//!     assert_eq!("README.md", normalized_path1);
//!
//!     let url_path2 = UrlPath::new("./README.md");
//!     let normalized_path2 = url_path2.normalize();
//!     assert_eq!("README.md", normalized_path2);
//! }
//! ```
pub struct UrlPath{
    pub parent: Option<String>,
    /// the last element of the url when split with `/`
    pub last: Option<String>,
    is_absolute: bool,
}

impl UrlPath{

    pub fn new(path: &str) -> Self {
        let (parent, last) = Self::canonicalize(path);
        let is_absolute = path.starts_with("/");
        UrlPath{
            parent,
            last,
            is_absolute,
        }
    }

    pub fn is_absolute(&self) -> bool {
        self.is_absolute
    }

    /// use own implementation of canonicalize since fs::canonicalize
    /// requires the file to be there
    fn canonicalize(path: &str) -> (Option<String>, Option<String>) {
        let segments:Vec<&str> = path.split("/").collect();
        let mut path:Vec<String> = vec![];
        let segments2:Vec<&str> = segments.into_iter()
                .filter(|s|!(s.is_empty() || *s == ".")).collect();
        let _filtered:Vec<&str> = segments2.into_iter()
            .inspect(|s| 
                 if *s == ".."{
                    path.pop();
                 }else{
                    path.push(s.to_string())
                 }).collect();
        let filename = path.pop();
        let parent = path.join("/");
        let parent = if parent.is_empty(){
            None
        }else{
            Some(parent)
        };
        (parent, filename)
    }


    pub fn normalize(&self) -> String {
        let full_path = if let Some(ref parent) = self.parent {
            if let Some(ref file) = self.last{
                format!("{}/{}", parent, file)
            }else{
                format!("{}", parent)
            }
        }
        else if let Some(ref file) = self.last{
            if let Some(ref parent) = self.parent{
                format!("{}/{}", parent, file)
            }else{
                format!("{}", file)
            }
        }
        else{
            "".to_string()
        };

        if self.is_absolute{
            format!("/{}", full_path)
        }else{
            full_path
        }
    }
}




#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let url = "md/README.md";
        let path = UrlPath::new(url);
        let result = path.normalize();
        let expected = "md/README.md";
        assert_eq!(expected, result);
    }

    #[test]
    fn relative_file(){
        let url = "./README.md";
        let path = UrlPath::new(url);
        let result = path.normalize();
        let expected = "README.md";
        assert_eq!(expected, result);
    }

    #[test]
    fn absolute() {
        let url = "/home/user/md/README.md";
        let path = UrlPath::new(url);
        let result = path.normalize();
        let expected = "/home/user/md/README.md";
        assert_eq!(expected, result);
    }

    #[test]
    fn absolute_with_dotdot() {
        let url = "/home/user/md/../../README.md";
        let path = UrlPath::new(url);
        let result = path.normalize();
        let expected = "/home/README.md";
        assert_eq!(expected, result);
    }

    #[test]
    fn normalize1() {
        let url = "md/more/../README.md";
        let path = UrlPath::new(url);
        let result = path.normalize();
        let expected = "md/README.md";
        assert_eq!(expected, result);
    }

    #[test]
    fn normalize2() {
        let url = "md/../README.md";
        let path = UrlPath::new(url);
        let result = path.normalize();
        let expected = "README.md";
        assert_eq!(expected, result);
    }

    #[test]
    fn no_parent() {
        let url = "README.md";
        let path = UrlPath::new(url);
        let last = "README.md";
        assert_eq!(Some(last.to_string()), path.last);
        assert_eq!(None, path.parent);
    }

    #[test]
    fn no_parent2() {
        let url = "md/../README.md";
        let path = UrlPath::new(url);
        let last = "README.md";
        assert_eq!(Some(last.to_string()), path.last);
        assert_eq!(None, path.parent);
    }

    #[test]
    fn normalize_no_more_back() {
        let url = "../../README.md";
        let path = UrlPath::new(url);
        let result = path.normalize();
        let expected = "README.md";
        assert_eq!(expected, result);
    }
}

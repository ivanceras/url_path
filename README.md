# url_path

url_path
Manipulate url paths without requiring the file to exist in the server or OS
This is useful for manipulating location urls
Example usage:
```rust
use url_path::UrlPath;

fn main(){
    let url_path1 = UrlPath::new("src/md/./../../README.md");
    let normalized_path1 = url_path1.normalize();
    assert_eq!("README.md", normalized_path1);

    let url_path2 = UrlPath::new("./README.md");
    let normalized_path2 = url_path2.normalize();
    assert_eq!("README.md", normalized_path2);
}
```

License: MIT

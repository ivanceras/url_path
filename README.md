# url_path

url_path
Manipulate url paths without requiring the file to exist in the server or OS
This is useful for manipulating location urls
Example usage:
```rust
use url_path::UrlPath;

fn main(){
    let url_path = UrlPath::new("src/md/./../../README.md");
    let normalized_path = url_path.normalize();
    assert_eq!("README.md", normalized_path);
}
```

License: MIT

# Interface5 Request
A simple Interface5-WebserviceInput Rust Client.

## Examples
```rust
use i5_req::{
    request::blocking::i5_http_post,
    types::{i5_request::I5Reqeust, i5_request_url::I5RequestUrl},
};
use std::fs;

fn main() {
    let i5_url: I5RequestUrl =
        I5RequestUrl::new("ip/hostname", 43001, "Scenario", "Tenant");
    let mut i5_reqest: I5Reqeust = I5Reqeust::new("newInterfaceRequest");
    let id: usize = i5_reqest.add_document("Document1");

    let test_file = fs::read("test.csv").unwrap();

    i5_reqest
        .get_document_mut(id)
        .unwrap()
        .add_header_field("InvoiceNumber", "3309979202")
        .add_item_field("Amount", "546", 1)
        .add_bytes_file("newStatus.csv", &test_file);

    let validated = i5_reqest.validate().unwrap();

    let res = i5_http_post(validated, i5_url, true).unwrap();
    println!("{:#?}", res)
}
```

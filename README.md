# Description 📄

Convert your CSV data to JSON.

JSONweaver will take your CSV data from your `.csv ` file convert it into JSON and then host it on `http://localhost:8000/` using async web framework Rocket.

## Setup and run main.rs file: -

1. Clone the repository:

```
git clone <repo-link>
```

2. Change directory:

```
cd JSONweaver
```

3. Put your csv file in `/public`
   > Now, follow these further steps to add custom code for your own csv file. 🐾

- Change the path and name of the csv file.

```rust
match read_from_file("./public/organizations.csv") {
    Ok(main_array) => {
        fill_json(main_array)
    } Err(e) => e.to_string()
}
```

- Change the `Head struct` in **/src/organization.rs** file according to your headers in csv file.

```rust
struct Head {
    index: String,
    organization_id: String,
    name: String,
    website: String,
    country: String,
    description: String,
    founded: String,
    industry: String,
    number_of_employees: String,
}
```

- Change the `fill_json() method` in **/src/organization.rs** file, again according to your headers in csv file.

```rust
let head: Head = Head {
    index: format!("{}", content[0]),
    organization_id: format!("{}", content[1]),
    name: format!("{}", content[2]),
    website: format!("{}", content[3]),
    country: format!("{}", content[4]),
    description: format!("{}", content[5]),
    founded: format!("{}", content[6]),
    industry: format!("{}", content[7]),
    number_of_employees: format!("{}", content[8]),
};
```

4. Run the file using this command:

```
cargo run
```

> This command will host your JSON data to `http://localhost:8000/`

_Note: These requirements are staged for version 0.1.0_

### LICENSE

MIT License

Copyright (c) 2023 Pratham - heylightning/JSONweaver

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

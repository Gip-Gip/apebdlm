# ApeBDLM
## Binary-Data-Layout-Macros, macros to assist in making the creation of binary data vectors more readable.

When creating this package, I had one problem I needed to solve; there is no clean way to organize binary data before storing it to a file.

You *could* just write everyting sequentially, or push everything to a vector and write that(what I was doing) but that tends to get ugly and while not hard to read I feel it wouldn't be hard to accidentally screw up the order of an important byte of data in a header, per se

For example, if you wanted to write three strings to a file you could do something like this:

```
let data = Vec::<u8>::new();

data.push(len1);
data.extend_from_slice(string1);
data.push(len2);
data.extend_from_slice(string2);
data.push(len3);
data.extend_from_slice(string3);

file.write_all(&data)?;
```

but, personally, this is fairly ugly, hard to distiguish from other code, and not too well structured for what is a very structured part of a file.

So I created a set of macros that can be used to structure binary data in a **safe, structured, and readable** way. Here is the same code above but with ApeBDLM:

```
let data = binary_data!
(
    byte!(len1),
    bytes_from_vec!(string1),
    byte!(len2),
    bytes_from_vec!(string2),
    byte!(len3),
    bytes_from_vec!(string3)
);

file.write_all(&data)?;
```

The data is not only better structured and separated from the code, but if there is a mistake or a mistructuring of any of the data, it will be easy to find and fix it due to the lack of unnessicary .push or .extend_from_slice code clogging up your eyeholes.

## Usage

This package allows the simple and structured creation of binary data vectors, and being so simple it is fairly easy to use. the ```binary_data!``` macro basically just creates a new vector and converts+adds all of the data in the parenthesis to the vector. **Note** only the other macros provided by this library like ```byte!``` and ```i32_be!``` are allowed as arguments to the ```binary_data!``` macro.

Here's an example of code that I've adapted to utilize this library:

```
import apebdlm::*;

// Lines 2-528 stay the same

pub(crate) fn write_chunk<W: Write>(mut w: W, name: chunk::ChunkType, data: &[u8]) -> Result<()> {
    let mut crc = Crc32::new();
    crc.update(&name.0);
    crc.update(data);

    let buffer = binary_data!(
        u32_be!(data.len),
        bytes_from_vec!(&name.0),
        bytes_from_vec!(data),
        u32_be!(crc.finalize())
    );

    w.write_all(&buffer)?;

    Ok(())
}

// Etc...
```

*Code example thanks to the rust png library, specifically lines [528-538](https://github.com/image-rs/image-png/blob/51e580da923d9e8ecfc4dd5d22f9ae09595b0f46/src/encoder.rs#L529-L538) have been adapted for this example.*
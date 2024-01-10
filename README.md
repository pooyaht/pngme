## Commands

### Encode a secret into a file

```bash
pngme encode <FILE_PATH> <CHUNK_TYPE> <MESSAGE>
```

Example:

```bash
pngme encode ./myfile.png sEcr "Hello, this is a PNG file secret"
```

### Decode a secret from a file

```bash
pngme decode <FILE_PATH> <CHUNK_TYPE>
```

Example:

```bash
pngme decode ./myfile.png sEcr
```

### Remove a secret from a file

```bash
pngme remove <FILE_PATH> <CHUNK_TYPE>
```

Example:

```bash
pngme remove ./myfile.png sEcr
```

### Print chunks

```bash
pngme print <FILE_PATH>
```

Example:

```bash
pngme print ./myfile.png
```


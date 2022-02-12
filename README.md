# swc-plugin-negative-indices
A plugin for swc to transform 
```let a = arr[-1];```
to
```let a = arr[arr.length - 1];```
## Usage
download .wasm from release
.swcrc
```json
{
  "jsc": {
    "experimental": {
      "plugins": [
        [
          "absolute path to .wasm",
          {}
        ]
      ]
    }
  }
}
```
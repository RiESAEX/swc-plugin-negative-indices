# swc-plugin-negative-indices
A plugin for swc to transform 
```let a = arr[-1];```
to
```let a = arr[arr.length - 1];```
## Usage
```npm i swc-plugin-negative-indices```  
.swcrc
```json
{
  "jsc": {
    "experimental": {
      "plugins": [
        [
          "swc-plugin-negative-indices",
          {}
        ]
      ]
    }
  }
}
```
## Compatibility
I made this just for fun, let's see how many breaks unitl swc rust plugin api stable

|plugin|@swc/core|
|--|--|
|1.0.5|1.2.120|

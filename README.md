# Overview

QuickShotMessage generator

## Detail

### Message File Format (for generate)

<MESSAGE_FILE_MANE>.qsmb
```
msg <MESSAGE_NAME>
{
  <TYPE_NAME> <VALUE_NAME>
}

```

### Example

ExampleMessage.qsmb
```
msg ExampleMessage
{
  Integer intVal
  Float  floatVal
  String stringVal
}
```

### Generate Output Result


#### cpp

QS_ExampleMessage.hpp
```
```

#### Rust
QS_ExampleMessage.rs
```
```


## Command Usage

qnerator -d <TARGET_FILE_DIRECTORY> <GENERATE_LANGUAGE> <GENERATE_DIRECTORY>

### Example

Example 1: qnerator -d /targets rust - => will generate /gen directory

Example 2: qnerator -d /targets rust /custom => will generate /custom directory












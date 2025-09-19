# SREMP Specification

- **Version 0.1**

# Notation

This specification uses Rust- and ABNF-inspired pseudocode for structured data definitions. [^1]

## 1 Definition Syntax

### 1.1 Primitive Types

- `bool`: Boolean value (true/false)
- `char`: A single UTF8 character
- `u8`, `u16`, `u32`, `u64`, `u128`: Unsigned integers of specified bit length
- `i8`, `i16`, `i32`, `i64`, `i128`: Signed integers of specified bit length
- `String`: UTF-8 encoded text string

### 1.2 Defined Datastructures

**Derived Type**

```
DerivedTypeName := InnerType(Constraint)
```

**Struct**

```
StructName := {
    field_name: FieldType(Constraint),
    optional_field: Optional<Type>
}
```

**Enumeration**

```
EnumName := Variant1 | Variant2 | Variant3
```

### 1.3 Constants

- `CONST_NAME: Type := Value` Defines a value of a certain type that never changes

### 1.4 Composite Types

- `[T]`: Array of type T
- `List<T>`: Ordered collection of type T
- `Optional<T>`: Nullable field of type T
- `Map<K, V>`: Key-value mapping from type K to type V
- `Result<T, E>`: Success value T or error E
- `RefCounted<T>`: Automatically reference counted T
- `Mutex<T>`: A mutual exclusion primitive useful for protecting shared data T

### 1.5 Time Types

- `DateTime<Utc>`: Time and Date with the universal timezone "UTC"
- `DateTime<Local>`: Time and Date with the local timezone of a user, like "CET"
- `Duration`: Time span/interval

### 1.6 Network Types

- `SocketAddr`: Network socket address (IP + port)
- `DNSName`: Domain name string
- `IPAddress`: IPv4 or IPv6 address

### 1.7 Cryptographic Types

- `Ed25519PublicKey`: 32-byte Ed25519 public key
- `Ed25519PrivateKey`: 32-byte Ed25519 private key
- `Ed25519Signature`: 64-byte Ed25519 signature
- `VerifyingKey`: Alias for Ed25519PublicKey
- `SigningKey`: Alias for Ed25519PrivateKey

### 1.8 Protocol Types

- `MessageId`: Unique message identifier, derived over `u32`

### 1.9 Operations

- `||`: Byte sequence concatenation
- `:=`: Type definition assignment
- `=`: Value assignment
- `==`, `!=`: Equality/inequality comparison
- `<`, `>`, `<=`, `>=`: Ordering comparison
- `&&`, `||`, `!`: Logical AND, OR, NOT
- `+`, `-`, `*`, `/`, `%`: Arithmetic operations
- `&`, `|`, `^`: Bitwise AND, OR, XOR
- `<<`, `>>`: Bit shift left/right
- `.`: Field access
- `[index]`: Indexing
- `1..40`: Range
- `1..=40`: Inclusive range

## 2 Conventions

## 2.1 Naming

- Field names use `snake_case`
- Type names use `PascalCase`
- Constants use `UPPER_CASE`

## A References

[^1]: ABNF is defined in RFC 5234 of the IETF: <https://www.rfc-editor.org/rfc/rfc5234>

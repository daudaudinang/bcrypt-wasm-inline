# bcrypt-wasm
bcrypt-wasm is a WebAssembly (WASM) binding for the popular bcrypt hashing algorithm, designed to be used directly in JavaScript applications. This package provides both synchronous and asynchronous methods for hashing passwords and verifying password hashes.



## Features
* Sync and Async Methods: Both synchronous and asynchronous methods to hash passwords and compare them with hashed values.
* Fast and Secure: Utilizes the bcrypt algorithm, which is well-known for its strong security and resistance to brute-force attacks.
* WASM-based: By compiling to WebAssembly, it can be used efficiently in web browsers and Node.js environments.
* Optimized Build with Rollup: The package uses Rollup for bundling, allowing for efficient and optimized JavaScript output.
* Inline Base64-encoded WASM: The WASM module is bundled as an inline base64-encoded JavaScript string to increase compatibility with different environments, especially where file system access might be limited.

## Installation
To install the package, run:
```bash
  npm install bcrypt-wasm
```
This package is built using WebAssembly and exposes hash and compare functions, making it usable in both Node.js and browser environments.

## Usage
Once installed, you can use the functions directly in your JavaScript code.

### Importing
In a modern JavaScript or TypeScript environment, import the functions like so:

``` js
import { hash, hashSync, compare, compareSync } from 'bcrypt-wasm';
```
### Synchronous Hashing (Sync)
To hash a password synchronously, use the hashSync function. This is the fastest option but will block the main thread, so it is recommended to use it only for non-critical operations or small datasets.

```js
const password = 'mySuperSecretPassword';
const saltRounds = 10;

try {
const hashedPassword = hashSync(password, saltRounds);
console.log("Hashed password:", hashedPassword);
} catch (error) {
console.error("Error hashing password:", error);
}
```

### Asynchronous Hashing (Async)
For asynchronous operations, use the hash function. This will return a Promise, allowing you to handle it asynchronously.

``` js
const password = 'mySuperSecretPassword';
const saltRounds = 10;

hash(password, saltRounds)
.then(hashedPassword => {
console.log("Hashed password:", hashedPassword);
})
.catch(error => {
console.error("Error hashing password:", error);
});
```

### Synchronous Password Comparison (Sync)
To compare a plaintext password with a hashed password synchronously, use the compareSync function.

```js
const password = 'mySuperSecretPassword';
const hashedPassword = '$2a$10$U2f2A4pJK6vwp7gRjBqFzuM0Iz6dHzvvwfi61QWQW9Fe5CevahwSi';

const isMatch = compareSync(password, hashedPassword);
console.log("Password match:", isMatch); // true or false
```

### Asynchronous Password Comparison (Async)
For asynchronous password comparison, use the compare function. It returns a Promise that resolves to true or false depending on whether the password matches the hash.

```js
const password = 'mySuperSecretPassword';
const hashedPassword = '$2a$10$U2f2A4pJK6vwp7gRjBqFzuM0Iz6dHzvvwfi61QWQW9Fe5CevahwSi';

compare(password, hashedPassword)
.then(isMatch => {
console.log("Password match:", isMatch); // true or false
})
.catch(error => {
console.error("Error comparing password:", error);
});
```

## API
`hashSync(data: string, saltOrRounds: number | undefined): string`
* Parameters:
  * data: The plaintext password to be hashed.
  * saltOrRounds: Optional. The number of salt rounds to use. If omitted, it defaults to 10.
* Returns: The hashed password as a string.

`compareSync(data: string, encrypted: string): boolean`
* Parameters:
  * data: The plaintext password to check.
  * encrypted: The hashed password to compare against.
* Returns: true if the password matches the hash, otherwise false.

`hash(data: string, saltOrRounds: number | undefined): Promise<string>`
* Parameters:
  * data: The plaintext password to be hashed.
  * saltOrRounds: Optional. The number of salt rounds to use. If omitted, it defaults to 10.
* Returns: A Promise that resolves to the hashed password as a string.

`compare(data: string, encrypted: string): Promise<boolean>`
* Parameters:
  * data: The plaintext password to check.
  * encrypted: The hashed password to compare against.
* Returns: A Promise that resolves to true if the password matches the hash, otherwise false.

## WebAssembly
This package is built using WebAssembly, providing a fast and secure way to perform password hashing directly in the browser or Node.js.

### Browser Support
This package is designed to be used in modern browsers that support WebAssembly. If you are using older browsers that do not support WebAssembly, you may need to polyfill or use a different hashing library.

### Node.js Support
This package can also be used in Node.js, which supports WebAssembly natively from version 12.x and above.
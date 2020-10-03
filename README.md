# The `idioma` Library

As Rust developers we deeply care about safety and error handling - our programs
are fast and reliable. However, users never make it easy for us: they
misunderstand instructions and break things. When we catch them doing something
they shouldn't be doing, we let them know (usually) with an error message.

Every command line tool prints handy messages to `stdout` from time to time, and
to do so, requires a function or two. I noticed that whenever I start a new
project I tend to copy the `util.rs` that contains those display functions from
my last project. That is simply no good.

It means that my error messages

- differ in style (since I regularly alter code in that util file);
- don't look like idiomatic Rust messages;
- require that `copy + paste` operation for every new project.

And I strongly believe that I am not alone in this. Take a look at
[this code][1] by [brain-lang]:

```rust
macro_rules! exit_with_error(
    ($($arg:tt)*) => { {
        use std::process;
        eprintln!($($arg)*);
        process::exit(1);
    } }
);
```

[1]: https://github.com/brain-lang/brainfuck/blob/master/src/bin/brainfuck.rs#L21
[brain-lang]: https://github.com/brain-lang/

As you can see, they wrote this macro right next to the `main` function and it
is the same problem that I have with my util file. The `idioma` library solves
all these issues forever. Here's how.



## Use me, baby!

In your `Cargo.toml` file.

```toml
[dependencies]
idioma = "*"
```

Include in any Rust file.

```rust
extern crate idioma;
```

Use within a function.

```rust
fn foo(i: i32) {
    if i != 42 {
        idioma::exit_with(error, "Your taste is appalling.")
    }
}
```



## Development

You are more than welcome to contribute to this library. Same as always:

- Fork;
- Change;
- Pull Request.

I will do my best to review requests as soon as possible. If you write a new
function or something, make sure to include [doc comments] with some of that
spicy humour!

[doc comments]: https://doc.rust-lang.org/stable/rust-by-example/meta/doc.html

To see what's already there, visit [docs.rs] where you can see full
documentation with links to source code.

[docs.rs]: https://docs.rs/idioma/



## License

I don't know why, but this code is licensed under the MIT license which means
the following:

```
THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```



-------------------------------------------------------------------------------

I know that it's cringe, but I really made it with ❤️ so 😘 bye-bye.

## Tools, that were used
<ol>
<li>rustc version: rustc 1.17.0 </li>
<li>Atom ide with rust plugins enabled </li>
</ol>

### How to compile .rs file
<pre>
 // Suppose we have HelloWorld.rs file
 //cat HelloWorld.rs
 fn main() {
  println!("Hello World");
 }

 //compile .rs file into executable file
 rustc Example1.rs -A warnings
 //now run ./Example1 file
./Example1
</pre>




//atom packages :
//racer has bugs , so i had to delete this plugin
language-rust, linter-rust, <s>racer</s>

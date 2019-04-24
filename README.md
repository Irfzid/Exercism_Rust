# Exercism_Rust
Esai ini dibuat untuk memenuhi tugas dari mata kuliah Operasi sistem semester 110, Ilmu Komputer, Universitas Negeri Jakarta, membahas tentang cara menyelesaikan suatu problem Rust yang ada pada Exercism.io.
***

# Triangle 
problem ini meminta untuk menentukan bentuk segitiga dari data sisi yang diberikan, apakah segitiga sama kaki, segitiga sama sisi, atau segitiga sembarang.

+ Equilateral  : segitiga yang memiliki tiga sisi yang sama panjang.
+ Isosceles    : segitiga yang memiliki dua sisi yang sama panjang.
+ scalene      : segitiga yang semua panjang sisinya berbeda.

# Solusi 
Untuk menyelesaikan problem ini, kita harus mengecek panjang setiap sisi yang diberikan harus lebih besar dari 0, dan jumlah panjang dari 2 sisi harus lebih besar atau sama dengan panjang sisi yang ketiga. Setelah sudah dicek bahwa bentuk tersebut adalah segitiga, kita harus mengecek lagi apakah itu segitiga sama sisi (Equilateral), segitiga sama kaki (Isoceles), atau segitiga sembarang (Scalene).

pertama kita buat struct terlebih dahulu untuk menampung nilai dari sisi yang sudah dicek agar dapat digunakan pada function lain.
```rust
pub struct Triangle{
    a: u64,
    b: u64,
    c: u64,
}
```

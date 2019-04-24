# Exercism_Rust
Esai ini dibuat untuk memenuhi tugas dari mata kuliah Operasi sistem semester 110, Ilmu Komputer, Universitas Negeri Jakarta, membahas tentang cara menyelesaikan suatu problem Rust yang ada pada Exercism.io.
***

# Triangle 
problem ini meminta untuk menentukan bentuk segitiga dari data sisi yang diberikan, apakah segitiga sama kaki, segitiga sama sisi, atau segitiga sembarang.

+ Equilateral  : segitiga yang memiliki tiga sisi yang sama panjang.
+ Isosceles    : segitiga yang memiliki dua sisi yang sama panjang.
+ scalene      : segitiga yang semua panjang sisinya berbeda.

# Solusi 
Untuk menyelesaikan problem ini, kita harus mengecek panjang setiap sisi yang diberikan harus lebih besar dari 0, dan jumlah panjang dari 2 sisi harus lebih besar atau sama dengan panjang sisi yang ketiga. Setelah sudah dicek bahwa bentuk memenuhi syarat sebagai segitiga, kita harus mengecek lagi apakah itu segitiga sama sisi (Equilateral), segitiga sama kaki (Isoceles), atau segitiga sembarang (Scalene).

```rust
pub struct Triangle{
    a: u64,
    b: u64,
    c: u64,
}

impl Triangle { 
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        if sides[0] == 0|| sides[1]  == 0 || sides[2] == 0{ //setiap panjang sisi harus lebih dari 0, jika tidak return none
            None
        }
        else if sides[0] + sides[1] < sides[2] || sides[1] + sides[2] < sides[0] || sides[2] + sides[0] < sides[1]{ //jumlah panjang dari 2 sisi harus lebih besar atau sama dengan
            None                                                                                                    //panjang sisi ketiga
        }else{
            Some(Triangle{a: sides[0], b: sides[1], c: sides[2]}) //return nilai sides[0],sides[1],sides[2] ke a,b,c di dalam struct
        }    
    }
```
Fungsi pada `build` adalah untuk mengecek apakah data yang diberikan memenuhi syarat sebagai segitiga. `sides[0]` digunakan untuk mengambil value dari index ke 0 pada `sides`.

## Equilateral
```rust
pub fn is_equilateral(&self) -> bool {  //triangle has all three sides the same length.
        self.a == self.b && self.b == self.c && self.c == self.a
    }
```
`is_equilateral` berfungsi untuk menentukan bentuk segitiga tersebut adalah sama sisi, dengan syarat semua panjang sisinya sama. `self.a` disini memanggil `a` pada struct yang nilainya telah disini oleh `side[0]`

## Isosceles
```rust
pub fn is_isosceles(&self) -> bool {    //triangle has at least two sides the same length.
        self.a == self.b || self.b == self.c || self.c == self.a
    }
```
`is_isosceles` berfungsi untuk menentukan bentuk segitiga tersebut adalah sama kaki, dengan syarat harus mempunyai 2 sisi yang sama panjang.

## Scalene
```rust
pub fn is_scalene(&self) -> bool {  //triangle has all sides of different lengths.
        self.a != self.b && self.b != self.c && self.c != self.a
    }
```
`is_scalene` berfungsi untuk menentukan bentuk segitiga tersebut adalah sembarang, dengan syarat semua panjang sisinya berbeda.

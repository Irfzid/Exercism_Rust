pub struct Triangle{
    a: u64,
    b: u64,
    c: u64,
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        if sides[0] == 0|| sides[1]  == 0 || sides[2] == 0{ //all sides have to be of length > 0
            None
        }
        else if sides[0] + sides[1] < sides[2] || sides[1] + sides[2] < sides[0] || sides[2] + sides[0] < sides[1]{ //the sum of the lengths of any two sides must be greater than or equal to the
            None                                                                                                    //length of the third side
        }else{
            Some(Triangle{a: sides[0], b: sides[1], c: sides[2]}) //return nilai sides[0],sides[1],sides[2] ke dalam a,b,c di di struct
        }    
    }

    pub fn is_equilateral(&self) -> bool {  //triangle has all three sides the same length.
        self.a == self.b && self.b == self.c && self.c == self.a
    }

    pub fn is_scalene(&self) -> bool {  //triangle has at least two sides the same length.
        
        self.a != self.b && self.b != self.c && self.c != self.a
    }

    pub fn is_isosceles(&self) -> bool {    //triangle has all sides of different lengths.
        
        self.a == self.b || self.b == self.c || self.c == self.a
    }
}

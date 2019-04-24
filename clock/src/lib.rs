use std::fmt;

pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let sehari = 60 * 24;
        let all = (hours * 60) + minutes; //jumlah seluruh waktu dalam bentuk menit
        let check = ((all % sehari ) + sehari ) % sehari; //<- disini jika ada data yang minus, akan di normalkan menjadi positif, sekaligus mengecek apakah waktunya lebih dari satu hari atau tidak
        let jam = check / 60; 
        let menit = check % 60; 
        Clock{hours: jam, minutes: menit}//->men-return nilai jam dan menit ke struct hours dan minutes
    }   

    pub fn add_minutes(&self, minutes: i32) -> Self {
        //unimplemented!("Add {} minutes to existing Clock time", minutes);
        //let minute = self.minutes + minutes;

        //Clock{hours: self.hours, minutes: minute}
        Clock::new(self.hours, self.minutes + minutes)
    }

        
    
    
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        /*let mut m = self.minutes % 60;
        let mut temp = self.minutes / 60;
        let mut h = temp + self.hours;
        h = h % 24;*/
        write!(f, "{:02}:{:02}",self.hours, self.minutes)//-> {;02} https://rust-lang-nursery.github.io/rust-cookbook/datetime/parse.html

    }
}

impl fmt::Debug for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.hours, self.minutes)
    }
}

impl PartialEq for Clock {                  //-> https://doc.rust-lang.org/std/cmp/trait.PartialEq.html
    fn eq(&self, other: &Clock) -> bool {
        (self.hours, self.minutes) == (other.hours, other.minutes) 
    }
}

// Mentor Bagus Nughara
// solusi untuk menyelesaikan problem ini, jam dan waktu ditambah lalu dijumlahnya dijadikan menit semua.


//Floyd cycle delection algorithm

enum Working {
	Process,
	Stop
}

pub struct Floyd{
   func: fn(&f32) -> f32,
   xstart: f32,
}

impl Floyd {
	pub fn new(xstart:f32, func: fn(f32) -> f32) -> Floyd {
	    Floyd {
	      func:&func,
	      xstart:xstart,
	   }
	}

	/*fn start(&self) -> (i32,i32) {
	   let mut tortoise = self.func(self.xstart);
	   let mut hare = self.func(self.func(self.xstart));
	   let (tortoise, hare) = self.fit(&tortoise, &hare);
	   let mu = self.find_position(tortoise, hare, 0);
	   let lam = self.find_length(tortoise, hare, 1);
	   return (mu, lam);
	}*/

	pub fn fit(&self, tortoise: f32, hare: f32) -> (f32,f32) {
	   let v = self.xstart;
	   match tortoise ==  hare {
	      True => (tortoise, hare),
	      _ => self.fit(&self.func(tortoise), &self.func(self.func(hare))),
	   }
	}

	/*fn find_position(&self, tortoise: f32, hare: f32, mu: f32) ->(f32,f32,f32) {
	   match tortoise != hare {
	     True => (tortoise, hare, mu),
	     _ => self.find_potition(self.func(tortoise), self.func(hare), mu+1),
	   }
	}

	fn find_length(&self, tortoise: f32, hare: f32, lam: f32) -> f32 {
	   match tortoise != hare {
	     True => lam,
	     _ => self.find_length(tortoise, self.func(hare), lam+1),
	   }
	}*/
}

#[test]
fn it_works() {
   let mut value = Floyd::new(1, |x:f32| -> f32 {
       x + 0.47
   });
}

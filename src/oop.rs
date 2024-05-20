#[derive(Debug)]
pub struct MonotonicStack<T: PartialOrd + Copy> {
  data: Vec<T>
}

impl<T: PartialOrd + Copy> MonotonicStack<T> {
    pub fn new(data: &Vec<T>) -> Self {
        let mut stack = Self {data: Vec::new()};

        stack.add_list(data);

        stack
    }    

    pub fn add(&mut self, val: T) {
        let mut n = self.data.len();

        while n > 0 && self.data[n-1] < val {
            self.data.pop();
            n -= 1;
        }

        self.data.push(val);
    }

    pub fn add_list(&mut self, data: &Vec<T>) {
      for &val in data {
        self.add(val);
      }
    }

    pub fn data(&self) -> Vec<T> {
        self.data.clone()
    }
}

pub trait Car: std::fmt::Debug {
    fn info(&self) -> String;
}

#[derive(Debug)]
pub struct DomesticCar {
  pub name: String,
  pub max_speed: i32
}

impl DomesticCar {
    pub fn new(name: String, max_speed: i32) -> Self {
        Self { name, max_speed }
    }
}

impl Car for DomesticCar {
    fn info(&self) -> String {
        format!("{}, {}", self.name, self.max_speed)
    }
}

#[derive(Debug)]
pub struct ForeignCar {
  pub car: DomesticCar,
  pub country: String
}

impl ForeignCar {
    pub fn new(name: String, max_speed: i32, country: String) -> Self {
      Self { car: DomesticCar::new(name, max_speed), country }
    }
}

impl Car for ForeignCar {
    fn info(&self) -> String {
        format!("{}, {}", self.car.info(), self.country)
    }
}

pub struct CarFabric {
  	pub country: String
}

impl CarFabric {
	pub fn new(country: String) -> Self {
		Self { country }
	}

    pub fn create_car(&self, foreign: bool, name: String, max_speed: i32, country: Option<String>) -> Box<dyn Car> {
        if foreign {
			Box::new(
				ForeignCar::new(
					name, max_speed, 
					country.unwrap_or(self.country.clone())
				)
			)
        } else {
			Box::new(
				DomesticCar::new(name, max_speed)
			)
		}
    }
}

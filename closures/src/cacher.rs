use std::collections::HashMap;

pub struct Cacher<T: Fn(u32) -> u32>
{
    calculation: T,
    cache_pool: HashMap<u32, u32>,
}

impl<T: Fn(u32) -> u32> Cacher<T> {
    /// 创建 cacher
    ///
    /// # Arguments
    ///
    /// * `calculation`: 一个闭包函数，入参和出餐都是u32
    ///
    /// returns: Cacher<T>
    ///
    /// # Examples
    ///
    /// ```
    ///   let cacher = Cacher::create(|num| {
    ///          num
    ///      });
    /// ```
    pub fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            cache_pool: HashMap::new(),
        }
    }

    pub fn get(&mut self, arg: u32) -> u32 {
        match self.cache_pool.get(&arg) {
            None => {
                let v = (self.calculation)(arg);
                self.cache_pool.insert(arg, v);
                v
            }
            Some(v) => {
                *v
            }
        }
    }
}
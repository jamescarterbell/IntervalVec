mod rotations;

pub struct IntervalVec<T>
    where T: PartialEq + Clone{
    head: Option<Box<IntervalNode<T>>>,
    count: usize,
}

struct IntervalNode<T>
    where T: PartialEq + Clone{
    element: T,
    start: usize,
    count: usize,
    left: Option<Box<IntervalNode<T>>>,
    right: Option<Box<IntervalNode<T>>>,
}

#[derive(Debug)]
pub enum IntervalVecErr{
    OutOfRange{
        count: usize,
        index: usize,
    }
}

impl<T> IntervalVec<T>
    where T: PartialEq + Clone{

    /// Create an empty InternalVec.  No allocations will be made
    /// until a value is inserted.
    pub fn new() -> Self{
        Self{
            head: None,
            count: 0,
        }
    }

    /// ```
    /// # use interval_vec::*;
    /// # fn main(){
    /// let mut v = IntervalVec::new();
    /// v.push(0);
    /// v.push(1);
    /// v.push(2);
    /// assert_eq!(v.get(1).unwrap(), &1);
    /// # }
    /// ```
    /// Gets the element at the given index.
    pub fn get(&self, index: usize) -> Result<&T, IntervalVecErr>{
        let result = match &self.head{
            Some(node) => node.get(index),
            None =>  None,
        };

        match result{
            Some(element) => Ok(element),
            None => Err(IntervalVecErr::OutOfRange{
                count: self.count,
                index
            })
        }
    }

    /// Replaces the element at the given index.
    /// ```
    /// # use interval_vec::*;
    /// # fn main(){
    /// let mut v = IntervalVec::new();
    /// v.push(0);
    /// v.push(1);
    /// v.push(1);
    /// v.push(2);
    /// assert_eq!(v.get(1).unwrap(), &1);
    /// v.set(0, 3);
    /// assert_eq!(v.get(0).unwrap(), &3);
    /// # assert_eq!(v.len(), 4);
    /// # }
    /// ```
    pub fn set(&mut self, index: usize, element: T) -> Result<(), IntervalVecErr>{
        match &mut self.head{
            Some(node) => 
                node.set(index, element).map_err(|_| IntervalVecErr::OutOfRange{
                    count: self.count,
                    index
                }),
            None => Err(IntervalVecErr::OutOfRange{
                count: self.count,
                index
            })
        }
    }

    /// Inserts the element at the given idnex.
    /// ```
    /// # use interval_vec::*;
    /// # fn main(){
    /// let mut v = IntervalVec::new();
    /// v.push(0);
    /// v.push(1);
    /// v.push(2);
    /// v.insert(0, 3);
    /// assert_eq!(v.get(0).unwrap(), &3);
    /// # assert_eq!(v.len(), 4);
    /// # }
    /// ```
    pub fn insert(&mut self, index: usize, element: T) -> Result<(), IntervalVecErr>{
        let result = match &mut self.head{
            Some(node) => 
                node.insert(index, element).map_err(|_| IntervalVecErr::OutOfRange{
                    count: self.count,
                    index
                }),
            None =>{
                if index == 0{
                    self.head = Some(Box::new(IntervalNode{
                        element,
                        start: 0,
                        count: 1,
                        left: None,
                        right: None,
                    }));
                    Ok(())
                } 
                else{
                    Err(IntervalVecErr::OutOfRange{
                        count: self.count,
                        index
                    })
                }
            }
        };
        self.count += 1;
        result
    }

    /// Add an element to the end of the vector.
    /// ```
    /// # use interval_vec::*;
    /// # fn main(){
    /// let mut v = IntervalVec::new();
    /// v.push(0);
    /// v.push(1);
    /// v.push(1);
    /// v.push(2);
    /// assert_eq!(v.get(1).unwrap(), &1);
    /// assert_eq!(v.get(3).unwrap(), &2);
    /// # assert_eq!(v.len(), 4);
    /// # }
    /// ```
    pub fn push(&mut self, element: T) -> Result<(), IntervalVecErr>{
        self.insert(self.count, element)
    }

    pub fn len(&self) -> usize{
        self.count
    }
}

impl<T> IntervalNode<T>
    where T: PartialEq + Clone{
    /// Gets a reference to an element at index via tree traversal.
    fn get(&self, index: usize) -> Option<&T>{
        match index{
            i if i < self.start => match &self.left{
                Some(node) => node.get(index),
                None => None,
            },
            i if i >= self.start + self.count => match &self.right{
                Some(node) => node.get(index),
                None => None,
            },
            _ => Some(&self.element)
        }
    }

    fn set(&mut self, index: usize, element: T) -> Result<(), ()>{
        match index{
            i if i < self.start => return match &mut self.left{
                Some(node) => node.set(index, element),
                None => Err(()),
            },
            i if i >= self.start + self.count => return match &mut self.right{
                Some(node) => node.set(index, element),
                None => Err(()),
            },
            _ => {}
        };

        match element == self.element{
            true => Ok(()),
            false => {
                self.replace_node(index, element);
                Ok(())
            }
        }
    }

    fn replace_node(&mut self, index: usize, element: T){
        let new_right = match self.count - (index - self.start){
            count if count > 0 => Some(
                IntervalNode{
                    count: self.count - (index - self.start),
                    start: self.start + index,
                    element: self.element.clone(),
                    left: None,
                    right: None,
                }
            ),
            _ => None,
        };
        
        let right_count = match &new_right{
            Some(node) => node.count,
            None => 0,
        };

        let new_left = match self.count - (right_count){
            count if count > 0 => Some(
                IntervalNode{
                    count,
                    start: self.start,
                    element: self.element.clone(),
                    left: None,
                    right: None,
                }
            ),
            _ => None,
        };
        
        self.count = 1;
        self.element = element;
        
        if let Some(mut new_left) = new_left{
            std::mem::swap( &mut self.left, &mut new_left.left);
            self.left = Some(Box::new(new_left));
        }

        if let Some(mut new_right) = new_right{
            std::mem::swap( &mut self.right, &mut new_right.right);
            self.right = Some(Box::new(new_right));
        }

        if let Some(node) = &self.left{
            self.start = node.count + node.start;
        }
    }

    fn insert(&mut self, index: usize, element: T) -> Result<(), ()>{
        match index{
            i if i < self.start => return match &mut self.left{
                Some(node) => node.insert(index, element),
                None => Err(()),
            },
            i if index >= self.start + self.count => return match &mut self.right{
                Some(node) => node.insert(index, element),
                None => {
                    if i == self.start + self.count{
                        self.right = Some(Box::new(IntervalNode{
                            element,
                            start: self.start + self.count,
                            count: 1,
                            left: None,
                            right: None,
                        }));
                        Ok(())
                    }
                    else{
                        Err(())
                    }
                },
            },
            _ => {
                if element != self.element{
                    self.replace_node(index, element);
                }
                else{
                    self.count += 1;
                }
        
                match &mut self.right{
                    Some(node) => node.increase_start(),
                    None => {},
                };
                Ok(())
            }
        }
    }

    fn increase_start(&mut self){
        self.start += 1;
        match &mut self.left{
            Some(node) => node.increase_start(),
            None => {}
        };
        match &mut self.right{
            Some(node) => node.increase_start(),
            None => {}
        };
    }
}
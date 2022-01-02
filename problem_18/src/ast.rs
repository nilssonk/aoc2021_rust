#[derive(Copy, Clone, PartialEq)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    pub fn invert(&self) -> Self {
        use Direction::*;
        match self {
            Left => Right,
            Right => Left,
        }
    }
}

#[derive(Clone)]
pub struct Node {
    left: Box<SnailNumber>,
    right: Box<SnailNumber>,
}

#[derive(Clone)]
pub enum SnailNumberType {
    Leaf(usize),
    Node(Node),
}

pub struct SnailNumber {
    parent: *mut SnailNumber,
    direction: Option<Direction>,
    data: SnailNumberType,
}

impl Clone for Box<SnailNumber> {
    fn clone(&self) -> Self {
        let mut new = Box::new(SnailNumber {
            parent: std::ptr::null_mut(),
            direction: self.direction,
            data: self.data.clone(),
        });

        let new_ptr: *mut SnailNumber = &mut *new;
        if let SnailNumberType::Node(ref mut n) = new.data {
            n.left.parent = new_ptr;
            n.right.parent = new_ptr;
        }

        new
    }
}

impl std::ops::Add for Box<SnailNumber> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let mut result = SnailNumber::new_node(self, other);
        result.reduce();
        result
    }
}

impl SnailNumber {
    pub fn magnitude(&self) -> usize {
        use SnailNumberType::*;
        match self.data {
            Node(ref n) => 3 * n.left.magnitude() + 2 * n.right.magnitude(),
            Leaf(ref x) => *x,
        }
    }

    pub fn new_node(left: Box<Self>, right: Box<Self>) -> Box<Self> {
        use SnailNumberType::*;

        let mut new = Box::new(Self {
            parent: std::ptr::null_mut(),
            direction: None,
            data: Node(crate::ast::Node { left, right }),
        });

        let new_ptr: *mut SnailNumber = &mut *new;
        match new.data {
            Node(ref mut n) => {
                n.left.update_metadata(new_ptr, Some(Direction::Left));
                n.right.update_metadata(new_ptr, Some(Direction::Right));
            }
            _ => unreachable!(),
        }

        new
    }

    pub fn new_leaf(value: usize) -> Box<Self> {
        Box::new(Self {
            parent: std::ptr::null_mut(),
            direction: None,
            data: SnailNumberType::Leaf(value),
        })
    }

    fn new_leaf_with_metadata(value: usize, parent: *mut Self, direction: Direction) -> Box<Self> {
        Box::new(Self {
            parent,
            direction: Some(direction),
            data: SnailNumberType::Leaf(value),
        })
    }

    fn update_metadata(&mut self, parent: *mut Self, direction: Option<Direction>) {
        self.parent = parent;
        self.direction = direction;
    }

    fn rightmost(&mut self) -> &mut Self {
        use SnailNumberType::*;
        match self.data {
            Node(ref mut n) => n.right.rightmost(),
            _ => self,
        }
    }

    fn leftmost(&mut self) -> &mut Self {
        use SnailNumberType::*;
        match self.data {
            Node(ref mut n) => n.left.leftmost(),
            _ => self,
        }
    }

    fn scan(&mut self, direction: Direction) -> Option<&mut Self> {
        use SnailNumberType::*;
        let result_direction = direction.invert();
        let follow_up: Box<dyn Fn(&mut crate::ast::Node) -> &mut Self> = match result_direction {
            Direction::Left => Box::new(|x| x.right.leftmost()),
            Direction::Right => Box::new(|x| x.left.rightmost()),
        };

        let mut current: *mut Self = &mut *self;
        let mut deref = unsafe { &mut *current };
        while let Some(dir) = deref.direction {
            if dir == result_direction {
                let parent = unsafe { &mut *deref.parent };
                match parent.data {
                    Node(ref mut n) => {
                        let result = follow_up(n);
                        return Some(result);
                    }
                    _ => unreachable!(),
                }
            }

            current = deref.parent;
            deref = unsafe { &mut *current };
        }

        None
    }

    fn do_explode(&mut self, to_add: usize, direction: Direction) {
        use SnailNumberType::*;
        if to_add > 0 {
            if let Some(x) = self.scan(direction) {
                match x.data {
                    Leaf(ref mut v) => *v += to_add,
                    _ => unreachable!(),
                }
            }
        }
    }

    fn explode(&mut self, level: u8) -> bool {
        use SnailNumberType::*;
        if let Node(ref mut n) = self.data {
            let l = &mut n.left;
            let r = &mut n.right;
            return match (&mut l.data, &mut r.data) {
                (Leaf(t_a), Leaf(t_b)) => {
                    let a = *t_a;
                    let b = *t_b;
                    if level >= 4 {
                        self.data = Leaf(0);
                        self.do_explode(a, Direction::Left);
                        self.do_explode(b, Direction::Right);
                        true
                    } else {
                        false
                    }
                }
                _ => l.explode(level + 1) || r.explode(level + 1),
            };
        }

        false
    }

    fn split(&mut self) -> bool {
        use SnailNumberType::*;
        match self.data {
            Node(ref mut n) => n.left.split() || n.right.split(),
            Leaf(x) => {
                x >= 10 && {
                    let new_left = Self::new_leaf_with_metadata(x / 2, &mut *self, Direction::Left);
                    let new_right =
                        Self::new_leaf_with_metadata((x + 1) / 2, &mut *self, Direction::Right);
                    self.data = Node(crate::ast::Node {
                        left: new_left,
                        right: new_right,
                    });
                    true
                }
            }
        }
    }

    fn reduce(&mut self) {
        let mut should_continue = true;
        while should_continue {
            should_continue = self.explode(0) || self.split();
        }
    }
}

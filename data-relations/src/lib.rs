


pub struct OneToOne<A, B> {
    pub from: A,
    pub to: B
}

impl<A, B> OneToOne<A, B> {
    pub fn new(from: A, to: B) -> Self {
        Self { from, to }
    }
}

pub struct OneToMany<A, B> {
    pub from: A,
    pub to: Vec<B>
}

impl<A, B> OneToMany<A, B> {
    pub fn new(from: A, to: Vec<B>) -> Self {
        Self { from, to }
    }
}

pub struct ManyToOne<A, B> {
    pub from: Vec<A>,
    pub to: B
}

impl<A, B> ManyToOne<A, B> {
    pub fn new(from: Vec<A>, to: B) -> Self {
        Self { from, to }
    }
}

pub struct ManyToMany<A, B> {
    pub from: Vec<A>,
    pub to: Vec<B>
}

impl<A, B> ManyToMany<A, B> {
    pub fn new(from: Vec<A>, to: Vec<B>) -> Self {
        Self { from, to }
    }
}
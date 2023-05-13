


pub struct OneToOne<'me, A, B> {
    pub from: &'me A,
    pub to: &'me B
}

impl<'me, A, B> OneToOne<'me, A, B> {
    pub fn new(from: &'me A, to: &'me B) -> Self {
        Self { from, to }
    }
}

pub struct OneToMany<'me, A, B> {
    pub from: &'me A,
    pub to: Vec<&'me B>
}

impl<'me, A, B> OneToMany<'me, A, B> {
    pub fn new(from: &'me A, to: Vec<&'me B>) -> Self {
        Self { from, to }
    }
}

pub struct ManyToOne<'me, A, B> {
    pub from: Vec<&'me A>,
    pub to: &'me B
}

impl<'me, A, B> ManyToOne<'me, A, B> {
    pub fn new(from: Vec<&'me A>, to: &'me B) -> Self {
        Self { from, to }
    }
}

pub struct ManyToMany<'me, A, B> {
    pub from: Vec<&'me A>,
    pub to: Vec<&'me B>
}

impl<'me, A, B> ManyToMany<'me, A, B> {
    pub fn new(from: Vec<&'me A>, to: Vec<&'me B>) -> Self {
        Self { from, to }
    }
}
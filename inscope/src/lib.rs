

pub enum Scope<T> {
    Present(T),
    Absent
}

impl<T> Scope<T> {
    pub fn new(t: T) -> Scope<T> {
        Scope::Present(t)
    }
}

/****************************************
* General impls
****************************************/
impl<T> std::fmt::Debug for Scope<T>
where
    T: std::fmt::Debug
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Scope::Present(t) => write!(f, "{:?}", t),
            Scope::Absent => write!(f, "?!")
        }
    }
}

impl<T> std::fmt::Display for Scope<T>
where
    T: std::fmt::Display
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Scope::Present(t) => write!(f, "{}", t),
            Scope::Absent => write!(f, "?!")
        }
    }
}
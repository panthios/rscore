use crate::Mc;


impl<T> std::fmt::Debug for Mc<T>
where
    T: std::fmt::Debug
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        unsafe { &*self.0 }.fmt(f)
    }
}

impl<T> std::fmt::Display for Mc<T>
where
    T: std::fmt::Display
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        unsafe { &*self.0 }.fmt(f)
    }
}

impl<T> Clone for Mc<T>
where
    T: Clone
{
    fn clone(&self) -> Self {
        Mc::new(unsafe { &*self.0 }.clone())
    }
}

impl<T> std::ops::Deref for Mc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}

impl<T> std::ops::DerefMut for Mc<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.0 }
    }
}

impl<T, R> std::ops::Add<R> for Mc<T>
where
    T: std::ops::Add<R> + Clone
{
    type Output = T::Output;

    fn add(self, rhs: R) -> Self::Output {
        unsafe { &*self.0 }.clone() + rhs
    }
}

impl<T, R> std::ops::AddAssign<R> for Mc<T>
where
    T: std::ops::AddAssign<R>
{
    fn add_assign(&mut self, rhs: R) {
        unsafe { &mut *self.0 }.add_assign(rhs);
    }
}

impl<T, R> std::ops::Sub<R> for Mc<T>
where
    T: std::ops::Sub<R> + Clone
{
    type Output = T::Output;

    fn sub(self, rhs: R) -> Self::Output {
        unsafe { &*self.0 }.clone() - rhs
    }
}

impl<T, R> std::ops::SubAssign<R> for Mc<T>
where
    T: std::ops::SubAssign<R>
{
    fn sub_assign(&mut self, rhs: R) {
        unsafe { &mut *self.0 }.sub_assign(rhs);
    }
}

impl<T, R> std::ops::Mul<R> for Mc<T>
where
    T: std::ops::Mul<R> + Clone
{
    type Output = T::Output;

    fn mul(self, rhs: R) -> Self::Output {
        unsafe { &*self.0 }.clone() * rhs
    }
}

impl<T, R> std::ops::MulAssign<R> for Mc<T>
where
    T: std::ops::MulAssign<R>
{
    fn mul_assign(&mut self, rhs: R) {
        unsafe { &mut *self.0 }.mul_assign(rhs);
    }
}

impl<T, R> std::ops::Div<R> for Mc<T>
where
    T: std::ops::Div<R> + Clone
{
    type Output = T::Output;

    fn div(self, rhs: R) -> Self::Output {
        unsafe { &*self.0 }.clone() / rhs
    }
}

impl<T, R> std::ops::DivAssign<R> for Mc<T>
where
    T: std::ops::DivAssign<R>
{
    fn div_assign(&mut self, rhs: R) {
        unsafe { &mut *self.0 }.div_assign(rhs);
    }
}

impl<T, R> std::ops::Rem<R> for Mc<T>
where
    T: std::ops::Rem<R> + Clone
{
    type Output = T::Output;

    fn rem(self, rhs: R) -> Self::Output {
        unsafe { &*self.0 }.clone() % rhs
    }
}

impl<T, R> std::ops::RemAssign<R> for Mc<T>
where
    T: std::ops::RemAssign<R>
{
    fn rem_assign(&mut self, rhs: R) {
        unsafe { &mut *self.0 }.rem_assign(rhs);
    }
}

impl<T> std::ops::Neg for Mc<T>
where
    T: std::ops::Neg + Clone
{
    type Output = T::Output;

    fn neg(self) -> Self::Output {
        -unsafe { &*self.0 }.clone()
    }
}

impl<T> std::ops::Not for Mc<T>
where
    T: std::ops::Not + Clone
{
    type Output = T::Output;

    fn not(self) -> Self::Output {
        !unsafe { &*self.0 }.clone()
    }
}

impl<T, R> std::ops::BitAnd<R> for Mc<T>
where
    T: std::ops::BitAnd<R> + Clone
{
    type Output = T::Output;

    fn bitand(self, rhs: R) -> Self::Output {
        unsafe { &*self.0 }.clone() & rhs
    }
}

impl<T, R> std::ops::BitAndAssign<R> for Mc<T>
where
    T: std::ops::BitAndAssign<R>
{
    fn bitand_assign(&mut self, rhs: R) {
        unsafe { &mut *self.0 }.bitand_assign(rhs);
    }
}

impl<T, R> std::ops::BitOr<R> for Mc<T>
where
    T: std::ops::BitOr<R> + Clone
{
    type Output = T::Output;

    fn bitor(self, rhs: R) -> Self::Output {
        unsafe { &*self.0 }.clone() | rhs
    }
}

impl<T, R> std::ops::BitOrAssign<R> for Mc<T>
where
    T: std::ops::BitOrAssign<R>
{
    fn bitor_assign(&mut self, rhs: R) {
        unsafe { &mut *self.0 }.bitor_assign(rhs);
    }
}

impl<T, R> std::ops::BitXor<R> for Mc<T>
where
    T: std::ops::BitXor<R> + Clone
{
    type Output = T::Output;

    fn bitxor(self, rhs: R) -> Self::Output {
        unsafe { &*self.0 }.clone() ^ rhs
    }
}

impl<T, R> std::ops::BitXorAssign<R> for Mc<T>
where
    T: std::ops::BitXorAssign<R>
{
    fn bitxor_assign(&mut self, rhs: R) {
        unsafe { &mut *self.0 }.bitxor_assign(rhs);
    }
}

impl<T> std::ops::Shl<usize> for Mc<T>
where
    T: std::ops::Shl<usize> + Clone
{
    type Output = T::Output;

    fn shl(self, rhs: usize) -> Self::Output {
        unsafe { &*self.0 }.clone() << rhs
    }
}

impl<T> std::ops::ShlAssign<usize> for Mc<T>
where
    T: std::ops::ShlAssign<usize>
{
    fn shl_assign(&mut self, rhs: usize) {
        unsafe { &mut *self.0 }.shl_assign(rhs);
    }
}

impl<T> std::ops::Shr<usize> for Mc<T>
where
    T: std::ops::Shr<usize> + Clone
{
    type Output = T::Output;

    fn shr(self, rhs: usize) -> Self::Output {
        unsafe { &*self.0 }.clone() >> rhs
    }
}

impl<T> std::ops::ShrAssign<usize> for Mc<T>
where
    T: std::ops::ShrAssign<usize>
{
    fn shr_assign(&mut self, rhs: usize) {
        unsafe { &mut *self.0 }.shr_assign(rhs);
    }
}

impl<T, R> PartialEq<R> for Mc<T>
where
    T: PartialEq<R> + Clone
{
    fn eq(&self, other: &R) -> bool {
        unsafe { &*self.0 }.clone() == *other
    }
}

impl<T, R> PartialOrd<R> for Mc<T>
where
    T: PartialOrd<R> + Clone
{
    fn partial_cmp(&self, other: &R) -> Option<std::cmp::Ordering> {
        unsafe { &*self.0 }.clone().partial_cmp(other)
    }
}

impl<T> Eq for Mc<T>
where
    T: Eq + Clone + PartialEq<Mc<T>>
{}

impl<T> Ord for Mc<T>
where
    T: Ord + Clone + PartialOrd<Mc<T>>
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { &*self.0 }.clone().cmp(unsafe { &*other.0 })
    }
}

impl<T> std::hash::Hash for Mc<T>
where
    T: std::hash::Hash + Clone
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        unsafe { &*self.0 }.clone().hash(state);
    }
}
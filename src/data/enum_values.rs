use std::slice::Iter;

pub trait EnumValues
where
	Self: Sized
{
	fn values() -> Iter<'static, Self>;
}

use enum_extra_derive::NonZeroRepr;
use enum_extra::NonZeroRepr;
use strum_macros::EnumMetadata;
use strum::EnumMetadata;

#[repr(C)]
union Error {
  non_error: NonError,
  concrete_error: ConcreteError,
  error: isize
}

#[derive(Copy, Clone)]
#[repr(isize)]
enum NonError {
  Ok = -1,
}


#[derive(Copy, Clone)]
#[derive(EnumMetadata, NonZeroRepr)]
#[repr(isize)]
enum ConcreteError {
  Foo = NonError::Ok as isize + 1,
  Bar,
}

fn main() {

}

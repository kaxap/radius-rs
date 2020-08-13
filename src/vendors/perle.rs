/// Definitions for vendor Perle, vendor value 1966
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PerleClusteredPortAccess(pub u32);
 
#[allow(non_upper_case_globals)]
impl PerleClusteredPortAccess {
	pub const Disabled: PerleClusteredPortAccess = PerleClusteredPortAccess(0);
	pub const Enabled: PerleClusteredPortAccess = PerleClusteredPortAccess(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PerleUserLevel(pub u32);
 
#[allow(non_upper_case_globals)]
impl PerleUserLevel {
	pub const Admin: PerleUserLevel = PerleUserLevel(1);
	pub const Normal: PerleUserLevel = PerleUserLevel(2);
	pub const Restricted: PerleUserLevel = PerleUserLevel(3);
	pub const Menu: PerleUserLevel = PerleUserLevel(4);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PerleLineAccessPort1(pub u32);
 
#[allow(non_upper_case_globals)]
impl PerleLineAccessPort1 {
	pub const Disabled: PerleLineAccessPort1 = PerleLineAccessPort1(0);
	pub const ReadWrite: PerleLineAccessPort1 = PerleLineAccessPort1(1);
	pub const ReadInput: PerleLineAccessPort1 = PerleLineAccessPort1(2);
	pub const ReadInputWrite: PerleLineAccessPort1 = PerleLineAccessPort1(3);
	pub const ReadOutput: PerleLineAccessPort1 = PerleLineAccessPort1(4);
	pub const ReadOutputWrite: PerleLineAccessPort1 = PerleLineAccessPort1(5);
	pub const ReadOutputInput: PerleLineAccessPort1 = PerleLineAccessPort1(6);
	pub const ReadOutputInputWrite: PerleLineAccessPort1 = PerleLineAccessPort1(7);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PerleLineAccessPort2(pub u32);
 
#[allow(non_upper_case_globals)]
impl PerleLineAccessPort2 {
	pub const Disabled: PerleLineAccessPort2 = PerleLineAccessPort2(0);
	pub const ReadWrite: PerleLineAccessPort2 = PerleLineAccessPort2(1);
	pub const ReadInput: PerleLineAccessPort2 = PerleLineAccessPort2(2);
	pub const ReadInputWrite: PerleLineAccessPort2 = PerleLineAccessPort2(3);
	pub const ReadOutput: PerleLineAccessPort2 = PerleLineAccessPort2(4);
	pub const ReadOutputWrite: PerleLineAccessPort2 = PerleLineAccessPort2(5);
	pub const ReadOutputInput: PerleLineAccessPort2 = PerleLineAccessPort2(6);
	pub const ReadOutputInputWrite: PerleLineAccessPort2 = PerleLineAccessPort2(7);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PerleLineAccessPort3(pub u32);
 
#[allow(non_upper_case_globals)]
impl PerleLineAccessPort3 {
	pub const Disabled: PerleLineAccessPort3 = PerleLineAccessPort3(0);
	pub const ReadWrite: PerleLineAccessPort3 = PerleLineAccessPort3(1);
	pub const ReadInput: PerleLineAccessPort3 = PerleLineAccessPort3(2);
	pub const ReadInputWrite: PerleLineAccessPort3 = PerleLineAccessPort3(3);
	pub const ReadOutput: PerleLineAccessPort3 = PerleLineAccessPort3(4);
	pub const ReadOutputWrite: PerleLineAccessPort3 = PerleLineAccessPort3(5);
	pub const ReadOutputInput: PerleLineAccessPort3 = PerleLineAccessPort3(6);
	pub const ReadOutputInputWrite: PerleLineAccessPort3 = PerleLineAccessPort3(7);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PerleLineAccessPort4(pub u32);
 
#[allow(non_upper_case_globals)]
impl PerleLineAccessPort4 {
	pub const Disabled: PerleLineAccessPort4 = PerleLineAccessPort4(0);
	pub const ReadWrite: PerleLineAccessPort4 = PerleLineAccessPort4(1);
	pub const ReadInput: PerleLineAccessPort4 = PerleLineAccessPort4(2);
	pub const ReadInputWrite: PerleLineAccessPort4 = PerleLineAccessPort4(3);
	pub const ReadOutput: PerleLineAccessPort4 = PerleLineAccessPort4(4);
	pub const ReadOutputWrite: PerleLineAccessPort4 = PerleLineAccessPort4(5);
	pub const ReadOutputInput: PerleLineAccessPort4 = PerleLineAccessPort4(6);
	pub const ReadOutputInputWrite: PerleLineAccessPort4 = PerleLineAccessPort4(7);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PerleLineAccessPort5(pub u32);
 
#[allow(non_upper_case_globals)]
impl PerleLineAccessPort5 {
	pub const Disabled: PerleLineAccessPort5 = PerleLineAccessPort5(0);
	pub const ReadWrite: PerleLineAccessPort5 = PerleLineAccessPort5(1);
	pub const ReadInput: PerleLineAccessPort5 = PerleLineAccessPort5(2);
	pub const ReadInputWrite: PerleLineAccessPort5 = PerleLineAccessPort5(3);
	pub const ReadOutput: PerleLineAccessPort5 = PerleLineAccessPort5(4);
	pub const ReadOutputWrite: PerleLineAccessPort5 = PerleLineAccessPort5(5);
	pub const ReadOutputInput: PerleLineAccessPort5 = PerleLineAccessPort5(6);
	pub const ReadOutputInputWrite: PerleLineAccessPort5 = PerleLineAccessPort5(7);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PerleLineAccessPort6(pub u32);
 
#[allow(non_upper_case_globals)]
impl PerleLineAccessPort6 {
	pub const Disabled: PerleLineAccessPort6 = PerleLineAccessPort6(0);
	pub const ReadWrite: PerleLineAccessPort6 = PerleLineAccessPort6(1);
	pub const ReadInput: PerleLineAccessPort6 = PerleLineAccessPort6(2);
	pub const ReadInputWrite: PerleLineAccessPort6 = PerleLineAccessPort6(3);
	pub const ReadOutput: PerleLineAccessPort6 = PerleLineAccessPort6(4);
	pub const ReadOutputWrite: PerleLineAccessPort6 = PerleLineAccessPort6(5);
	pub const ReadOutputInput: PerleLineAccessPort6 = PerleLineAccessPort6(6);
	pub const ReadOutputInputWrite: PerleLineAccessPort6 = PerleLineAccessPort6(7);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PerleLineAccessPort7(pub u32);
 
#[allow(non_upper_case_globals)]
impl PerleLineAccessPort7 {
	pub const Disabled: PerleLineAccessPort7 = PerleLineAccessPort7(0);
	pub const ReadWrite: PerleLineAccessPort7 = PerleLineAccessPort7(1);
	pub const ReadInput: PerleLineAccessPort7 = PerleLineAccessPort7(2);
	pub const ReadInputWrite: PerleLineAccessPort7 = PerleLineAccessPort7(3);
	pub const ReadOutput: PerleLineAccessPort7 = PerleLineAccessPort7(4);
	pub const ReadOutputWrite: PerleLineAccessPort7 = PerleLineAccessPort7(5);
	pub const ReadOutputInput: PerleLineAccessPort7 = PerleLineAccessPort7(6);
	pub const ReadOutputInputWrite: PerleLineAccessPort7 = PerleLineAccessPort7(7);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PerleLineAccessPort8(pub u32);
 
#[allow(non_upper_case_globals)]
impl PerleLineAccessPort8 {
	pub const Disabled: PerleLineAccessPort8 = PerleLineAccessPort8(0);
	pub const ReadWrite: PerleLineAccessPort8 = PerleLineAccessPort8(1);
	pub const ReadInput: PerleLineAccessPort8 = PerleLineAccessPort8(2);
	pub const ReadInputWrite: PerleLineAccessPort8 = PerleLineAccessPort8(3);
	pub const ReadOutput: PerleLineAccessPort8 = PerleLineAccessPort8(4);
	pub const ReadOutputWrite: PerleLineAccessPort8 = PerleLineAccessPort8(5);
	pub const ReadOutputInput: PerleLineAccessPort8 = PerleLineAccessPort8(6);
	pub const ReadOutputInputWrite: PerleLineAccessPort8 = PerleLineAccessPort8(7);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PerleLineAccessPort9(pub u32);
 
#[allow(non_upper_case_globals)]
impl PerleLineAccessPort9 {
	pub const Disabled: PerleLineAccessPort9 = PerleLineAccessPort9(0);
	pub const ReadWrite: PerleLineAccessPort9 = PerleLineAccessPort9(1);
	pub const ReadInput: PerleLineAccessPort9 = PerleLineAccessPort9(2);
	pub const ReadInputWrite: PerleLineAccessPort9 = PerleLineAccessPort9(3);
	pub const ReadOutput: PerleLineAccessPort9 = PerleLineAccessPort9(4);
	pub const ReadOutputWrite: PerleLineAccessPort9 = PerleLineAccessPort9(5);
	pub const ReadOutputInput: PerleLineAccessPort9 = PerleLineAccessPort9(6);
	pub const ReadOutputInputWrite: PerleLineAccessPort9 = PerleLineAccessPort9(7);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PerleLineAccessPort10(pub u32);
 
#[allow(non_upper_case_globals)]
impl PerleLineAccessPort10 {
	pub const Disabled: PerleLineAccessPort10 = PerleLineAccessPort10(0);
	pub const ReadWrite: PerleLineAccessPort10 = PerleLineAccessPort10(1);
	pub const ReadInput: PerleLineAccessPort10 = PerleLineAccessPort10(2);
	pub const ReadInputWrite: PerleLineAccessPort10 = PerleLineAccessPort10(3);
	pub const ReadOutput: PerleLineAccessPort10 = PerleLineAccessPort10(4);
	pub const ReadOutputWrite: PerleLineAccessPort10 = PerleLineAccessPort10(5);
	pub const ReadOutputInput: PerleLineAccessPort10 = PerleLineAccessPort10(6);
	pub const ReadOutputInputWrite: PerleLineAccessPort10 = PerleLineAccessPort10(7);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PerleLineAccessPort11(pub u32);
 
#[allow(non_upper_case_globals)]
impl PerleLineAccessPort11 {
	pub const Disabled: PerleLineAccessPort11 = PerleLineAccessPort11(0);
	pub const ReadWrite: PerleLineAccessPort11 = PerleLineAccessPort11(1);
	pub const ReadInput: PerleLineAccessPort11 = PerleLineAccessPort11(2);
	pub const ReadInputWrite: PerleLineAccessPort11 = PerleLineAccessPort11(3);
	pub const ReadOutput: PerleLineAccessPort11 = PerleLineAccessPort11(4);
	pub const ReadOutputWrite: PerleLineAccessPort11 = PerleLineAccessPort11(5);
	pub const ReadOutputInput: PerleLineAccessPort11 = PerleLineAccessPort11(6);
	pub const ReadOutputInputWrite: PerleLineAccessPort11 = PerleLineAccessPort11(7);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PerleLineAccessPort12(pub u32);
 
#[allow(non_upper_case_globals)]
impl PerleLineAccessPort12 {
	pub const Disabled: PerleLineAccessPort12 = PerleLineAccessPort12(0);
	pub const ReadWrite: PerleLineAccessPort12 = PerleLineAccessPort12(1);
	pub const ReadInput: PerleLineAccessPort12 = PerleLineAccessPort12(2);
	pub const ReadInputWrite: PerleLineAccessPort12 = PerleLineAccessPort12(3);
	pub const ReadOutput: PerleLineAccessPort12 = PerleLineAccessPort12(4);
	pub const ReadOutputWrite: PerleLineAccessPort12 = PerleLineAccessPort12(5);
	pub const ReadOutputInput: PerleLineAccessPort12 = PerleLineAccessPort12(6);
	pub const ReadOutputInputWrite: PerleLineAccessPort12 = PerleLineAccessPort12(7);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PerleLineAccessPort13(pub u32);
 
#[allow(non_upper_case_globals)]
impl PerleLineAccessPort13 {
	pub const Disabled: PerleLineAccessPort13 = PerleLineAccessPort13(0);
	pub const ReadWrite: PerleLineAccessPort13 = PerleLineAccessPort13(1);
	pub const ReadInput: PerleLineAccessPort13 = PerleLineAccessPort13(2);
	pub const ReadInputWrite: PerleLineAccessPort13 = PerleLineAccessPort13(3);
	pub const ReadOutput: PerleLineAccessPort13 = PerleLineAccessPort13(4);
	pub const ReadOutputWrite: PerleLineAccessPort13 = PerleLineAccessPort13(5);
	pub const ReadOutputInput: PerleLineAccessPort13 = PerleLineAccessPort13(6);
	pub const ReadOutputInputWrite: PerleLineAccessPort13 = PerleLineAccessPort13(7);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PerleLineAccessPort14(pub u32);
 
#[allow(non_upper_case_globals)]
impl PerleLineAccessPort14 {
	pub const Disabled: PerleLineAccessPort14 = PerleLineAccessPort14(0);
	pub const ReadWrite: PerleLineAccessPort14 = PerleLineAccessPort14(1);
	pub const ReadInput: PerleLineAccessPort14 = PerleLineAccessPort14(2);
	pub const ReadInputWrite: PerleLineAccessPort14 = PerleLineAccessPort14(3);
	pub const ReadOutput: PerleLineAccessPort14 = PerleLineAccessPort14(4);
	pub const ReadOutputWrite: PerleLineAccessPort14 = PerleLineAccessPort14(5);
	pub const ReadOutputInput: PerleLineAccessPort14 = PerleLineAccessPort14(6);
	pub const ReadOutputInputWrite: PerleLineAccessPort14 = PerleLineAccessPort14(7);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PerleLineAccessPort15(pub u32);
 
#[allow(non_upper_case_globals)]
impl PerleLineAccessPort15 {
	pub const Disabled: PerleLineAccessPort15 = PerleLineAccessPort15(0);
	pub const ReadWrite: PerleLineAccessPort15 = PerleLineAccessPort15(1);
	pub const ReadInput: PerleLineAccessPort15 = PerleLineAccessPort15(2);
	pub const ReadInputWrite: PerleLineAccessPort15 = PerleLineAccessPort15(3);
	pub const ReadOutput: PerleLineAccessPort15 = PerleLineAccessPort15(4);
	pub const ReadOutputWrite: PerleLineAccessPort15 = PerleLineAccessPort15(5);
	pub const ReadOutputInput: PerleLineAccessPort15 = PerleLineAccessPort15(6);
	pub const ReadOutputInputWrite: PerleLineAccessPort15 = PerleLineAccessPort15(7);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PerleLineAccessPort16(pub u32);
 
#[allow(non_upper_case_globals)]
impl PerleLineAccessPort16 {
	pub const Disabled: PerleLineAccessPort16 = PerleLineAccessPort16(0);
	pub const ReadWrite: PerleLineAccessPort16 = PerleLineAccessPort16(1);
	pub const ReadInput: PerleLineAccessPort16 = PerleLineAccessPort16(2);
	pub const ReadInputWrite: PerleLineAccessPort16 = PerleLineAccessPort16(3);
	pub const ReadOutput: PerleLineAccessPort16 = PerleLineAccessPort16(4);
	pub const ReadOutputWrite: PerleLineAccessPort16 = PerleLineAccessPort16(5);
	pub const ReadOutputInput: PerleLineAccessPort16 = PerleLineAccessPort16(6);
	pub const ReadOutputInputWrite: PerleLineAccessPort16 = PerleLineAccessPort16(7);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PerleLineAccessPort17(pub u32);
 
#[allow(non_upper_case_globals)]
impl PerleLineAccessPort17 {
	pub const Disabled: PerleLineAccessPort17 = PerleLineAccessPort17(0);
	pub const ReadWrite: PerleLineAccessPort17 = PerleLineAccessPort17(1);
	pub const ReadInput: PerleLineAccessPort17 = PerleLineAccessPort17(2);
	pub const ReadInputWrite: PerleLineAccessPort17 = PerleLineAccessPort17(3);
	pub const ReadOutput: PerleLineAccessPort17 = PerleLineAccessPort17(4);
	pub const ReadOutputWrite: PerleLineAccessPort17 = PerleLineAccessPort17(5);
	pub const ReadOutputInput: PerleLineAccessPort17 = PerleLineAccessPort17(6);
	pub const ReadOutputInputWrite: PerleLineAccessPort17 = PerleLineAccessPort17(7);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PerleLineAccessPort18(pub u32);
 
#[allow(non_upper_case_globals)]
impl PerleLineAccessPort18 {
	pub const Disabled: PerleLineAccessPort18 = PerleLineAccessPort18(0);
	pub const ReadWrite: PerleLineAccessPort18 = PerleLineAccessPort18(1);
	pub const ReadInput: PerleLineAccessPort18 = PerleLineAccessPort18(2);
	pub const ReadInputWrite: PerleLineAccessPort18 = PerleLineAccessPort18(3);
	pub const ReadOutput: PerleLineAccessPort18 = PerleLineAccessPort18(4);
	pub const ReadOutputWrite: PerleLineAccessPort18 = PerleLineAccessPort18(5);
	pub const ReadOutputInput: PerleLineAccessPort18 = PerleLineAccessPort18(6);
	pub const ReadOutputInputWrite: PerleLineAccessPort18 = PerleLineAccessPort18(7);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PerleLineAccessPort19(pub u32);
 
#[allow(non_upper_case_globals)]
impl PerleLineAccessPort19 {
	pub const Disabled: PerleLineAccessPort19 = PerleLineAccessPort19(0);
	pub const ReadWrite: PerleLineAccessPort19 = PerleLineAccessPort19(1);
	pub const ReadInput: PerleLineAccessPort19 = PerleLineAccessPort19(2);
	pub const ReadInputWrite: PerleLineAccessPort19 = PerleLineAccessPort19(3);
	pub const ReadOutput: PerleLineAccessPort19 = PerleLineAccessPort19(4);
	pub const ReadOutputWrite: PerleLineAccessPort19 = PerleLineAccessPort19(5);
	pub const ReadOutputInput: PerleLineAccessPort19 = PerleLineAccessPort19(6);
	pub const ReadOutputInputWrite: PerleLineAccessPort19 = PerleLineAccessPort19(7);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PerleLineAccessPort20(pub u32);
 
#[allow(non_upper_case_globals)]
impl PerleLineAccessPort20 {
	pub const Disabled: PerleLineAccessPort20 = PerleLineAccessPort20(0);
	pub const ReadWrite: PerleLineAccessPort20 = PerleLineAccessPort20(1);
	pub const ReadInput: PerleLineAccessPort20 = PerleLineAccessPort20(2);
	pub const ReadInputWrite: PerleLineAccessPort20 = PerleLineAccessPort20(3);
	pub const ReadOutput: PerleLineAccessPort20 = PerleLineAccessPort20(4);
	pub const ReadOutputWrite: PerleLineAccessPort20 = PerleLineAccessPort20(5);
	pub const ReadOutputInput: PerleLineAccessPort20 = PerleLineAccessPort20(6);
	pub const ReadOutputInputWrite: PerleLineAccessPort20 = PerleLineAccessPort20(7);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PerleLineAccessPort21(pub u32);
 
#[allow(non_upper_case_globals)]
impl PerleLineAccessPort21 {
	pub const Disabled: PerleLineAccessPort21 = PerleLineAccessPort21(0);
	pub const ReadWrite: PerleLineAccessPort21 = PerleLineAccessPort21(1);
	pub const ReadInput: PerleLineAccessPort21 = PerleLineAccessPort21(2);
	pub const ReadInputWrite: PerleLineAccessPort21 = PerleLineAccessPort21(3);
	pub const ReadOutput: PerleLineAccessPort21 = PerleLineAccessPort21(4);
	pub const ReadOutputWrite: PerleLineAccessPort21 = PerleLineAccessPort21(5);
	pub const ReadOutputInput: PerleLineAccessPort21 = PerleLineAccessPort21(6);
	pub const ReadOutputInputWrite: PerleLineAccessPort21 = PerleLineAccessPort21(7);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PerleLineAccessPort22(pub u32);
 
#[allow(non_upper_case_globals)]
impl PerleLineAccessPort22 {
	pub const Disabled: PerleLineAccessPort22 = PerleLineAccessPort22(0);
	pub const ReadWrite: PerleLineAccessPort22 = PerleLineAccessPort22(1);
	pub const ReadInput: PerleLineAccessPort22 = PerleLineAccessPort22(2);
	pub const ReadInputWrite: PerleLineAccessPort22 = PerleLineAccessPort22(3);
	pub const ReadOutput: PerleLineAccessPort22 = PerleLineAccessPort22(4);
	pub const ReadOutputWrite: PerleLineAccessPort22 = PerleLineAccessPort22(5);
	pub const ReadOutputInput: PerleLineAccessPort22 = PerleLineAccessPort22(6);
	pub const ReadOutputInputWrite: PerleLineAccessPort22 = PerleLineAccessPort22(7);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PerleLineAccessPort23(pub u32);
 
#[allow(non_upper_case_globals)]
impl PerleLineAccessPort23 {
	pub const Disabled: PerleLineAccessPort23 = PerleLineAccessPort23(0);
	pub const ReadWrite: PerleLineAccessPort23 = PerleLineAccessPort23(1);
	pub const ReadInput: PerleLineAccessPort23 = PerleLineAccessPort23(2);
	pub const ReadInputWrite: PerleLineAccessPort23 = PerleLineAccessPort23(3);
	pub const ReadOutput: PerleLineAccessPort23 = PerleLineAccessPort23(4);
	pub const ReadOutputWrite: PerleLineAccessPort23 = PerleLineAccessPort23(5);
	pub const ReadOutputInput: PerleLineAccessPort23 = PerleLineAccessPort23(6);
	pub const ReadOutputInputWrite: PerleLineAccessPort23 = PerleLineAccessPort23(7);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PerleLineAccessPort24(pub u32);
 
#[allow(non_upper_case_globals)]
impl PerleLineAccessPort24 {
	pub const Disabled: PerleLineAccessPort24 = PerleLineAccessPort24(0);
	pub const ReadWrite: PerleLineAccessPort24 = PerleLineAccessPort24(1);
	pub const ReadInput: PerleLineAccessPort24 = PerleLineAccessPort24(2);
	pub const ReadInputWrite: PerleLineAccessPort24 = PerleLineAccessPort24(3);
	pub const ReadOutput: PerleLineAccessPort24 = PerleLineAccessPort24(4);
	pub const ReadOutputWrite: PerleLineAccessPort24 = PerleLineAccessPort24(5);
	pub const ReadOutputInput: PerleLineAccessPort24 = PerleLineAccessPort24(6);
	pub const ReadOutputInputWrite: PerleLineAccessPort24 = PerleLineAccessPort24(7);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PerleLineAccessPort25(pub u32);
 
#[allow(non_upper_case_globals)]
impl PerleLineAccessPort25 {
	pub const Disabled: PerleLineAccessPort25 = PerleLineAccessPort25(0);
	pub const ReadWrite: PerleLineAccessPort25 = PerleLineAccessPort25(1);
	pub const ReadInput: PerleLineAccessPort25 = PerleLineAccessPort25(2);
	pub const ReadInputWrite: PerleLineAccessPort25 = PerleLineAccessPort25(3);
	pub const ReadOutput: PerleLineAccessPort25 = PerleLineAccessPort25(4);
	pub const ReadOutputWrite: PerleLineAccessPort25 = PerleLineAccessPort25(5);
	pub const ReadOutputInput: PerleLineAccessPort25 = PerleLineAccessPort25(6);
	pub const ReadOutputInputWrite: PerleLineAccessPort25 = PerleLineAccessPort25(7);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PerleLineAccessPort26(pub u32);
 
#[allow(non_upper_case_globals)]
impl PerleLineAccessPort26 {
	pub const Disabled: PerleLineAccessPort26 = PerleLineAccessPort26(0);
	pub const ReadWrite: PerleLineAccessPort26 = PerleLineAccessPort26(1);
	pub const ReadInput: PerleLineAccessPort26 = PerleLineAccessPort26(2);
	pub const ReadInputWrite: PerleLineAccessPort26 = PerleLineAccessPort26(3);
	pub const ReadOutput: PerleLineAccessPort26 = PerleLineAccessPort26(4);
	pub const ReadOutputWrite: PerleLineAccessPort26 = PerleLineAccessPort26(5);
	pub const ReadOutputInput: PerleLineAccessPort26 = PerleLineAccessPort26(6);
	pub const ReadOutputInputWrite: PerleLineAccessPort26 = PerleLineAccessPort26(7);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PerleLineAccessPort27(pub u32);
 
#[allow(non_upper_case_globals)]
impl PerleLineAccessPort27 {
	pub const Disabled: PerleLineAccessPort27 = PerleLineAccessPort27(0);
	pub const ReadWrite: PerleLineAccessPort27 = PerleLineAccessPort27(1);
	pub const ReadInput: PerleLineAccessPort27 = PerleLineAccessPort27(2);
	pub const ReadInputWrite: PerleLineAccessPort27 = PerleLineAccessPort27(3);
	pub const ReadOutput: PerleLineAccessPort27 = PerleLineAccessPort27(4);
	pub const ReadOutputWrite: PerleLineAccessPort27 = PerleLineAccessPort27(5);
	pub const ReadOutputInput: PerleLineAccessPort27 = PerleLineAccessPort27(6);
	pub const ReadOutputInputWrite: PerleLineAccessPort27 = PerleLineAccessPort27(7);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PerleLineAccessPort28(pub u32);
 
#[allow(non_upper_case_globals)]
impl PerleLineAccessPort28 {
	pub const Disabled: PerleLineAccessPort28 = PerleLineAccessPort28(0);
	pub const ReadWrite: PerleLineAccessPort28 = PerleLineAccessPort28(1);
	pub const ReadInput: PerleLineAccessPort28 = PerleLineAccessPort28(2);
	pub const ReadInputWrite: PerleLineAccessPort28 = PerleLineAccessPort28(3);
	pub const ReadOutput: PerleLineAccessPort28 = PerleLineAccessPort28(4);
	pub const ReadOutputWrite: PerleLineAccessPort28 = PerleLineAccessPort28(5);
	pub const ReadOutputInput: PerleLineAccessPort28 = PerleLineAccessPort28(6);
	pub const ReadOutputInputWrite: PerleLineAccessPort28 = PerleLineAccessPort28(7);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PerleLineAccessPort29(pub u32);
 
#[allow(non_upper_case_globals)]
impl PerleLineAccessPort29 {
	pub const Disabled: PerleLineAccessPort29 = PerleLineAccessPort29(0);
	pub const ReadWrite: PerleLineAccessPort29 = PerleLineAccessPort29(1);
	pub const ReadInput: PerleLineAccessPort29 = PerleLineAccessPort29(2);
	pub const ReadInputWrite: PerleLineAccessPort29 = PerleLineAccessPort29(3);
	pub const ReadOutput: PerleLineAccessPort29 = PerleLineAccessPort29(4);
	pub const ReadOutputWrite: PerleLineAccessPort29 = PerleLineAccessPort29(5);
	pub const ReadOutputInput: PerleLineAccessPort29 = PerleLineAccessPort29(6);
	pub const ReadOutputInputWrite: PerleLineAccessPort29 = PerleLineAccessPort29(7);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PerleLineAccessPort30(pub u32);
 
#[allow(non_upper_case_globals)]
impl PerleLineAccessPort30 {
	pub const Disabled: PerleLineAccessPort30 = PerleLineAccessPort30(0);
	pub const ReadWrite: PerleLineAccessPort30 = PerleLineAccessPort30(1);
	pub const ReadInput: PerleLineAccessPort30 = PerleLineAccessPort30(2);
	pub const ReadInputWrite: PerleLineAccessPort30 = PerleLineAccessPort30(3);
	pub const ReadOutput: PerleLineAccessPort30 = PerleLineAccessPort30(4);
	pub const ReadOutputWrite: PerleLineAccessPort30 = PerleLineAccessPort30(5);
	pub const ReadOutputInput: PerleLineAccessPort30 = PerleLineAccessPort30(6);
	pub const ReadOutputInputWrite: PerleLineAccessPort30 = PerleLineAccessPort30(7);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PerleLineAccessPort31(pub u32);
 
#[allow(non_upper_case_globals)]
impl PerleLineAccessPort31 {
	pub const Disabled: PerleLineAccessPort31 = PerleLineAccessPort31(0);
	pub const ReadWrite: PerleLineAccessPort31 = PerleLineAccessPort31(1);
	pub const ReadInput: PerleLineAccessPort31 = PerleLineAccessPort31(2);
	pub const ReadInputWrite: PerleLineAccessPort31 = PerleLineAccessPort31(3);
	pub const ReadOutput: PerleLineAccessPort31 = PerleLineAccessPort31(4);
	pub const ReadOutputWrite: PerleLineAccessPort31 = PerleLineAccessPort31(5);
	pub const ReadOutputInput: PerleLineAccessPort31 = PerleLineAccessPort31(6);
	pub const ReadOutputInputWrite: PerleLineAccessPort31 = PerleLineAccessPort31(7);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PerleLineAccessPort32(pub u32);
 
#[allow(non_upper_case_globals)]
impl PerleLineAccessPort32 {
	pub const Disabled: PerleLineAccessPort32 = PerleLineAccessPort32(0);
	pub const ReadWrite: PerleLineAccessPort32 = PerleLineAccessPort32(1);
	pub const ReadInput: PerleLineAccessPort32 = PerleLineAccessPort32(2);
	pub const ReadInputWrite: PerleLineAccessPort32 = PerleLineAccessPort32(3);
	pub const ReadOutput: PerleLineAccessPort32 = PerleLineAccessPort32(4);
	pub const ReadOutputWrite: PerleLineAccessPort32 = PerleLineAccessPort32(5);
	pub const ReadOutputInput: PerleLineAccessPort32 = PerleLineAccessPort32(6);
	pub const ReadOutputInputWrite: PerleLineAccessPort32 = PerleLineAccessPort32(7);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PerleLineAccessPort33(pub u32);
 
#[allow(non_upper_case_globals)]
impl PerleLineAccessPort33 {
	pub const Disabled: PerleLineAccessPort33 = PerleLineAccessPort33(0);
	pub const ReadWrite: PerleLineAccessPort33 = PerleLineAccessPort33(1);
	pub const ReadInput: PerleLineAccessPort33 = PerleLineAccessPort33(2);
	pub const ReadInputWrite: PerleLineAccessPort33 = PerleLineAccessPort33(3);
	pub const ReadOutput: PerleLineAccessPort33 = PerleLineAccessPort33(4);
	pub const ReadOutputWrite: PerleLineAccessPort33 = PerleLineAccessPort33(5);
	pub const ReadOutputInput: PerleLineAccessPort33 = PerleLineAccessPort33(6);
	pub const ReadOutputInputWrite: PerleLineAccessPort33 = PerleLineAccessPort33(7);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PerleLineAccessPort34(pub u32);
 
#[allow(non_upper_case_globals)]
impl PerleLineAccessPort34 {
	pub const Disabled: PerleLineAccessPort34 = PerleLineAccessPort34(0);
	pub const ReadWrite: PerleLineAccessPort34 = PerleLineAccessPort34(1);
	pub const ReadInput: PerleLineAccessPort34 = PerleLineAccessPort34(2);
	pub const ReadInputWrite: PerleLineAccessPort34 = PerleLineAccessPort34(3);
	pub const ReadOutput: PerleLineAccessPort34 = PerleLineAccessPort34(4);
	pub const ReadOutputWrite: PerleLineAccessPort34 = PerleLineAccessPort34(5);
	pub const ReadOutputInput: PerleLineAccessPort34 = PerleLineAccessPort34(6);
	pub const ReadOutputInputWrite: PerleLineAccessPort34 = PerleLineAccessPort34(7);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PerleLineAccessPort35(pub u32);
 
#[allow(non_upper_case_globals)]
impl PerleLineAccessPort35 {
	pub const Disabled: PerleLineAccessPort35 = PerleLineAccessPort35(0);
	pub const ReadWrite: PerleLineAccessPort35 = PerleLineAccessPort35(1);
	pub const ReadInput: PerleLineAccessPort35 = PerleLineAccessPort35(2);
	pub const ReadInputWrite: PerleLineAccessPort35 = PerleLineAccessPort35(3);
	pub const ReadOutput: PerleLineAccessPort35 = PerleLineAccessPort35(4);
	pub const ReadOutputWrite: PerleLineAccessPort35 = PerleLineAccessPort35(5);
	pub const ReadOutputInput: PerleLineAccessPort35 = PerleLineAccessPort35(6);
	pub const ReadOutputInputWrite: PerleLineAccessPort35 = PerleLineAccessPort35(7);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PerleLineAccessPort36(pub u32);
 
#[allow(non_upper_case_globals)]
impl PerleLineAccessPort36 {
	pub const Disabled: PerleLineAccessPort36 = PerleLineAccessPort36(0);
	pub const ReadWrite: PerleLineAccessPort36 = PerleLineAccessPort36(1);
	pub const ReadInput: PerleLineAccessPort36 = PerleLineAccessPort36(2);
	pub const ReadInputWrite: PerleLineAccessPort36 = PerleLineAccessPort36(3);
	pub const ReadOutput: PerleLineAccessPort36 = PerleLineAccessPort36(4);
	pub const ReadOutputWrite: PerleLineAccessPort36 = PerleLineAccessPort36(5);
	pub const ReadOutputInput: PerleLineAccessPort36 = PerleLineAccessPort36(6);
	pub const ReadOutputInputWrite: PerleLineAccessPort36 = PerleLineAccessPort36(7);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PerleLineAccessPort37(pub u32);
 
#[allow(non_upper_case_globals)]
impl PerleLineAccessPort37 {
	pub const Disabled: PerleLineAccessPort37 = PerleLineAccessPort37(0);
	pub const ReadWrite: PerleLineAccessPort37 = PerleLineAccessPort37(1);
	pub const ReadInput: PerleLineAccessPort37 = PerleLineAccessPort37(2);
	pub const ReadInputWrite: PerleLineAccessPort37 = PerleLineAccessPort37(3);
	pub const ReadOutput: PerleLineAccessPort37 = PerleLineAccessPort37(4);
	pub const ReadOutputWrite: PerleLineAccessPort37 = PerleLineAccessPort37(5);
	pub const ReadOutputInput: PerleLineAccessPort37 = PerleLineAccessPort37(6);
	pub const ReadOutputInputWrite: PerleLineAccessPort37 = PerleLineAccessPort37(7);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PerleLineAccessPort38(pub u32);
 
#[allow(non_upper_case_globals)]
impl PerleLineAccessPort38 {
	pub const Disabled: PerleLineAccessPort38 = PerleLineAccessPort38(0);
	pub const ReadWrite: PerleLineAccessPort38 = PerleLineAccessPort38(1);
	pub const ReadInput: PerleLineAccessPort38 = PerleLineAccessPort38(2);
	pub const ReadInputWrite: PerleLineAccessPort38 = PerleLineAccessPort38(3);
	pub const ReadOutput: PerleLineAccessPort38 = PerleLineAccessPort38(4);
	pub const ReadOutputWrite: PerleLineAccessPort38 = PerleLineAccessPort38(5);
	pub const ReadOutputInput: PerleLineAccessPort38 = PerleLineAccessPort38(6);
	pub const ReadOutputInputWrite: PerleLineAccessPort38 = PerleLineAccessPort38(7);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PerleLineAccessPort39(pub u32);
 
#[allow(non_upper_case_globals)]
impl PerleLineAccessPort39 {
	pub const Disabled: PerleLineAccessPort39 = PerleLineAccessPort39(0);
	pub const ReadWrite: PerleLineAccessPort39 = PerleLineAccessPort39(1);
	pub const ReadInput: PerleLineAccessPort39 = PerleLineAccessPort39(2);
	pub const ReadInputWrite: PerleLineAccessPort39 = PerleLineAccessPort39(3);
	pub const ReadOutput: PerleLineAccessPort39 = PerleLineAccessPort39(4);
	pub const ReadOutputWrite: PerleLineAccessPort39 = PerleLineAccessPort39(5);
	pub const ReadOutputInput: PerleLineAccessPort39 = PerleLineAccessPort39(6);
	pub const ReadOutputInputWrite: PerleLineAccessPort39 = PerleLineAccessPort39(7);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PerleLineAccessPort40(pub u32);
 
#[allow(non_upper_case_globals)]
impl PerleLineAccessPort40 {
	pub const Disabled: PerleLineAccessPort40 = PerleLineAccessPort40(0);
	pub const ReadWrite: PerleLineAccessPort40 = PerleLineAccessPort40(1);
	pub const ReadInput: PerleLineAccessPort40 = PerleLineAccessPort40(2);
	pub const ReadInputWrite: PerleLineAccessPort40 = PerleLineAccessPort40(3);
	pub const ReadOutput: PerleLineAccessPort40 = PerleLineAccessPort40(4);
	pub const ReadOutputWrite: PerleLineAccessPort40 = PerleLineAccessPort40(5);
	pub const ReadOutputInput: PerleLineAccessPort40 = PerleLineAccessPort40(6);
	pub const ReadOutputInputWrite: PerleLineAccessPort40 = PerleLineAccessPort40(7);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PerleLineAccessPort41(pub u32);
 
#[allow(non_upper_case_globals)]
impl PerleLineAccessPort41 {
	pub const Disabled: PerleLineAccessPort41 = PerleLineAccessPort41(0);
	pub const ReadWrite: PerleLineAccessPort41 = PerleLineAccessPort41(1);
	pub const ReadInput: PerleLineAccessPort41 = PerleLineAccessPort41(2);
	pub const ReadInputWrite: PerleLineAccessPort41 = PerleLineAccessPort41(3);
	pub const ReadOutput: PerleLineAccessPort41 = PerleLineAccessPort41(4);
	pub const ReadOutputWrite: PerleLineAccessPort41 = PerleLineAccessPort41(5);
	pub const ReadOutputInput: PerleLineAccessPort41 = PerleLineAccessPort41(6);
	pub const ReadOutputInputWrite: PerleLineAccessPort41 = PerleLineAccessPort41(7);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PerleLineAccessPort42(pub u32);
 
#[allow(non_upper_case_globals)]
impl PerleLineAccessPort42 {
	pub const Disabled: PerleLineAccessPort42 = PerleLineAccessPort42(0);
	pub const ReadWrite: PerleLineAccessPort42 = PerleLineAccessPort42(1);
	pub const ReadInput: PerleLineAccessPort42 = PerleLineAccessPort42(2);
	pub const ReadInputWrite: PerleLineAccessPort42 = PerleLineAccessPort42(3);
	pub const ReadOutput: PerleLineAccessPort42 = PerleLineAccessPort42(4);
	pub const ReadOutputWrite: PerleLineAccessPort42 = PerleLineAccessPort42(5);
	pub const ReadOutputInput: PerleLineAccessPort42 = PerleLineAccessPort42(6);
	pub const ReadOutputInputWrite: PerleLineAccessPort42 = PerleLineAccessPort42(7);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PerleLineAccessPort43(pub u32);
 
#[allow(non_upper_case_globals)]
impl PerleLineAccessPort43 {
	pub const Disabled: PerleLineAccessPort43 = PerleLineAccessPort43(0);
	pub const ReadWrite: PerleLineAccessPort43 = PerleLineAccessPort43(1);
	pub const ReadInput: PerleLineAccessPort43 = PerleLineAccessPort43(2);
	pub const ReadInputWrite: PerleLineAccessPort43 = PerleLineAccessPort43(3);
	pub const ReadOutput: PerleLineAccessPort43 = PerleLineAccessPort43(4);
	pub const ReadOutputWrite: PerleLineAccessPort43 = PerleLineAccessPort43(5);
	pub const ReadOutputInput: PerleLineAccessPort43 = PerleLineAccessPort43(6);
	pub const ReadOutputInputWrite: PerleLineAccessPort43 = PerleLineAccessPort43(7);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PerleLineAccessPort44(pub u32);
 
#[allow(non_upper_case_globals)]
impl PerleLineAccessPort44 {
	pub const Disabled: PerleLineAccessPort44 = PerleLineAccessPort44(0);
	pub const ReadWrite: PerleLineAccessPort44 = PerleLineAccessPort44(1);
	pub const ReadInput: PerleLineAccessPort44 = PerleLineAccessPort44(2);
	pub const ReadInputWrite: PerleLineAccessPort44 = PerleLineAccessPort44(3);
	pub const ReadOutput: PerleLineAccessPort44 = PerleLineAccessPort44(4);
	pub const ReadOutputWrite: PerleLineAccessPort44 = PerleLineAccessPort44(5);
	pub const ReadOutputInput: PerleLineAccessPort44 = PerleLineAccessPort44(6);
	pub const ReadOutputInputWrite: PerleLineAccessPort44 = PerleLineAccessPort44(7);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PerleLineAccessPort45(pub u32);
 
#[allow(non_upper_case_globals)]
impl PerleLineAccessPort45 {
	pub const Disabled: PerleLineAccessPort45 = PerleLineAccessPort45(0);
	pub const ReadWrite: PerleLineAccessPort45 = PerleLineAccessPort45(1);
	pub const ReadInput: PerleLineAccessPort45 = PerleLineAccessPort45(2);
	pub const ReadInputWrite: PerleLineAccessPort45 = PerleLineAccessPort45(3);
	pub const ReadOutput: PerleLineAccessPort45 = PerleLineAccessPort45(4);
	pub const ReadOutputWrite: PerleLineAccessPort45 = PerleLineAccessPort45(5);
	pub const ReadOutputInput: PerleLineAccessPort45 = PerleLineAccessPort45(6);
	pub const ReadOutputInputWrite: PerleLineAccessPort45 = PerleLineAccessPort45(7);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PerleLineAccessPort46(pub u32);
 
#[allow(non_upper_case_globals)]
impl PerleLineAccessPort46 {
	pub const Disabled: PerleLineAccessPort46 = PerleLineAccessPort46(0);
	pub const ReadWrite: PerleLineAccessPort46 = PerleLineAccessPort46(1);
	pub const ReadInput: PerleLineAccessPort46 = PerleLineAccessPort46(2);
	pub const ReadInputWrite: PerleLineAccessPort46 = PerleLineAccessPort46(3);
	pub const ReadOutput: PerleLineAccessPort46 = PerleLineAccessPort46(4);
	pub const ReadOutputWrite: PerleLineAccessPort46 = PerleLineAccessPort46(5);
	pub const ReadOutputInput: PerleLineAccessPort46 = PerleLineAccessPort46(6);
	pub const ReadOutputInputWrite: PerleLineAccessPort46 = PerleLineAccessPort46(7);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PerleLineAccessPort47(pub u32);
 
#[allow(non_upper_case_globals)]
impl PerleLineAccessPort47 {
	pub const Disabled: PerleLineAccessPort47 = PerleLineAccessPort47(0);
	pub const ReadWrite: PerleLineAccessPort47 = PerleLineAccessPort47(1);
	pub const ReadInput: PerleLineAccessPort47 = PerleLineAccessPort47(2);
	pub const ReadInputWrite: PerleLineAccessPort47 = PerleLineAccessPort47(3);
	pub const ReadOutput: PerleLineAccessPort47 = PerleLineAccessPort47(4);
	pub const ReadOutputWrite: PerleLineAccessPort47 = PerleLineAccessPort47(5);
	pub const ReadOutputInput: PerleLineAccessPort47 = PerleLineAccessPort47(6);
	pub const ReadOutputInputWrite: PerleLineAccessPort47 = PerleLineAccessPort47(7);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PerleLineAccessPort48(pub u32);
 
#[allow(non_upper_case_globals)]
impl PerleLineAccessPort48 {
	pub const Disabled: PerleLineAccessPort48 = PerleLineAccessPort48(0);
	pub const ReadWrite: PerleLineAccessPort48 = PerleLineAccessPort48(1);
	pub const ReadInput: PerleLineAccessPort48 = PerleLineAccessPort48(2);
	pub const ReadInputWrite: PerleLineAccessPort48 = PerleLineAccessPort48(3);
	pub const ReadOutput: PerleLineAccessPort48 = PerleLineAccessPort48(4);
	pub const ReadOutputWrite: PerleLineAccessPort48 = PerleLineAccessPort48(5);
	pub const ReadOutputInput: PerleLineAccessPort48 = PerleLineAccessPort48(6);
	pub const ReadOutputInputWrite: PerleLineAccessPort48 = PerleLineAccessPort48(7);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PerleLineAccessPort49(pub u32);
 
#[allow(non_upper_case_globals)]
impl PerleLineAccessPort49 {
	pub const Disabled: PerleLineAccessPort49 = PerleLineAccessPort49(0);
	pub const ReadWrite: PerleLineAccessPort49 = PerleLineAccessPort49(1);
	pub const ReadInput: PerleLineAccessPort49 = PerleLineAccessPort49(2);
	pub const ReadInputWrite: PerleLineAccessPort49 = PerleLineAccessPort49(3);
	pub const ReadOutput: PerleLineAccessPort49 = PerleLineAccessPort49(4);
	pub const ReadOutputWrite: PerleLineAccessPort49 = PerleLineAccessPort49(5);
	pub const ReadOutputInput: PerleLineAccessPort49 = PerleLineAccessPort49(6);
	pub const ReadOutputInputWrite: PerleLineAccessPort49 = PerleLineAccessPort49(7);
}


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		99 => map! {i, be_u32, |v| Attribute::VsaPerleClusteredPortAccess(PerleClusteredPortAccess(v))},
		100 => map! {i, be_u32, |v| Attribute::VsaPerleUserLevel(PerleUserLevel(v))},
		101 => map! {i, be_u32, |v| Attribute::VsaPerleLineAccessPort1(PerleLineAccessPort1(v))},
		102 => map! {i, be_u32, |v| Attribute::VsaPerleLineAccessPort2(PerleLineAccessPort2(v))},
		103 => map! {i, be_u32, |v| Attribute::VsaPerleLineAccessPort3(PerleLineAccessPort3(v))},
		104 => map! {i, be_u32, |v| Attribute::VsaPerleLineAccessPort4(PerleLineAccessPort4(v))},
		105 => map! {i, be_u32, |v| Attribute::VsaPerleLineAccessPort5(PerleLineAccessPort5(v))},
		106 => map! {i, be_u32, |v| Attribute::VsaPerleLineAccessPort6(PerleLineAccessPort6(v))},
		107 => map! {i, be_u32, |v| Attribute::VsaPerleLineAccessPort7(PerleLineAccessPort7(v))},
		108 => map! {i, be_u32, |v| Attribute::VsaPerleLineAccessPort8(PerleLineAccessPort8(v))},
		109 => map! {i, be_u32, |v| Attribute::VsaPerleLineAccessPort9(PerleLineAccessPort9(v))},
		110 => map! {i, be_u32, |v| Attribute::VsaPerleLineAccessPort10(PerleLineAccessPort10(v))},
		111 => map! {i, be_u32, |v| Attribute::VsaPerleLineAccessPort11(PerleLineAccessPort11(v))},
		112 => map! {i, be_u32, |v| Attribute::VsaPerleLineAccessPort12(PerleLineAccessPort12(v))},
		113 => map! {i, be_u32, |v| Attribute::VsaPerleLineAccessPort13(PerleLineAccessPort13(v))},
		114 => map! {i, be_u32, |v| Attribute::VsaPerleLineAccessPort14(PerleLineAccessPort14(v))},
		115 => map! {i, be_u32, |v| Attribute::VsaPerleLineAccessPort15(PerleLineAccessPort15(v))},
		116 => map! {i, be_u32, |v| Attribute::VsaPerleLineAccessPort16(PerleLineAccessPort16(v))},
		117 => map! {i, be_u32, |v| Attribute::VsaPerleLineAccessPort17(PerleLineAccessPort17(v))},
		118 => map! {i, be_u32, |v| Attribute::VsaPerleLineAccessPort18(PerleLineAccessPort18(v))},
		119 => map! {i, be_u32, |v| Attribute::VsaPerleLineAccessPort19(PerleLineAccessPort19(v))},
		120 => map! {i, be_u32, |v| Attribute::VsaPerleLineAccessPort20(PerleLineAccessPort20(v))},
		121 => map! {i, be_u32, |v| Attribute::VsaPerleLineAccessPort21(PerleLineAccessPort21(v))},
		122 => map! {i, be_u32, |v| Attribute::VsaPerleLineAccessPort22(PerleLineAccessPort22(v))},
		123 => map! {i, be_u32, |v| Attribute::VsaPerleLineAccessPort23(PerleLineAccessPort23(v))},
		124 => map! {i, be_u32, |v| Attribute::VsaPerleLineAccessPort24(PerleLineAccessPort24(v))},
		125 => map! {i, be_u32, |v| Attribute::VsaPerleLineAccessPort25(PerleLineAccessPort25(v))},
		126 => map! {i, be_u32, |v| Attribute::VsaPerleLineAccessPort26(PerleLineAccessPort26(v))},
		127 => map! {i, be_u32, |v| Attribute::VsaPerleLineAccessPort27(PerleLineAccessPort27(v))},
		128 => map! {i, be_u32, |v| Attribute::VsaPerleLineAccessPort28(PerleLineAccessPort28(v))},
		129 => map! {i, be_u32, |v| Attribute::VsaPerleLineAccessPort29(PerleLineAccessPort29(v))},
		130 => map! {i, be_u32, |v| Attribute::VsaPerleLineAccessPort30(PerleLineAccessPort30(v))},
		131 => map! {i, be_u32, |v| Attribute::VsaPerleLineAccessPort31(PerleLineAccessPort31(v))},
		132 => map! {i, be_u32, |v| Attribute::VsaPerleLineAccessPort32(PerleLineAccessPort32(v))},
		133 => map! {i, be_u32, |v| Attribute::VsaPerleLineAccessPort33(PerleLineAccessPort33(v))},
		134 => map! {i, be_u32, |v| Attribute::VsaPerleLineAccessPort34(PerleLineAccessPort34(v))},
		135 => map! {i, be_u32, |v| Attribute::VsaPerleLineAccessPort35(PerleLineAccessPort35(v))},
		136 => map! {i, be_u32, |v| Attribute::VsaPerleLineAccessPort36(PerleLineAccessPort36(v))},
		137 => map! {i, be_u32, |v| Attribute::VsaPerleLineAccessPort37(PerleLineAccessPort37(v))},
		138 => map! {i, be_u32, |v| Attribute::VsaPerleLineAccessPort38(PerleLineAccessPort38(v))},
		139 => map! {i, be_u32, |v| Attribute::VsaPerleLineAccessPort39(PerleLineAccessPort39(v))},
		140 => map! {i, be_u32, |v| Attribute::VsaPerleLineAccessPort40(PerleLineAccessPort40(v))},
		141 => map! {i, be_u32, |v| Attribute::VsaPerleLineAccessPort41(PerleLineAccessPort41(v))},
		142 => map! {i, be_u32, |v| Attribute::VsaPerleLineAccessPort42(PerleLineAccessPort42(v))},
		143 => map! {i, be_u32, |v| Attribute::VsaPerleLineAccessPort43(PerleLineAccessPort43(v))},
		144 => map! {i, be_u32, |v| Attribute::VsaPerleLineAccessPort44(PerleLineAccessPort44(v))},
		145 => map! {i, be_u32, |v| Attribute::VsaPerleLineAccessPort45(PerleLineAccessPort45(v))},
		146 => map! {i, be_u32, |v| Attribute::VsaPerleLineAccessPort46(PerleLineAccessPort46(v))},
		147 => map! {i, be_u32, |v| Attribute::VsaPerleLineAccessPort47(PerleLineAccessPort47(v))},
		148 => map! {i, be_u32, |v| Attribute::VsaPerleLineAccessPort48(PerleLineAccessPort48(v))},
		149 => map! {i, be_u32, |v| Attribute::VsaPerleLineAccessPort49(PerleLineAccessPort49(v))},
        _ => value!(i, Attribute::VsaUnknown(1966, typ, i)),
    }
}

use bits_macro::Bits;
use std::fmt::Display;

pub trait Bits {
    fn to_u64(&self) -> u64;
    fn size(&self) -> u8;
}

#[derive(Bits)]
pub struct B1(#[size(1)] pub u64);

#[derive(Bits)]
pub struct B2(#[size(2)] pub u64);

#[derive(Bits)]
pub struct B3(#[size(3)] pub u64);

#[derive(Bits)]
pub struct B4(#[size(4)] pub u64);

#[derive(Bits)]
pub struct B5(#[size(5)] pub u64);

#[derive(Bits)]
pub struct B6(#[size(6)] pub u64);

#[derive(Bits)]
pub struct B7(#[size(7)] pub u64);

#[derive(Bits)]
pub struct B8(#[size(8)] pub u64);

#[derive(Bits)]
pub struct B9(#[size(9)] pub u64);

#[derive(Bits)]
pub struct B10(#[size(10)] pub u64);

#[derive(Bits)]
pub struct B11(#[size(11)] pub u64);

#[derive(Bits)]
pub struct B12(#[size(12)] pub u64);

#[derive(Bits)]
pub struct B13(#[size(13)] pub u64);

#[derive(Bits)]
pub struct B14(#[size(14)] pub u64);

#[derive(Bits)]
pub struct B15(#[size(15)] pub u64);

#[derive(Bits)]
pub struct B16(#[size(16)] pub u64);

#[derive(Bits)]
pub struct B17(#[size(17)] pub u64);

#[derive(Bits)]
pub struct B18(#[size(18)] pub u64);

#[derive(Bits)]
pub struct B19(#[size(19)] pub u64);

#[derive(Bits)]
pub struct B20(#[size(20)] pub u64);

#[derive(Bits)]
pub struct B21(#[size(21)] pub u64);

#[derive(Bits)]
pub struct B22(#[size(22)] pub u64);

#[derive(Bits)]
pub struct B23(#[size(23)] pub u64);

#[derive(Bits)]
pub struct B24(#[size(24)] pub u64);

#[derive(Bits)]
pub struct B25(#[size(25)] pub u64);

#[derive(Bits)]
pub struct B26(#[size(26)] pub u64);

#[derive(Bits)]
pub struct B27(#[size(27)] pub u64);

#[derive(Bits)]
pub struct B28(#[size(28)] pub u64);

#[derive(Bits)]
pub struct B29(#[size(29)] pub u64);

#[derive(Bits)]
pub struct B30(#[size(30)] pub u64);

#[derive(Bits)]
pub struct B31(#[size(31)] pub u64);

#[derive(Bits)]
pub struct B32(#[size(32)] pub u64);

#[derive(Bits)]
pub struct B33(#[size(33)] pub u64);

#[derive(Bits)]
pub struct B34(#[size(34)] pub u64);

#[derive(Bits)]
pub struct B35(#[size(35)] pub u64);

#[derive(Bits)]
pub struct B36(#[size(36)] pub u64);

#[derive(Bits)]
pub struct B37(#[size(37)] pub u64);

#[derive(Bits)]
pub struct B38(#[size(38)] pub u64);

#[derive(Bits)]
pub struct B39(#[size(39)] pub u64);

#[derive(Bits)]
pub struct B40(#[size(40)] pub u64);

#[derive(Bits)]
pub struct B41(#[size(41)] pub u64);

#[derive(Bits)]
pub struct B42(#[size(42)] pub u64);

#[derive(Bits)]
pub struct B43(#[size(43)] pub u64);

#[derive(Bits)]
pub struct B44(#[size(44)] pub u64);

#[derive(Bits)]
pub struct B45(#[size(45)] pub u64);

#[derive(Bits)]
pub struct B46(#[size(46)] pub u64);

#[derive(Bits)]
pub struct B47(#[size(47)] pub u64);

#[derive(Bits)]
pub struct B48(#[size(48)] pub u64);

#[derive(Bits)]
pub struct B49(#[size(49)] pub u64);

#[derive(Bits)]
pub struct B50(#[size(50)] pub u64);

#[derive(Bits)]
pub struct B51(#[size(51)] pub u64);

#[derive(Bits)]
pub struct B52(#[size(52)] pub u64);

#[derive(Bits)]
pub struct B53(#[size(53)] pub u64);

#[derive(Bits)]
pub struct B54(#[size(54)] pub u64);

#[derive(Bits)]
pub struct B55(#[size(55)] pub u64);

#[derive(Bits)]
pub struct B56(#[size(56)] pub u64);

#[derive(Bits)]
pub struct B57(#[size(57)] pub u64);

#[derive(Bits)]
pub struct B58(#[size(58)] pub u64);

#[derive(Bits)]
pub struct B59(#[size(59)] pub u64);

#[derive(Bits)]
pub struct B60(#[size(60)] pub u64);

#[derive(Bits)]
pub struct B61(#[size(61)] pub u64);

#[derive(Bits)]
pub struct B62(#[size(62)] pub u64);

#[derive(Bits)]
pub struct B63(#[size(63)] pub u64);

#[derive(Bits)]
pub struct B64(#[size(64)] pub u64);
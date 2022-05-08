#[repr(align(   1))] pub struct    A1<T> {pub data: T}
#[repr(align(   2))] pub struct    A2<T> {pub data: T}
#[repr(align(   4))] pub struct    A4<T> {pub data: T}
#[repr(align(   8))] pub struct    A8<T> {pub data: T}
#[repr(align(  16))] pub struct   A16<T> {pub data: T}
#[repr(align(  32))] pub struct   A32<T> {pub data: T}
#[repr(align(  64))] pub struct   A64<T> {pub data: T}
#[repr(align( 128))] pub struct  A128<T> {pub data: T}
#[repr(align( 256))] pub struct  A256<T> {pub data: T}
#[repr(align( 512))] pub struct  A512<T> {pub data: T}
#[repr(align(1024))] pub struct A1024<T> {pub data: T}
#[repr(align(2048))] pub struct A2048<T> {pub data: T}
#[repr(align(4096))] pub struct A4096<T> {pub data: T}

#[repr(align(8192))] pub struct A8192<T> {pub data: T}

impl<T>    A1<T> {pub const fn new(data: T) -> Self {   A1::<T>{data}}}
impl<T>    A2<T> {pub const fn new(data: T) -> Self {   A2::<T>{data}}}
impl<T>    A4<T> {pub const fn new(data: T) -> Self {   A4::<T>{data}}}
impl<T>    A8<T> {pub const fn new(data: T) -> Self {   A8::<T>{data}}}
impl<T>   A16<T> {pub const fn new(data: T) -> Self {  A16::<T>{data}}}
impl<T>   A32<T> {pub const fn new(data: T) -> Self {  A32::<T>{data}}}
impl<T>   A64<T> {pub const fn new(data: T) -> Self {  A64::<T>{data}}}
impl<T>  A128<T> {pub const fn new(data: T) -> Self { A128::<T>{data}}}
impl<T>  A256<T> {pub const fn new(data: T) -> Self { A256::<T>{data}}}
impl<T>  A512<T> {pub const fn new(data: T) -> Self { A512::<T>{data}}}
impl<T> A1024<T> {pub const fn new(data: T) -> Self {A1024::<T>{data}}}
impl<T> A2048<T> {pub const fn new(data: T) -> Self {A2048::<T>{data}}}
impl<T> A4096<T> {pub const fn new(data: T) -> Self {A4096::<T>{data}}}
impl<T> A8192<T> {pub const fn new(data: T) -> Self {A8192::<T>{data}}}

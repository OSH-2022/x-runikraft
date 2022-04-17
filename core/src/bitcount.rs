//二进制表示中的1的个数
//原作者: John Baldwin <jhb@FreeBSD.org>
pub trait BitCount{
    fn bitcount(self)->Self;
}

impl BitCount for u16{
    fn bitcount(self)->u16{
        let mut x = self;
        x = (x & 0x5555_u16) + ((x & 0xaaaa_u16) >> 1);
	    x = (x & 0x3333_u16) + ((x & 0xcccc_u16) >> 2);
	    x = (x + (x >> 4)) & 0x0f0f_u16;
	    x = (x + (x >> 8)) & 0x00ff_u16;
	    x
    }
}

impl BitCount for u32{
    fn bitcount(self)->u32{
        let mut x = self;
        x = (x & 0x55555555_u32) + ((x & 0xaaaaaaaa_u32) >> 1);
        x = (x & 0x33333333_u32) + ((x & 0xcccccccc_u32) >> 2);
        x = (x + (x >> 4)) & 0x0f0f0f0f_u32;
        x =  x + (x >> 8);
        x = (x + (x >> 16)) & 0x000000ff_u32;
        x
    }
}

impl BitCount for u64{
    fn bitcount(self)->u64{
        let mut x = self;
        x = (x & 0x5555555555555555_u64) + ((x & 0xaaaaaaaaaaaaaaaa_u64) >> 1);
        x = (x & 0x3333333333333333_u64) + ((x & 0xcccccccccccccccc_u64) >> 2);
        x = (x + (x >> 4)) & 0x0f0f0f0f0f0f0f0f_u64;
        x =  x + (x >> 8);
        x =  x + (x >> 16);
        x = (x + (x >> 32)) & 0x000000ff_u64;
        x
    }
}

impl BitCount for u128{
    fn bitcount(self)->u128{
        (((self>>64) as u64).bitcount() + ((self & 0xffffffffffffffffu128) as u64).bitcount()) as u128
    }
} 

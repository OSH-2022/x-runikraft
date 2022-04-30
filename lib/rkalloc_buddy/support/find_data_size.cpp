#include <fmt/core.h>
#include <cassert>

constexpr size_t log2(size_t x)
{
    size_t y=0;
    if(x>=4294967296) y+=32,x>>=32;
    if(x>=65536) y+=16,x>>=16;
    if(x>=256) y+=8,x>>=8;
    if(x>=16) y+=4,x>>=4;
    if(x>=4) y+=2,x>>=2;
    if(x>=2) ++y;
    return y;
}

size_t get_max_index(size_t n_blocks)
{
    auto log2n = log2(n_blocks);
    if(n_blocks!=1<<log2n)log2n++;
    return (1<<log2n)-2+n_blocks;
}

int main()
{
    size_t n_meta_last = 0;
    size_t n_total_last = 0;
    for(size_t i=1;i<=65536;++i)
    {
        auto max_index = get_max_index(i);
        auto n_meta = (max_index+1+127)/128;
        if(n_meta>n_meta_last)
        {
            assert(n_meta<=(n_total_last+1+42)/43);
            assert(n_meta>=(n_total_last+1)/65);
            fmt::print("{:>5} {:>5} {:>5.2f}\n",n_meta,n_total_last+1,static_cast<double>(n_total_last+1)/n_meta);
        }
        n_meta_last=n_meta;
        n_total_last=n_meta+i;
        //fmt::print("{:>5} {:>5}({:>5}) {:>5}\n",i,n_meta,max_index,i+n_meta);
    }
    return 0;
}


static mut ID_COUNTER: usize = 0;

pub fn new() -> usize
{
    unsafe
    {
        ID_COUNTER += 1;
        return ID_COUNTER;
    }
}
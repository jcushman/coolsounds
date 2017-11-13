use std::time::Duration;
use rodio::Sample;
use rodio::Source;


/// Modify each sample by a given function:
#[derive(Clone, Debug)]
pub struct Map<I>
    where I: Source,
          I::Item: Sample
{
    input: I,
    func: fn(I::Item, usize)->I::Item,
    num_sample: usize,
}

impl<I> Map<I>
    where I: Source,
          I::Item: Sample
{
    #[inline]
    pub fn new(input: I, func: fn(I::Item, usize)->I::Item) -> Map<I> {
        Map {
            input,
            func,
            num_sample: 0
        }
    }
}

impl<I> Iterator for Map<I>
    where I: Source,
          I::Item: Sample
{
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<I::Item> {
        self.num_sample = self.num_sample.wrapping_add(1);
        self.input.next().map(|value| (self.func)(value, self.num_sample))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.input.size_hint()
    }
}

impl<I> ExactSizeIterator for Map<I>
    where I: Source + ExactSizeIterator,
          I::Item: Sample
{
}

impl<I> Source for Map<I>
    where I: Source,
          I::Item: Sample
{
    #[inline]
    fn current_frame_len(&self) -> Option<usize> {
        self.input.current_frame_len()
    }

    #[inline]
    fn channels(&self) -> u16 {
        self.input.channels()
    }

    #[inline]
    fn samples_rate(&self) -> u32 {
        self.input.samples_rate()
    }

    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        self.input.total_duration()
    }
}

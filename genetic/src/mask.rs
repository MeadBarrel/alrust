type Mask = Vec<bool>;

pub struct ApplyMask<'a, S> {
    iterable: Box<dyn Iterator<Item = &'a S> + 'a>,
    mask: Box<dyn Iterator<Item = &'a bool> + 'a>,
}

pub fn invert_mask(mask: &mut Mask) {
    mask.iter_mut().for_each(|x| *x = !*x);
    // for i in 0..mask.len() {
    //     mask[i] = !mask[i]
    // }
}

pub fn apply_mask<'a, I, S>(iterable: I, mask: &'a Mask) -> ApplyMask<'a, S>
where
    I: IntoIterator<Item = &'a S> + 'a,
{
    ApplyMask {
        iterable: Box::new(iterable.into_iter()),
        mask: Box::new(mask.iter()),
    }
}

pub fn mut_mask<I, S>(iterable: I, mask: &mut Mask, func: impl Fn(S) -> bool)
where
    I: IntoIterator<Item = S>,
{
    for (i, v) in iterable.into_iter().enumerate() {
        if !mask[i] {
            continue;
        }
        mask[i] = func(v);
    }
}

impl<'a, S> Iterator for ApplyMask<'a, S> {
    type Item = &'a S;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let bv = true;
            let flag = self.mask.next().unwrap_or(&bv);
            let value = self.iterable.next();

            if !flag {
                continue;
            }

            return value;
        }
    }
}

pub fn indices_from_mask(mask: &[bool]) -> Vec<usize> {
    let mut result = Vec::new();

    for (i, &item) in mask.iter().enumerate() {
        if item {
            result.push(i)
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mut_mask() {
        let mut mask = vec![false, false, true, false, true, true, false];
        let masked = vec![16, 10, 1, 3, 51, 9, 5];
        let expected = vec![false, false, false, false, true, false, false];

        mut_mask(masked, &mut mask, |x| x > 9);

        assert_eq!(mask, expected);
    }

    #[test]
    fn test_apply_mask() {
        let mask = vec![false, false, true, false, true, true, false];
        let masked = vec![[0, 1], [1, 2], [3, 4], [5, 6], [7, 8], [9, 10], [11, 12]];

        let masked_ref: Vec<&[i32; 2]> = masked.iter().collect();

        let actual = apply_mask(masked_ref, &mask);
        let expected = vec![&masked[2], &masked[4], &masked[5]];

        let collected: Vec<&[i32; 2]> = actual.collect();
        assert_eq!(collected, expected);
    }

    #[test]
    fn test_indices_from_mask() {
        let mask = vec![false, false, true, false, true, true, false];
        let expected = vec![2, 4, 5];
        let actual = indices_from_mask(&mask);

        assert_eq!(expected, actual, "Indices don't match")
    }
}

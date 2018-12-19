use std::fmt;

/// A box address from a mythical warehouse which may or may not contain items of importance to
/// a certain festive altruist.
pub struct Address<'a> {
    raw: &'a str,
}

impl<'a> Address<'a> {
    pub fn new(input: &'a str) -> Address<'a> {
        Address { raw: input }
    }

    /// Compare with the provided address by masking each character in turn to confirm that there
    /// is only 1 character difference between the two addresses. If the address matches in this
    /// way, the tuple returned will indicate the comparison is true and the col the differing
    /// character is in.
    pub fn compare(&self, other: &Address) -> (bool, usize) {
        let mut matched = false;
        let mut col: usize = 0;

        for p in 0..other.raw.len() {
            let our_parts = self.raw.split_at(p);
            let our_mask: String = [our_parts.0, "?", &our_parts.1[1..our_parts.1.len()]].join("");

            let other_parts = other.raw.split_at(p);
            let other_mask: String =
                [other_parts.0, "?", &other_parts.1[1..other_parts.1.len()]].join("");

            if our_mask.eq(&other_mask) {
                matched = true;
                col = p;
                break;
            }
        }

        return (matched, col);
    }

    /// Once a comparison indicates which column a match differs on, this will return the characters
    /// except the indicated one which will be dropped.
    pub fn drop_col(&self, col: usize) -> String {
        let our_parts = self.raw.split_at(col);
        let our_result: String = [our_parts.0, &our_parts.1[1..our_parts.1.len()]].join("");
        return our_result;
    }
}

impl<'a> fmt::Display for Address<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.raw)
    }
}

#[test]
fn test_compare() {
    let addr1 = Address::new("abcdefg");
    let addr2 = Address::new("abcrefg");
    assert_eq!(addr1.compare(&addr2), (true, 3));
}

/// Mythical warehouse operated by the large, red furry suit wearing, festive altruist. There may or
/// may not be magical items in here.
pub struct Warehouse<'a> {
    /// List of addresses within this warehouse. These are read from the input lines.
    addresses: Vec<Address<'a>>,
}

impl<'a> Warehouse<'a> {
    /// Create a new warehouse catering for addresses with the provided number of cols.
    pub fn new(input: &'a str) -> Warehouse<'a> {
        let mut addresses = vec![];

        for line in input.lines() {
            let mut addr: Address = Address::new(line);

            addresses.push(addr);
        }

        Warehouse { addresses }
    }

    /// Work through the list of addresses in the warehouse and compare each one to the rest of the
    /// addresses in the list. Return the matching characters that meet the matching criteria of
    /// puzzle.
    pub fn look_for_best_matches(&self) -> String {
        // track the position of the left address to ignore as a right address.
        let mut left_pos = 0;

        for a_left in self.addresses.iter() {
            // track the right position for the comparison to the left (outer) position.
            let mut right_pos = 0;
            for a_right in self.addresses.iter() {
                // ignore the address from the same position as this will definitely be a match but
                // not the one we are looking for!
                if left_pos == right_pos {
                    continue;
                }
                let (matched, col) = a_left.compare(&a_right);
                if matched {
                    return a_right.drop_col(col);
                }
                right_pos += 1;
            }
            left_pos += 1;
        }
        return "".to_string();
    }
}

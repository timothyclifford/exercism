#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    if first_list.eq(second_list) {
        return Comparison::Equal;
    }

    return if first_list.len() > second_list.len() {
        if is_sublist(second_list, first_list) {
            Comparison::Superlist
        } else {
            Comparison::Unequal
        }
    } else {
        if is_sublist(first_list, second_list) {
            Comparison::Sublist
        } else {
            Comparison::Unequal
        }
    };
}

fn is_sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> bool {
    let mut possible_matches = Vec::new();
    return match first_list.first() {
        Some(first_value) => {
            second_list.iter().enumerate().for_each(|(i, x)| {
                let slice_end = i + first_list.len();
                let list_end = second_list.len();
                if x.eq(first_value) && slice_end <= list_end {
                    let slice = &second_list[i..slice_end];
                    possible_matches.push(slice)
                }
            });
            return possible_matches.contains(&first_list);
        }
        None => true,
    };
}

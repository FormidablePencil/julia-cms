pub enum Association {
    Related,
    Reference,
}

pub enum AssociationWay {
    BothWays,
    LeftToRight,
    RightToLeft,
}

pub fn associate_compositions_to_spaces() {}

// not author space
pub fn associate_spaces(
    space_address: u128,
    associate_to_space_address: u128,
    association: Association,
    association_way: AssociationWay,
) {
    todo!()
}

use super::Metadata;

pub fn meta_index_increment(meta: &mut impl Metadata) {
    let index = meta.get_index();
    meta.set_index(index + 1);
}
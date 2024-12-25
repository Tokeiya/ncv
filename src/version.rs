use super::version_element::Element;

pub struct Version {
    major: u64,
    minor: Element,
    revision: Element,
}
